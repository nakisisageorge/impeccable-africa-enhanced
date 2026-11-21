# INTERACTIVE_DOC_TREE.md

# Interactive Documentation Tree - Impeccable Africa Enhanced

## Navigation Structure

```
impeccable-africa-enhanced/
├── 📁 Core Documentation
│   ├── README.md                    # Project overview & quick start
│   ├── LICENSE                      # Apache-2.0 license
│   ├── NOTICE.md                    # Attribution notices
│   ├── CHANGELOG.md                 # Version history
│   └── CONTRIBUTING.md              # Contribution guidelines
│
├── 📁 Architecture & Design
│   ├── DESIGN_DOCUMENT.md           # Architecture decisions & rationale
│   ├── PORTFOLIO_CASE_STUDY.md      # Business impact & engineering approach
│   ├── BEFORE_AFTER.md              # Detailed before/after comparisons
│   ├── INTERACTIVE_DOC_TREE.md      # This file - navigation guide
│   └── 📁 Architecture Visualization
│       └── architecture/            # Interactive React Flow viz (8 views)
│           ├── index.html           # Main visualization entry
│           ├── assets/              # JS/CSS bundles
│           └── Views:
│               ├── Overview         # Complete system (82+ nodes)
│               ├── Crates           # 23 Rust crates by category
│               ├── Commands         # 24 design commands flow
│               ├── Detectors        # 82+ rules visualization
│               ├── MCP              # 5 transports + 8 tools
│               ├── Skills           # 30+ agent integrations
│               ├── CI/CD            # Pipeline + security tools
│               └── Data Flow        # End-to-end pipeline
│
├── 📁 African Context Guides
│   ├── AFRICAN_DESIGN_GUIDE.md      # African patterns reference (20 rules)
│   ├── AFRICAN_LANGUAGE_GUIDE.md    # 7 language implementation details
│   ├── AFRICAN_ACCESSIBILITY_GUIDE.md # 5 African audit dimensions
│   └── AFRICAN_MOBILE_MONEY_GUIDE.md # 18+ provider integration
│
├── 📁 Technical References
│   ├── RULE_GUIDE.md                # All 82+ detector rules documented
│   ├── COMMAND_GUIDE.md             # 24 design commands reference
│   ├── MCP_GUIDE.md                 # MCP server usage (5 transports)
│   ├── BENCHMARK_GUIDE.md           # Regression detection system
│   ├── CLI_GUIDE.md                 # CLI usage & verb reference
│   └── SKILL_GUIDE.md               # 30+ agent integrations
│
├── 📁 Development
│   ├── DEVELOP.md                   # Development setup
│   ├── ENGINE.md                    # Engine architecture
│   ├── BUILD.md                     # Build system (bun + cargo)
│   ├── TESTING.md                   # Test strategies & commands
│   └── RELEASE.md                   # Release process
│
├── 📁 Crate Documentation (auto-generated)
│   ├── crates/foundation/           # Rule registry, color, CSS, JS
│   ├── crates/detect/               # Slop detector implementation
│   ├── crates/mcp/                  # MCP server (5 transports)
│   ├── crates/benchmark/            # Regression detection
│   ├── crates/live/                 # Live mode, browser overlay
│   ├── crates/comp/                 # Component review
│   ├── crates/agents/               # Agent skill definitions
│   └── ... (23 total crates)
│
└── 📁 Distribution Artifacts
    ├── .github/workflows/           # CI/CD pipelines
    ├── dist/                        # Built distributions
    ├── plugin/                      # VS Code extension
    ├── extension/                   # Browser extension
    └── cli/                         # npm shim
```

## Quick Links by Role

### For Developers Integrating Impeccable
1. Start with [README.md](../README.md) - Installation & quick start
2. [CLI_GUIDE.md](./CLI_GUIDE.md) - Verb reference & usage
3. [MCP_GUIDE.md](./MCP_GUIDE.md) - Agent integration via MCP
4. [ARCHITECTURE_VIZ](../architecture/) - Understand system structure

