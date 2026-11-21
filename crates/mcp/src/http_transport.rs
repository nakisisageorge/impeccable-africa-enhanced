//! HTTP, SSE, and Streamable HTTP transports for the Impeccable MCP server.

use crate::{Upstream, Failure, WAITING, NOT_APPROVED, NO_BROWSER, SIGN_IN_WAIT};
use axum::{
    body::Body,
    extract::{State, WebSocketUpgrade, Query},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::{stream, Stream, StreamExt};
use impeccable_account::{config::Account, http::ApiError, session::{Session, SessionError}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;
use tracing::{debug, error, info, warn};

const PROTOCOL: &str = "2025-06-18";

/// Shared state for HTTP transports
#[derive(Clone)]
pub struct HttpTransportState {
    upstream: Arc<Mutex<Box<dyn Upstream + Send>>>,
    sessions: Arc<Mutex<HashMap<String, SessionState>>>,
    sse_tx: broadcast::Sender<String>,
}

#[derive(Debug, Clone)]
struct SessionState {
    agent: Option<String>,
    presence_id: Option<String>,
    initialized: bool,
}

impl HttpTransportState {
    pub fn new(upstream: Box<dyn Upstream + Send>) -> Self {
        let (sse_tx, _) = broadcast::channel(100);
        Self {
            upstream: Arc::new(Mutex::new(upstream)),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            sse_tx,
        }
    }
}

/// Create the HTTP transport router
pub fn create_http_router(state: HttpTransportState) -> Router {
    Router::new()
        .route("/mcp", post(handle_mcp_post))
        .route("/mcp/sse", get(handle_sse))
        .route("/mcp/stream", get(handle_streamable_http))
        .route("/mcp/ws", get(handle_websocket))
        .with_state(state)
}

/// Handle MCP JSON-RPC over HTTP POST
async fn handle_mcp_post(
    State(state): State<HttpTransportState>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> Response {
    let session_id = headers
        .get("mcp-session-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let mut session_guard = state.sessions.lock().await;
    let session = session_guard.entry(session_id.clone()).or_insert_with(|| SessionState {
        agent: None,
        presence_id: None,
        initialized: false,
    });

    // Handle initialize
    if request.get("method").and_then(Value::as_str) == Some("initialize") {
        session.initialized = true;
        session.agent = request
            .pointer("/params/clientInfo/name")
            .and_then(Value::as_str)
            .map(|s| s.to_string());
    }

    drop(session_guard);

    // Process the request through upstream
    let mut upstream = state.upstream.lock().await;
    match upstream.call(&request) {
        Ok(reply) => {
            let mut response = Json(reply);
            // Add session ID header
            let mut resp = response.into_response();
            resp.headers_mut().insert(
                "mcp-session-id",
                session_id.parse().unwrap(),
            );
            resp
        }
        Err(Failure::Auth) => {
            if !upstream.browser() {
                (StatusCode::UNAUTHORIZED, Json(json!({
                    "jsonrpc": "2.0",
                    "id": request.get("id").cloned().unwrap_or(Value::Null),
                    "error": { "code": -32001, "message": NO_BROWSER }
                }))).into_response()
            } else {
                // Send progress notification
                let _ = state.sse_tx.send(json!({
                    "jsonrpc": "2.0",
                    "method": "notifications/progress",
                    "params": { "progressToken": request.pointer("/params/_meta/progressToken"), "progress": 0, "message": WAITING }
                }).to_string());

                match upstream.sign_in() {
                    Ok(_) => {
                        match upstream.call(&request) {
                            Ok(reply) => Json(reply).into_response(),
                            Err(Failure::Auth) => (StatusCode::UNAUTHORIZED, Json(json!({
                                "jsonrpc": "2.0",
                                "id": request.get("id").cloned().unwrap_or(Value::Null),
                                "error": { "code": -32001, "message": NOT_APPROVED }
                            }))).into_response(),
                            Err(Failure::Other(why)) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "jsonrpc": "2.0",
                                "id": request.get("id").cloned().unwrap_or(Value::Null),
                                "error": { "code": -32603, "message": why }
                            }))).into_response(),
                        }
                    }
                    Err(line) => (StatusCode::UNAUTHORIZED, Json(json!({
                        "jsonrpc": "2.0",
                        "id": request.get("id").cloned().unwrap_or(Value::Null),
                        "error": { "code": -32001, "message": line }
                    }))).into_response(),
                }
            }
        }
        Err(Failure::Other(why)) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "jsonrpc": "2.0",
                "id": request.get("id").cloned().unwrap_or(Value::Null),
                "error": { "code": -32603, "message": why }
            }))).into_response()
        }
    }
}

/// Handle Server-Sent Events for MCP
async fn handle_sse(
    State(state): State<HttpTransportState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let session_id = params.get("session_id").cloned().unwrap_or_else(|| "default".to_string());
    
    let rx = state.sse_tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|msg| async move {
            msg.ok().map(|data| {
                Ok::<axum::response::sse::Event, std::convert::Infallible>(axum::response::sse::Event::default().data(data))
            })
        });

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(30)))
        .into_response()
}

/// Handle Streamable HTTP for MCP (bidirectional streaming)
async fn handle_streamable_http(
    State(state): State<HttpTransportState>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    let _session_id = headers
        .get("mcp-session-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default")
        .to_string();

    // Parse the request body as JSON-RPC
    let request: Value = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "jsonrpc": "2.0",
                "id": Value::Null,
                "error": { "code": -32700, "message": "Parse error" }
            }))).into_response();
        }
    };

    // Process similar to POST but with streaming response
    handle_mcp_post(State(state), headers, Json(request)).await
}

