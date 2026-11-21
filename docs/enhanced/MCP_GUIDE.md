# MCP_GUIDE.md

# MCP Server Guide - Impeccable Africa Enhanced

## Overview

The Impeccable Africa Enhanced MCP (Model Context Protocol) server provides 5 transport options and 8 design orchestration tools, enabling coding agents to leverage the full design vocabulary system for African market contexts.

**Version**: 0.1.27
**Protocol**: 2025-06-18
**Transports**: stdio, HTTP, SSE, Streamable HTTP, WebSocket

---

## Quick Start

### For Agent Developers

```json
// Claude Code / Codex / Cursor MCP config
{
  "mcpServers": {
    "impeccable": {
      "command": "impeccable",
      "args": ["mcp"],
      "env": {
        "IMPECCABLE_BIN": "/path/to/impeccable"
      }
    }
  }
}
```

### For HTTP-based Agents

```python
# Python agent using HTTP transport
import httpx
import json

async def call_impeccable_tool(tool_name: str, args: dict):
    async with httpx.AsyncClient() as client:
        response = await client.post(
            "http://localhost:3000/mcp",
            headers={"mcp-session-id": "agent-session-1"},
            json={
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/call",
                "params": {
                    "name": tool_name,
                    "arguments": args
                }
            }
        )
    return response.json()
```

---

## Transport Configuration

### 1. stdio (Default)
**Use case**: Local agents (Claude Code, Codex, Cursor, Hermes)

```bash
# Start stdio server
impeccable mcp
```

**Configuration**: No additional config needed. Agent starts the process.

---

### 2. HTTP POST
**Use case**: REST-style agent integration, serverless functions

```bash
# Start HTTP server (includes all transports)
impeccable mcp-http --port 3000
```

**Endpoint**: `POST http://localhost:3000/mcp`

**Headers**:
```
Content-Type: application/json
mcp-session-id: your-session-id  # Optional, auto-generated if omitted
```

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "design_critique",
    "arguments": {
      "url": "https://example.com",
      "viewport": "1280x720"
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Design critique results..."
      }
    ]
  }
}
```

---

### 3. Server-Sent Events (SSE)
**Use case**: Streaming responses, long-running operations

```bash
# Connect to SSE endpoint
curl -N "http://localhost:3000/mcp/sse?session_id=your-session-id"
```

**Response Stream**:
```
event: message
data: {"jsonrpc":"2.0","method":"notifications/progress","params":{"progressToken":1,"progress":0,"message":"Waiting for you to approve in the browser"}}

event: message
data: {"jsonrpc":"2.0","id":1,"result":{"content":[{"type":"text","text":"Critique complete"}]}}
```

**Keep-alive**: 30-second intervals

---

### 4. Streamable HTTP
**Use case**: Bidirectional streaming over HTTP

```bash
# Connect with streaming
curl -N -X GET "http://localhost:3000/mcp/stream?session_id=your-session-id" \
  -H "Accept: text/event-stream"
```

Similar to SSE but supports client-to-server streaming in the same connection.

---

### 5. WebSocket
**Use case**: Real-time bidirectional communication

```javascript
// JavaScript WebSocket client
const ws = new WebSocket("ws://localhost:3000/mcp/ws?session_id=your-session-id");

ws.onopen = () => {
  ws.send(JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "tools/call",
    params: {
      name: "design_critique",
      arguments: { url: "https://example.com" }
    }
  }));
};

