# Impeccable Africa Enhanced

**Enhanced Impeccable with African design vocabulary, 7 African languages, 5 African accessibility audit dimensions, MCP server with 5 transports, benchmark regression detection, interactive architecture visualization, comprehensive documentation, and enhanced CI/CD.**

This is a **Mode B (Substantial Enhancement Fork)** of the original [Impeccable](https://github.com/pbakaus/impeccable) by Paul Bakaus, licensed under Apache-2.0. All improvements are built on top of the upstream project with full attribution.

## 🌍 African Enhancements

### African Design Vocabulary (20 Patterns)
New detector rules in the `african` category addressing uniquely African digital realities:

**Mobile Money (5 rules)**
- `african-mobile-money-form` — Transaction forms with provider branding, local currency, fee transparency, USSD fallback, data cost estimates
- `ussd-flow-pattern` — Hierarchical USSD menus with breadcrumbs, numeric shortcuts, session timeout handling, SMS fallback
- `sim-toolkit-menu` — STK menus within 160-char SMS limits, proactive commands, 2G SIM browser compatibility
- `mobile-money-confirmation` — Recipient masking, local currency, fee breakdown, PIN/biometric prompt, transaction reference
- `agent-cash-in-out` — Agent verification, float balance, denomination breakdown, receipt generation, SMS confirmation

**FinTech (4 rules)**
- `african-fintech-dashboard` — Multi-currency balances (10 African currencies), 12+ mobile money providers, low-data mode, offline caching
- `interbank-transfer-flow` — NIBSS, PesaLink, BankservAfrica, GHIPSS integration with real-time status and fees
- `wallet-balance-display` — Consolidated view across mobile money, bank, crypto, cash with currency conversion
- `transaction-history-african` — Provider badges, currency tags, fee breakdown, USSD reference codes, CSV/PDF export

**Healthcare (4 rules)**
- `african-healthcare-appointment` — CHW referrals, SMS/USSD reminders, facility tier selection, NHIF/NHIS integration
- `patient-record-sms` — Structured SMS formats, 7 local languages, facility codes, opt-out mechanisms
- `community-health-worker-ui` — Offline-first, icon-heavy navigation, voice input, emergency escalation, background sync
- `telemedicine-low-bandwidth` — Audio-only mode, async consultation, image compression, 2G/Edge support

**Agriculture (4 rules)**
- `african-agri-market-prices` — Mandi/market prices, grade indicators, trends, SMS/USSD access, local languages
- `crop-advisory-sms` — Crop-specific, calendar-timed, local language, input sourcing, feedback loop
- `farmer-registration-ussd` — Geo-location, crop types, land size, ID verification, extension service linkage
- `weather-alert-local-language` — Local languages, indigenous indicators, actionable advice, multi-channel delivery

**Government (4 rules)**
- `african-gov-service-portal` — Service catalog, eligibility checker, document checklist, application tracker, multi-channel payment, certificate download, low-data mode, WCAG AA + local language screen readers
- `digital-id-verification` — NIN, Ghana Card, Huduma Namba, NIDA integration with biometric/OTP/offline fallback
- `benefit-disbursement-ui` — Program details, payment methods, collection points, status tracking, USSD balance, grievance link, language toggle
- `citizen-feedback-ussd` — Anonymous reporting, category selection, geo-tagging, reference numbers, SMS follow-up

**General Accessibility (7 rules)**
- `low-bandwidth-first` — Critical path < 30KB on 2G/3G, WebP/AVIF, lazy loading, SW cache, data cost estimates
- `offline-capable-indicator` — Persistent online/offline/syncing status, local mutation queue, 24h+ cache, background sync
- `data-cost-aware` — Estimated data cost per action, OS data saver respect, compression defaults, Wi-Fi selective sync
- `multi-language-toggle` — Flag + native name + English, RTL for Hausa Ajami, font fallback for Ge'ez/N'Ko/Tifinagh, 20-40% expansion allowance
- `high-contrast-mode` — WCAG AAA, forced colors, African screen reader support
- `ussd-accessibility` — SMS/USSD flow accessibility patterns
- `voice-over-african-languages` — TTS support for 7 African languages

### African Language `clarify` Support (7 Languages)
The `design_clarify` MCP tool now supports rewriting ambiguous UX copy in:
| Language | Code | Region | Script |
|----------|------|--------|--------|
| Kiswahili | `swahili` | East Africa | Latin |
| Luganda | `luganda` | Uganda | Latin |
| Amharic | `amharic` | Ethiopia | Ge'ez |
| Hausa | `hausa` | West Africa | Latin/Arabic |
| Yoruba | `yoruba` | Nigeria | Latin |
| Zulu | `zulu` | Southern Africa | Latin |
| Xhosa | `xhosa` | South Africa | Latin |

### African Accessibility Audit (5 Dimensions + 2 Bonus)
The `design_audit` MCP tool includes 5 African-specific audit dimensions:
1. **Low-bandwidth** — Critical path < 30KB, service worker caching
2. **Offline-first** — Mutation queue, background sync, graceful degradation
3. **High-contrast** — WCAG AAA, forced colors, African screen reader support
3. **Data-cost** — Cost estimates, OS data saver respect, selective sync
4. **Multi-language** — 7 African languages, RTL, font fallback, expansion allowance
5. **USSD Accessibility** — SMS/USSD flow accessibility patterns (bonus)
6. **Voice-over African Languages** — TTS for local languages (bonus)

## 🚀 MCP Server with 5 Transports
The MCP server now supports 5 transport protocols for broader agent integration:

| Transport | Use Case |
|-----------|----------|
| **stdio** | Local agents (Claude Code, Codex, Cursor, Hermes) |
| **HTTP POST** | REST-style agents, serverless functions |
| **SSE** | Streaming responses, long-running operations |
| **Streamable HTTP** | Bidirectional streaming over HTTP |
| **WebSocket** | Real-time bidirectional communication |

### 8 Design Orchestration Tools
All available across all transports:
1. `design_critique` — Comprehensive design critique on UI/HTML
2. `design_audit` — 10-dimension audit (5 standard + 5 African)
3. `design_polish` — Fix detected slop patterns
4. `design_bolder` — Strengthen visual hierarchy
5. `design_clarify` — Rewrite copy in 7 African languages
6. `design_live_start` — Live mode browser overlay
7. `design_benchmark_run` — Benchmarks with regression detection
8. `design_viz_generate` — Interactive architecture visualization

## 📊 Benchmark Regression Detection
- **7 Categories**: detector, mcp, live, rule_evaluation, memory, startup, ci_cd
- **5% Threshold** for regression detection
- **SQLite Storage** for historical data
- **GitHub Actions Integration** with SARIF upload

## 🎨 Interactive Architecture Visualization
React Flow-based visualization with 8 views:
- Overview, Crates, Commands, Detectors, MCP, Skills, CI, Data Flow
- 82+ nodes, exportable as HTML/JSON/Mermaid

## 📚 Comprehensive Documentation (7 Guides)
- `AFRICAN_DESIGN_GUIDE.md` — All 20 patterns with examples
- `MCP_GUIDE.md` — 5 transports, 8 tools, auth, rate limiting, audit logging
- `DESIGN_DOCUMENT.md` — Architecture decisions
- `PORTFOLIO_CASE_STUDY.md` — Business impact analysis
- `BEFORE_AFTER.md` — Visual comparisons
- `RULE_GUIDE.md` — All 82+ rules documented
- `INTERACTIVE_DOC_TREE.md` — Navigation

## 🔒 Enhanced CI/CD Security Pipeline
- **cargo-audit** — Rust vulnerability scanning
- **cargo-deny** — Advisories, licenses, bans, sources
- **TruffleHog** — Secret scanning
- **SBOM Generation** — CycloneDX format
- **govulncheck** — Go vulnerability scanning
- **Biome** — JS/TS linting
- **Mutation Testing** — cargo-mutants
- **Dependency Review** — PR dependency analysis
- **Multi-platform builds** — Linux, macOS, Windows

## 🛡️ Security Hardening
- Rate limiting on all HTTP transports (100 req/min HTTP, 50 conn/min SSE/Streamable/WS)
- Capability-based tokens for tool access
- Audit logging for all MCP operations
- Input validation and sanitization
- Seccomp sandboxing for detector

## 📦 Installation

```bash
# From source
git clone https://github.com/nakisisageorge/impeccable-africa-enhanced
cd impeccable-africa-enhanced
cargo build --release -p impeccable

# The binary will be at target/release/impeccable
# Set IMPECCABLE_BIN to use it with the CLI
export IMPECCABLE_BIN=/path/to/impeccable
npx impeccable install
```

## 🔗 Original Project Attribution
- **Original Repository**: https://github.com/pbakaus/impeccable
- **Original Author**: Paul Bakaus
- **License**: Apache-2.0
- **Upstream Version**: 0.1.27 (ENGINE_VERSION)

This fork maintains full compatibility with the upstream Impeccable skill ecosystem (30+ agent integrations) while adding African market enhancements.

## 📄 License
Apache-2.0 — See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md) for attribution requirements.

## 🤝 Contributing
See [DEVELOP.md](docs/DEVELOP.md) for development guidelines.

## 📞 Contact
**George Nakisisa** — Senior Software Engineer · AI Engineer · Cybersecurity
- GitHub: [nakisisageorge](https://github.com/nakisisageorge)
- Website: [gictafrica.com](https://gictafrica.com)
- Email: nakisisageorge@gmail.com
- Company: GICT Africa Technologies
- Location: Kampala, Uganda