/// Handle WebSocket for MCP
async fn handle_websocket(
    State(state): State<HttpTransportState>,
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let session_id = params.get("session_id").cloned().unwrap_or_else(|| "default".to_string());
    
    ws.on_upgrade(move |socket| handle_websocket_connection(socket, state, session_id))
}

async fn handle_websocket_connection(
    mut socket: axum::extract::ws::WebSocket,
    state: HttpTransportState,
    session_id: String,
) {
    let mut rx = state.sse_tx.subscribe();
    
    // Send incoming messages to upstream
    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    Some(Ok(axum::extract::ws::Message::Text(text))) => {
                        if let Ok(request) = serde_json::from_str::<Value>(&text) {
                            let mut upstream = state.upstream.lock().await;
                            match upstream.call(&request) {
                                Ok(reply) => {
                                    let _ = socket.send(axum::extract::ws::Message::Text(reply.to_string())).await;
                                }
                                Err(Failure::Auth) => {
                                    let _ = socket.send(axum::extract::ws::Message::Text(json!({
                                        "jsonrpc": "2.0",
                                        "id": request.get("id").cloned().unwrap_or(Value::Null),
                                        "error": { "code": -32001, "message": NOT_APPROVED }
                                    }).to_string())).await;
                                }
                                Err(Failure::Other(why)) => {
                                    let _ = socket.send(axum::extract::ws::Message::Text(json!({
                                        "jsonrpc": "2.0",
                                        "id": request.get("id").cloned().unwrap_or(Value::Null),
                                        "error": { "code": -32603, "message": why }
                                    }).to_string())).await;
                                }
                            }
                        }
                    }
                    Some(Ok(axum::extract::ws::Message::Close(_))) => break,
                    Some(Ok(_)) => {}, // Ignore Binary, Ping, Pong
                    Some(Err(e)) => {
                        warn!("WebSocket error: {}", e);
                        break;
                    }
                    None => break,
                }
            }
            sse_msg = rx.recv() => {
                if let Ok(data) = sse_msg {
                    let _ = socket.send(axum::extract::ws::Message::Text(data)).await;
                }
            }
        }
    }
}

/// MCP Tool definitions for design orchestration
#[derive(Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

impl McpTool {
    pub fn design_critique() -> Self {
        Self {
            name: "design_critique".to_string(),
            description: "Run design critique on a UI screenshot or HTML".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "URL to analyze" },
                    "html": { "type": "string", "description": "HTML content to analyze" },
                    "viewport": { "type": "string", "description": "Viewport size (e.g., '1280x720')" }
                },
                "required": []
            }),
        }
    }

    pub fn design_audit() -> Self {
        Self {
            name: "design_audit".to_string(),
            description: "Run comprehensive design audit".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" },
                    "html": { "type": "string" },
                    "categories": { "type": "array", "items": { "type": "string" } }
                },
                "required": []
            }),
        }
    }

    pub fn design_polish() -> Self {
        Self {
            name: "design_polish".to_string(),
            description: "Apply polish transformations to fix slop".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "html": { "type": "string" },
                    "rules": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["html"]
            }),
        }
    }

    pub fn design_bolder() -> Self {
        Self {
            name: "design_bolder".to_string(),
            description: "Apply bolder design transformations".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "html": { "type": "string" },
                    "intensity": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
                },
                "required": ["html"]
            }),
        }
    }

    pub fn design_clarify() -> Self {
        Self {
            name: "design_clarify".to_string(),
            description: "Clarify ambiguous UI copy, with African language support".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string" },
                    "language": { "type": "string", "enum": ["swahili", "luganda", "amharic", "hausa", "yoruba", "zulu", "xhosa", "english"] },
                    "context": { "type": "string" }
                },
                "required": ["text"]
            }),
        }
    }

    pub fn design_live_start() -> Self {
        Self {
            name: "design_live_start".to_string(),
            description: "Start live mode browser overlay session".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" }
                },
                "required": ["url"]
            }),
        }
    }

    pub fn design_benchmark_run() -> Self {
        Self {
            name: "design_benchmark_run".to_string(),
            description: "Run design quality benchmarks".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "category": { "type": "string", "enum": ["detector", "mcp", "live", "rule_evaluation", "memory", "startup", "ci_cd"] },
                    "iterations": { "type": "integer", "default": 100 }
                },
                "required": []
            }),
        }
    }

    pub fn design_viz_generate() -> Self {
        Self {
            name: "design_viz_generate".to_string(),
            description: "Generate interactive architecture visualization".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "format": { "type": "string", "enum": ["html", "json", "mermaid"] },
                    "view": { "type": "string", "enum": ["overview", "crates", "commands", "detectors", "mcp", "skills", "ci", "data_flow"] }
                },
                "required": []
            }),
        }
    }

    /// Get all design orchestration tools
    pub fn all_design_tools() -> Vec<Self> {
        vec![
            Self::design_critique(),
            Self::design_audit(),
            Self::design_polish(),
            Self::design_bolder(),
            Self::design_clarify(),
            Self::design_live_start(),
            Self::design_benchmark_run(),
            Self::design_viz_generate(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_tool_serialization() {
        let tool = McpTool::design_critique();
        let json = serde_json::to_value(&tool).unwrap();
        assert_eq!(json["name"], "design_critique");
        assert!(json["input_schema"].is_object());
    }

    #[test]
    fn test_all_design_tools() {
        let tools = McpTool::all_design_tools();
        assert_eq!(tools.len(), 8);
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"design_critique"));
        assert!(names.contains(&"design_audit"));
        assert!(names.contains(&"design_polish"));
        assert!(names.contains(&"design_bolder"));
        assert!(names.contains(&"design_clarify"));
        assert!(names.contains(&"design_live_start"));
        assert!(names.contains(&"design_benchmark_run"));
        assert!(names.contains(&"design_viz_generate"));
    }
}