ws.onmessage = (event) => {
  const response = JSON.parse(event.data);
  console.log(response);
};
```

---

## Design Orchestration Tools

All 8 tools are available across all transports.

### 1. design_critique
Run comprehensive design critique on a UI screenshot or HTML.

**Parameters**:
```typescript
interface DesignCritiqueArgs {
  url?: string;           // URL to analyze (requires browser)
  html?: string;          // HTML content to analyze
  viewport?: string;      // Viewport size (e.g., "1280x720")
}
```

**Example**:
```json
{
  "name": "design_critique",
  "arguments": {
    "url": "https://myapp.com/dashboard",
    "viewport": "1280x720"
  }
}
```

**Returns**: Structured critique with slop findings, quality issues, and recommendations.

---

### 2. design_audit
Run comprehensive design audit across 10 dimensions (5 standard + 5 African).

**Parameters**:
```typescript
interface DesignAuditArgs {
  url?: string;
  html?: string;
  categories?: string[];  // Optional: specific categories to audit
}
```

**Categories**:
- `accessibility` - WCAG AA compliance
- `performance` - Core Web Vitals, optimization
- `theming` - Design token usage, dark mode
- `responsive` - Mobile/tablet/desktop, touch targets
- `integrity` - Detector findings, design system drift
- `low-bandwidth` - Critical path < 30KB, SW cache
- `offline-first` - Mutation queue, background sync
- `high-contrast` - WCAG AAA, forced colors, African SR
- `data-cost` - Cost estimates, OS data saver respect
- `multi-language` - 7 African languages, RTL, expansion

**Example**:
```json
{
  "name": "design_audit",
  "arguments": {
    "url": "https://myapp.com",
    "categories": ["accessibility", "low-bandwidth", "multi-language"]
  }
}
```

**Returns**: Audit health score (0-40), dimension scores, P0-P3 findings, recommended commands.

---

### 3. design_polish
Apply polish transformations to fix detected slop patterns.

**Parameters**:
```typescript
interface DesignPolishArgs {
  html: string;           // Required: HTML to polish
  rules?: string[];       // Optional: specific rules to apply
}
```

**Example**:
```json
{
  "name": "design_polish",
  "arguments": {
    "html": "<div class='card'><div class='card'>Nested</div></div>",
    "rules": ["nested-cards", "monotonous-spacing", "gradient-text"]
  }
}
```

**Returns**: Polished HTML with slop patterns fixed.

---

### 4. design_bolder
Strengthen visual hierarchy with bolder design transformations.

**Parameters**:
```typescript
interface DesignBolderArgs {
  html: string;           // Required: HTML to transform
  intensity?: number;     // 0.0 to 1.0 (default: 0.5)
}
```

**Example**:
```json
{
  "name": "design_bolder",
  "arguments": {
    "html": "<h1>Welcome</h1><p>Description</p>",
    "intensity": 0.7
  }
}
```

**Returns**: HTML with strengthened hierarchy (larger headings, better contrast, clearer spacing).

---

### 5. design_clarify
Rewrite ambiguous UX copy with support for 7 African languages.

**Parameters**:
```typescript
interface DesignClarifyArgs {
  text: string;           // Required: Text to clarify
  language?: string;      // "english" | "swahili" | "luganda" | "amharic" | "hausa" | "yoruba" | "zulu" | "xhosa"
  context?: string;       // Optional: UI context for better results
}
```

**Languages**:
| Code | Language | Region | Script |
|------|----------|--------|--------|
| `english` | English | Global | Latin |
| `swahili` | Kiswahili | East Africa | Latin |
| `luganda` | Luganda | Uganda | Latin |
| `amharic` | Amharic | Ethiopia | Ge'ez |
| `hausa` | Hausa | West Africa | Latin/Arabic |
| `yoruba` | Yoruba | Nigeria | Latin |
| `zulu` | Zulu | Southern Africa | Latin |
| `xhosa` | Xhosa | South Africa | Latin |

**Example**:
```json
{
  "name": "design_clarify",
  "arguments": {
    "text": "Click here to proceed with the transaction",
    "language": "swahili",
    "context": "Mobile money confirmation button"
  }
}
```

**Returns**: Clarified copy in target language with cultural adaptations.

**Swahili Output Example**:
```
"Bofya hapa kuendelea na malipo" 
// Instead of literal "Bofya hapa kuendelea na miamala"
// Uses respectful address, noun class agreement
```

---

### 6. design_live_start
Start live mode browser overlay session for real-time design iteration.

**Parameters**:
```typescript
interface DesignLiveStartArgs {
  url: string;            // Required: URL to open in live mode
}
```

**Example**:
```json
{
  "name": "design_live_start",
  "arguments": {
    "url": "https://myapp.com/dashboard"
  }
}
```

**Returns**: Live mode session URL with browser overlay for variant testing.

---

### 7. design_benchmark_run
Run design quality benchmarks with regression detection.

**Parameters**:
```typescript
interface DesignBenchmarkRunArgs {
  category?: "detector" | "mcp" | "live" | "rule_evaluation" | "memory" | "startup" | "ci_cd";
  iterations?: number;    // Default: 100
}
```

**Example**:
```json
{
  "name": "design_benchmark_run",
  "arguments": {
    "category": "detector",
    "iterations": 200
  }
}
```

**Returns**: Benchmark metrics with percentile tracking, regression status vs baseline.

---

### 8. design_viz_generate
Generate interactive architecture visualization.

**Parameters**:
```typescript
interface DesignVizGenerateArgs {
  format?: "html" | "json" | "mermaid";  // Default: "html"
  view?: "overview" | "crates" | "commands" | "detectors" | "mcp" | "skills" | "ci" | "data_flow";
}
```

**Example**:
```json
{
  "name": "design_viz_generate",
  "arguments": {
    "format": "html",
    "view": "overview"
  }
}
```

**Returns**: Visualization output in requested format.

---

## Authentication & Authorization

### Capability Tokens

All HTTP/SSE/Streamable/WS transports require capability tokens for tool access:

```bash
# Generate token (CLI)
impeccable mcp-token generate --tool design_critique --expires 3600

# Use token
curl -H "Authorization: Bearer <token>" \
  -H "mcp-session-id: session-1" \
  -X POST http://localhost:3000/mcp \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"design_critique","arguments":{"url":"https://example.com"}}}'
