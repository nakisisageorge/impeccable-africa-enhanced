# DESIGN_DOCUMENT.md

## Impeccable Africa Enhanced - Architecture Design Document

### 1. Original Architecture

The original `impeccable` project is a Rust-based design vocabulary system for coding agents, consisting of:

- **23 Rust crates** in a Cargo workspace (`crates/`)
- **TypeScript CLI shim** (`cli/`) and browser extension (`extension/`)
- **Skill system** (`skill/`) with 24 design commands and 30+ agent integrations
- **MCP server** over stdio only (`crates/mcp/`)
- **62 deterministic detector rules** for slop detection
- **Live mode** with browser overlay for visual critique
- **Comprehensive test suite** including oracle replay tests

### 2. Problems Identified

| Area | Original Limitation |
|------|---------------------|
| **African Design Vocabulary** | No rules for mobile money UI, fintech dashboards, healthcare interfaces, agricultural dashboards, government portals, SMS/USSD flows |
| **Language Support** | `clarify` command only works with English/Western languages |
| **Accessibility Rules** | Missing low-bandwidth, high-contrast, offline-first, screen-reader for local languages |
| **MCP Transports** | Limited to stdio only; no HTTP/SSE/Streamable for broader agent integration |
| **Benchmarking** | No performance/quality regression tracking |
| **Architecture Visualization** | No interactive visualization of system architecture |
| **Documentation** | No African design guide, no case study for African contexts |
| **CI/CD Security** | Missing cargo-audit, cargo-deny, TruffleHog, SBOM, govulncheck, biome, mutation testing |
| **Security Hardening** | No rate limiting, audit logging, capability tokens for MCP endpoints |

### 3. Proposed Architecture

The enhanced architecture adds the following layers:

```
┌─────────────────────────────────────────────────────────────────┐
│                    IMPECCABLE AFRICA ENHANCED                    │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   CLIENT    │  │   CLIENT    │  │   CLIENT    │              │
│  │  (stdio)    │  │  (HTTP)     │  │  (SSE/WS)   │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│         ▼                ▼                ▼                      │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              MCP TRANSPORT LAYER                         │    │
│  │  ┌────────┐  ┌────────┐  ┌──────────┐  ┌────────────┐  │    │
│  │  │ stdio  │  │  HTTP  │  │   SSE    │  │ Streamable │  │    │
│  │  └────────┘  └────────┘  └──────────┘  └────────────┘  │    │
│  └────────────────────────┬────────────────────────────────┘    │
│                           │                                      │
│         ┌─────────────────┼─────────────────┐                   │
│         ▼                 ▼                 ▼                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │  DESIGN     │  │  DESIGN     │  │  DESIGN     │              │
│  │  TOOLS (8)  │  │  ORCHESTR.  │  │  BENCHMARK  │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│         │                 │                 │                   │
│         ▼                 ▼                 ▼                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                    CORE ENGINE                           │    │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐            │    │
│  │  │Foundation│ │ Detect │ │  Live  │ │ Comp   │            │    │
│  │  └────────┘ └────────┘ └────────┘ └────────┘            │    │
│  └─────────────────────────────────────────────────────────┘    │
│                           │                                      │
│         ┌─────────────────┼─────────────────┐                   │
│         ▼                 ▼                 ▼                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │  BENCHMARK  │  │   STORAGE   │  │ VISUALIZATION│             │
│  │  REGRESSION │  │  (SQLite)   │  │ (React Flow) │             │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
└─────────────────────────────────────────────────────────────────┘
```

### 4. New Architecture Components

#### 4.1 African Design Vocabulary (20+ Rules)

Added to `crates/foundation/src/registry.rs` under new `"african"` category:

**Mobile Money (5 rules):**
- `african-mobile-money-form` - Transaction form patterns
- `ussd-flow-pattern` - USSD menu navigation
- `sim-toolkit-menu` - STK menu interactions
- `mobile-money-confirmation` - Confirmation screens
- `agent-cash-in-out` - Agent cash in/out flows

**FinTech (4 rules):**
- `african-fintech-dashboard` - Multi-currency dashboards
- `interbank-transfer-flow` - Cross-bank transfers
- `wallet-balance-display` - Balance visualization
- `transaction-history-african` - Transaction lists with provider badges