### For African Market Developers
1. [AFRICAN_DESIGN_GUIDE.md](./AFRICAN_DESIGN_GUIDE.md) - 20 African patterns
2. [AFRICAN_LANGUAGE_GUIDE.md](./AFRICAN_LANGUAGE_GUIDE.md) - 7 language support
3. [AFRICAN_ACCESSIBILITY_GUIDE.md](./AFRICAN_ACCESSIBILITY_GUIDE.md) - 5 audit dims
4. [AFRICAN_MOBILE_MONEY_GUIDE.md](./AFRICAN_MOBILE_MONEY_GUIDE.md) - 18+ providers

### For Security Engineers
1. [MCP_GUIDE.md](./MCP_GUIDE.md#security) - Capability tokens, rate limiting
2. [CI/CD Security](./DESIGN_DOCUMENT.md#8-enhanced-cicd--security) - 9 security tools
3. [SECURITY.md](../SECURITY.md) - Security policy & reporting

### For Researchers/Analysts
1. [PORTFOLIO_CASE_STUDY.md](./PORTFOLIO_CASE_STUDY.md) - Engineering approach
3. [BEFORE_AFTER.md](./BEFORE_AFTER.md) - Quantitative comparisons
4. [BENCHMARK_GUIDE.md](./BENCHMARK_GUIDE.md) - Regression detection

### For Contributors
1. [CONTRIBUTING.md](../CONTRIBUTING.md) - Guidelines
2. [DEVELOP.md](../DEVELOP.md) - Local development
3. [TESTING.md](../TESTING.md) - Test commands
4. [RULE_GUIDE.md](./RULE_GUIDE.md) - Adding new detector rules

## Interactive Architecture Visualization

The architecture visualization at `/architecture/` provides 8 interactive views:

| View | Nodes | Edges | Focus |
|------|-------|-------|-------|
| **Overview** | 82+ | 120+ | Complete system |
| **Crates** | 23 | 35 | Rust workspace structure |
| **Commands** | 31 | 42 | Design command flow |
| **Detectors** | 82+ | 95 | Rule categories & deps |
| **MCP** | 15 | 28 | Transports, tools, sessions |
| **Skills** | 35 | 48 | Agent integrations |
| **CI/CD** | 18 | 25 | Pipeline stages |
| **Data Flow** | 28 | 40 | End-to-end pipeline |

### Visualization Controls
- **Pan**: Click + drag background
- **Zoom**: Mouse wheel or pinch
- **Mini-map**: Bottom-right corner
- **Node details**: Click any node
- **View switcher**: Top header tabs
- **Legend**: Bottom-right panel
- **Export**: HTML/JSON/Mermaid via `design_viz_generate` MCP tool

## Search Index

### By Feature Area
- **African Design Patterns** → AFRICAN_DESIGN_GUIDE.md
- **Language Support** → AFRICAN_LANGUAGE_GUIDE.md
- **Accessibility** → AFRICAN_ACCESSIBILITY_GUIDE.md
- **Mobile Money** → AFRICAN_MOBILE_MONEY_GUIDE.md
- **MCP Integration** → MCP_GUIDE.md
- **Benchmarking** → BENCHMARK_GUIDE.md
- **All Rules** → RULE_GUIDE.md
- **All Commands** → COMMAND_GUIDE.md
- **Agent Skills** → SKILL_GUIDE.md

### By Technical Component
- **Foundation Crate** → crates/foundation/
- **Detector Crate** → crates/detect/
- **MCP Crate** → crates/mcp/
- **Benchmark Crate** → crates/benchmark/
- **Live Crate** → crates/live/
- **Comp Crate** → crates/comp/

## Version Compatibility

| Doc Version | Impeccable Version | Engine Version | Status |
|-------------|-------------------|----------------|--------|
| 1.0 | 0.1.27 | v0.1.0 | Current |
| 0.9 | 0.1.26 | v0.1.0 | Archived |

## Feedback & Updates

- **Documentation issues**: [GitHub Issues](https://github.com/nakisisageorge/impeccable-africa-enhanced/issues)
- **Architecture feedback**: [Discussions](https://github.com/nakisisageorge/impeccable-africa-enhanced/discussions)
- **African context suggestions**: Label `african-context` on issues
- **Security concerns**: See SECURITY.md for responsible disclosure

---

*Last updated: 2026-11-21 | Generated for impeccable-africa-enhanced v0.1.27*