```

### Rate Limiting

| Transport | Limit | Window |
|-----------|-------|--------|
| HTTP | 100 requests | 1 minute |
| SSE | 50 connections | 1 minute |
| Streamable | 50 connections | 1 minute |
| WebSocket | 20 connections | 1 minute |

Exceeded limits return `429 Too Many Requests` with `Retry-After` header.

### Audit Logging

All MCP operations are logged:

```json
{
  "timestamp": "2026-11-21T10:30:00Z",
  "session_id": "agent-session-1",
  "tool": "design_critique",
  "agent": "claude-code",
  "duration_ms": 1250,
  "status": "success",
  "findings_count": 12
}
```

Access logs via: `impeccable mcp-logs --session <id>`

---

## Error Handling

### Standard Errors

| Code | Message | Resolution |
|------|---------|------------|
| -32700 | Parse error | Valid JSON-RPC 2.0 required |
| -32600 | Invalid request | Check method/params format |
| -32601 | Method not found | Tool name must be valid |
| -32602 | Invalid params | Check parameter schema |
| -32603 | Internal error | Check server logs |
| -32001 | Auth required | Run `impeccable login` |
| -32002 | Rate limited | Wait for `Retry-After` |
| -32003 | Tool error | Check tool-specific error |

### Authentication Flow

1. Agent calls tool → `401 Unauthorized` + `mcp-session-id`
2. Agent receives progress notification: "Waiting for you to approve in the browser"
3. Browser opens for `impeccable login`
4. Agent retries call → succeeds with new session

---

## Deployment

### Local Development

```bash
# Start all transports on localhost:3000
impeccable mcp-http --port 3000

# With custom config
impeccable mcp-http --port 3000 --host 0.0.0.0 --tls-cert cert.pem --tls-key key.pem
```

### Production (Docker)

```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p impeccable

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/impeccable /usr/local/bin/
EXPOSE 3000
CMD ["impeccable", "mcp-http", "--port", "3000", "--host", "0.0.0.0"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: impeccable-mcp
spec:
  replicas: 3
  selector:
    matchLabels:
      app: impeccable-mcp
  template:
    metadata:
      labels:
        app: impeccable-mcp
    spec:
      containers:
      - name: mcp
        image: ghcr.io/nakisisageorge/impeccable-mcp:latest
        ports:
        - containerPort: 3000
        env:
        - name: IMPECCABLE_MCP_PORT
          value: "3000"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: impeccable-mcp
spec:
  selector:
    app: impeccable-mcp
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

---

## Troubleshooting

### Connection Refused
```bash
# Check if server is running
curl http://localhost:3000/mcp -X POST -d '{"jsonrpc":"2.0","id":1,"method":"ping"}'

# Check logs
impeccable mcp-logs --follow
```

### Authentication Loop
```bash
# Clear session and re-login
impeccable mcp-session clear
impeccable login
```

### Rate Limited
```bash
# Check rate limit status
impeccable mcp-rate-limit --session <id>

# Increase limits (admin)
impeccable mcp-config set rate_limit.http=200
```

### Performance Issues
```bash
# Run benchmark to diagnose
impeccable benchmark run --category mcp --iterations 50
```

---

## Integration Examples

### Claude Code
```json
// .claude/mcp.json
{
  "mcpServers": {
    "impeccable": {
      "command": "impeccable",
      "args": ["mcp"],
      "env": {}
    }
  }
}
```

### Cursor
```json
// .cursor/mcp.json
{
  "mcpServers": {
    "impeccable": {
      "command": "impeccable",
      "args": ["mcp"]
    }
  }
}
```

### Custom Agent (Python)
```python
import asyncio
from impeccable_mcp_client import ImpeccableMCPClient

async def main():
    async with ImpeccableMCPClient("http://localhost:3000") as client:
        # Critique a dashboard
        result = await client.design_critique(
            url="https://myapp.com/dashboard",
            viewport="1280x720"
        )
        print(result)
        
        # Clarify copy in Swahili
        clarified = await client.design_clarify(
            text="Enter your PIN to confirm",
            language="swahili",
            context="Mobile money PIN entry"
        )
        print(clarified)

asyncio.run(main())
```

---

## Security Best Practices

1. **Never expose MCP server publicly** without authentication
2. **Use capability tokens** for all production deployments
3. **Enable TLS** for all non-localhost connections
4. **Rotate tokens** regularly (max 1 hour expiry)
5. **Monitor audit logs** for anomalous activity
6. **Restrict tool access** by capability (not all agents need all tools)
7. **Use network policies** in Kubernetes to limit access

---

## Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [Impeccable GitHub](https://github.com/nakisisageorge/impeccable-africa-enhanced)
- [Architecture Visualization](https://nakisisageorge.github.io/impeccable-africa-enhanced/architecture/)
- [Security Policy](https://github.com/nakisisageorge/impeccable-africa-enhanced/security)