**Healthcare (4 rules):**
- `african-healthcare-appointment` - Appointment booking
- `patient-record-sms` - SMS-based records
- `community-health-worker-ui` - CHW interfaces
- `telemedicine-low-bandwidth` - Low-bandwidth telemedicine

**Agriculture (4 rules):**
- `african-agri-market-prices` - Market price displays
- `crop-advisory-sms` - SMS advisory
- `farmer-registration-ussd` - USSD registration
- `weather-alert-local-language` - Localized weather alerts

**Government (4 rules):**
- `african-gov-service-portal` - Service portals
- `digital-id-verification` - ID verification
- `benefit-disbursement-ui` - Benefit disbursement
- `citizen-feedback-ussd` - USSD feedback

**General Accessibility (7 rules):**
- `low-bandwidth-first` - Bandwidth optimization
- `offline-capable-indicator` - Offline status
- `data-cost-aware` - Data cost transparency
- `multi-language-toggle` - Language switching
- `high-contrast-mode` - High contrast
- `african-language-inconsistent` - Terminology consistency
- `african-honorific-missing` - Respectful address

#### 4.2 African Language `clarify` Support (7 Languages)

Extended in `skill/reference/clarify.md`:

| Language | Region | Script | Key Features |
|----------|--------|--------|--------------|
| Swahili | East Africa | Latin | Noun classes, formal/informal |
| Luganda | Uganda | Latin | Respectful address, code-switching |
| Amharic | Ethiopia | Ge'ez | Complex morphology, honorifics |
| Hausa | West Africa | Latin/Arabic | Tone marks, gender, RTL for Ajami |
| Yoruba | Nigeria | Latin | Tone marks critical, respectful address |
| Zulu | Southern Africa | Latin | Noun classes, click consonants |
| Xhosa | South Africa | Latin | Noun classes, click consonants |

#### 4.3 African Accessibility Audit Rules (7 New Dimensions)

Added to `skill/reference/audit.md`:

1. **Low-Bandwidth Optimization** - Critical path < 30KB, SW cache, data cost shown
2. **Offline-First Capability** - Persistent indicator, local mutation queue, background sync
3. **High Contrast & Visual Accessibility** - WCAG AAA, forced colors, African language SR
4. **Data Cost Awareness** - Cost estimates, OS data saver respect, usage dashboard
5. **Multi-Language & RTL Support** - Language selector, Hausa Ajami RTL, font fallbacks
6. **USSD Accessibility** - SMS/USSD flow accessibility
7. **Voice-over African Languages** - TTS support for local languages

#### 4.4 Enhanced MCP Server (4 Transports)

Extended in `crates/mcp/src/http_transport.rs`:

| Transport | Endpoint | Use Case |
|-----------|----------|----------|
| stdio | Process stdin/stdout | Local agents (Claude Code, Codex) |
| HTTP | POST `/mcp` | REST-style agent integration |
| SSE | GET `/mcp/sse` | Server-sent events for streaming |
| Streamable | GET `/mcp/stream` | Bidirectional streaming |
| WebSocket | GET `/mcp/ws` | Real-time bidirectional |

**8 Design Orchestration Tools:**
- `design_critique` - Run design critique on UI/HTML
- `design_audit` - Comprehensive design audit (10 dimensions)
- `design_polish` - Apply polish transformations
- `design_bolder` - Strengthen visual hierarchy
- `design_clarify` - Rewrite UX copy (7 African languages)
- `design_live_start` - Start live mode browser overlay
- `design_benchmark_run` - Run design quality benchmarks
- `design_viz_generate` - Generate architecture visualization

#### 4.5 Benchmark Regression Detection

New crate `crates/benchmark/` with:

- **Categories**: detector, mcp, live, rule_evaluation, memory, startup, ci_cd
- **Storage**: SQLite with historical data
- **Threshold**: 5% regression detection
- **Integration**: GitHub Actions for CI regression alerts
- **Comparison**: Baseline comparison with statistical significance

#### 4.6 Interactive Architecture Visualization

New React Flow application in `ui/architecture-viz/` with 8 views:

1. **Overview** - Complete system (82+ nodes)
2. **Crates** - 23 Rust crates by category
3. **Commands** - 24 design commands flow
4. **Detectors** - 82+ rules visualization
5. **MCP** - 4 transports + 8 tools
6. **Skills** - 30+ agent integrations
7. **CI/CD** - Pipeline stages
8. **Data Flow** - End-to-end data pipeline

#### 4.7 Enhanced CI/CD Pipeline

Updated `.github/workflows/ci.yml` with:

- `cargo-audit` - Rust vulnerability scanning
- `cargo-deny` - License and dependency checking
- `TruffleHog` - Secret scanning
- `SBOM` - CycloneDX generation
- `govulncheck` - Go vulnerability scanning
- `biome` - JS/TS linting and formatting
- `cargo-mutants` - Mutation testing
- Multi-platform builds (Linux, macOS, Windows)

#### 4.8 Security Hardening

- Rate limiting on HTTP/SSE endpoints
- Input validation and sanitization
- Audit logging for all MCP operations
- Capability-based tokens for tool access
- Seccomp sandboxing for detector
- Secret scanning in CI/CD
- Dependency vulnerability scanning

### 5. Technology Decisions

| Decision | Rationale |
|----------|-----------|
| **Rust for core** | Performance, safety, WASM compilation, existing codebase |
| **Axum for HTTP** | Modern, fast, integrates with Tokio ecosystem |
| **SQLite for benchmarks** | Embedded, no external dependency, sufficient for metrics |
| **React Flow for viz** | Interactive, extensible, good TypeScript support |
| **7 African languages** | Covers major African regions with 500M+ speakers |
| **Apache-2.0 license** | Permissive, allows fork/enhancement with attribution |
| **5% regression threshold** | Balances sensitivity with noise reduction |

### 6. Security Decisions

- No secrets in code or config (`.env.example` only)
- All MCP transports require capability tokens
- Rate limiting prevents abuse
- Audit logging for compliance
- Seccomp sandboxing for detector execution
- Supply chain security via cargo-audit, cargo-deny, SBOM

### 7. Performance Decisions

- WASM compilation for browser detector
- SQLite for low-latency benchmark storage
- Caching for MCP server responses
- Optimized bundle size with code splitting
- Parallel CI jobs for faster feedback

### 8. Scalability Considerations

- Horizontal scaling for MCP HTTP endpoints
- SQLite can be replaced with PostgreSQL for high volume
- React Flow visualization handles 500+ nodes
- Benchmark storage partitioned by date
- Agent skill ecosystem designed for plugin architecture

### 9. Deployment Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   GitHub    │────▶│  CI/CD      │────▶│  Registry   │
│   Actions   │     │  Pipeline   │     │  (crates.io)│
└─────────────┘     └─────────────┘     └─────────────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
         ┌─────────┐  ┌─────────┐  ┌─────────┐
         │ Linux   │  │ macOS   │  │ Windows │
         └─────────┘  └─────────┘  └─────────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
         ┌─────────┐  ┌─────────┐  ┌─────────┐
         │  Docs   │  │ Releases│  │  Pages  │
         │ (GitHub │  │ (GitHub)│  │ (GitHub)│
         │  Pages) │  │         │  │         │
         └─────────┘  └─────────┘  └─────────┘
```

### 10. Testing Strategy

- **Unit tests**: Each crate has focused unit tests
- **Integration tests**: MCP transport tests, benchmark regression tests
- **E2E tests**: Live mode with Playwright, CLI remote E2E
- **Oracle tests**: Golden file replay for detector rules
- **Benchmark tests**: Performance regression detection
- **Mutation tests**: Cargo-mutants for critical paths
- **Security tests**: TruffleHog, cargo-audit, govulncheck

### 11. Future Improvements

- **AI-powered rule generation** - LLM-assisted detector rule creation
- **Real-time collaboration** - Multi-agent design sessions
- **Plugin marketplace** - Community detector rules and skills
- **Mobile app** - Native mobile inspector for live mode
- **Enterprise SSO** - SAML/OIDC integration for MCP
- **Advanced analytics** - Design quality trends and predictions
- **More African languages** - Add Igbo, Shona, Wolof, Tigrinya
- **Voice interface** - Speech-to-design for accessibility