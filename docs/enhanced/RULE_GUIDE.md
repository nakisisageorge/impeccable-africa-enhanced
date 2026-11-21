# RULE_GUIDE.md

# Complete Detector Rule Reference - Impeccable Africa Enhanced

## Overview

This guide documents all 82+ detector rules in the impeccable-africa-enhanced system, organized by category. Each rule includes its ID, category, severity, description, skill section, and guideline.

**Total Rules**: 82 (62 original + 20 African patterns)
**Categories**: `slop` (38), `quality` (24), `african` (20)

---

## Category: slop (38 rules)

### Visual Details
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `side-tab` | Side-tab accent border | — | A bar down one side of a rounded box. Use a full border, a background tint, or a leading icon or label instead. | colored accent stripe |
| `border-accent-on-rounded` | Border accent on rounded element | — | Thick accent border on a rounded card — the border clashes with the rounded corners. Remove the border or the border-radius. | colored accent stripe |
| `nested-cards` | Nested cards | — | Cards inside cards create visual noise and excessive depth. Flatten the hierarchy — use spacing, typography, and dividers instead. | Nest cards inside cards |
| `hard-offset-shadow` | Hard offset shadow | advisory | A visible, unblurred shadow displaced by at least 4px can belong to an intentional neobrutalist direction. Check whether it fits the intended design. | review hard offset shadows |
| `dark-glow` | Glowing shadow accents | — | Colored glow shadows — a zero-offset chromatic halo on any background, or any colored blurred shadow on a dark background — are the default "cool" look of AI-generated UIs. | dark mode with glowing accents |
| `radial-halo` | Radial-gradient background halo | — | A chromatic radial-gradient wash used as a decorative background glow on a dark page. Ground the surface with a solid or subtly shifted background instead. | dark mode with glowing accents |
| `radial-spotlight-glow` | Decorative radial spotlight glow | — | An accent-colored radial gradient fading to transparent, dropped behind a hero or section as a "spotlight". Let the surface stand on its own. | dark mode with glowing accents |

### Typography
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `overused-font` | Overused font | — | Inter, Roboto, Fraunces, Geist, Plus Jakarta Sans, and Space Grotesk are used on so many sites they no longer feel distinctive. Choose a face that gives your interface personality. | overused fonts like Inter |
| `flat-type-hierarchy` | Flat type hierarchy | — | Dominant heading and body roles are separated by less than 1.25× at every step. Add at least one stronger size step. | flat type hierarchy |
| `gradient-text` | Gradient text | — | Gradient text is decorative rather than meaningful — a common AI tell. Use solid colors for text. | gradient text for |
| `italic-serif-display` | Italic serif display headline | — | Oversized italic serif as the primary hero headline reads as taste in isolation but has become the universal AI-startup landing page hero. | oversized italic serif as the hero headline |
| `hero-eyebrow-chip` | Hero eyebrow / pill chip | — | A tiny uppercase letter-spaced label sitting immediately above an oversized hero headline is now the default AI SaaS hero. | tiny uppercase tracked label above the hero headline |
| `kicker-above-heading` | Kicker / eyebrow label above heading | — | A tiny tracked uppercase or small-caps label sitting as its own block directly above a heading is banned outright. | kicker or eyebrow labels above headings |
| `numbered-section-labels` | Tiny numbered section labels | advisory | Small numeric index labels riding next to section headings are AI editorial scaffolding. | numbered section markers |
| `extreme-negative-tracking` | Crushed letter spacing | — | Letter-spacing pulled tighter than the point where characters keep their own shapes costs legibility. | letter spacing crushed past legibility |
| `oversized-h1` | Oversized hero headline | — | A full-sentence headline set at display size ends up dominating the viewport. Set long headlines smaller, or tighten the copy. | long headline set at display size |

### Color & Contrast
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `ai-color-palette` | AI color palette | — | Purple/violet gradients and cyan-on-dark are the most recognizable tells of AI-generated UIs. Choose a distinctive, intentional palette. | AI color palette |
| `cream-palette` | Cream / beige palette | — | A warm cream or beige page background has become the default "tasteful" AI surface. Choose a background from a deliberate palette. | cream and beige as the default surface |
| `gray-on-color` | Gray text on colored background | — | Gray text looks washed out on colored backgrounds. Use a darker shade of the background color instead, or white/near-white for contrast. | gray text on colored background |

### Layout & Space
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `monotonous-spacing` | Monotonous spacing | — | The same spacing value used everywhere — no rhythm, no variation. Use tight groupings for related items and generous separations between sections. | same spacing everywhere |
| `icon-tile-stack` | Icon tile stacked above heading | — | A small rounded-square icon container above a heading is the universal AI feature-card template. Try a side-by-side icon and heading. | large icons with rounded corners above every heading |
| `first-viewport-column-overflow` | One column stretches the first viewport | — | A multi-column opening section lets one column run far past the fold while its sibling fits in a single viewport. | — |
| `edge-flush-cards` | Cards flush against the scroller edge | — | Cards inside a horizontal scroller or tab panel sit flush against the container edge at rest while keeping a gutter on the other side. | — |
| `text-occlusion` | Text occluded by an overlapping element | — | Text is painted under an opaque element or a second text run, so part of it cannot be read. | — |

### Motion
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `bounce-easing` | Bounce or elastic easing | advisory | Bounce and elastic easing feel dated and tacky. Use exponential easing (ease-out-quart/quint/expo) instead. | bounce or elastic easing |
| `pulsing-dot` | Pulsing status dot | — | Small pulsing status dots simulate liveness decoratively. Reserve pulse animation for genuinely live, changing data. | decorative pulsing status dot |
| `blinking-cursor` | Decorative blinking cursor | advisory | A blinking text cursor animated into a hero or landing section simulates typing where no input exists. | — |
| `marquee` | Auto-scrolling marquee | — | Continuously auto-scrolling content demands attention it has not earned and hides half its content at any moment. | auto-scrolling marquee |

### Imagery
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `shape-assembled-illustration` | Shape-assembled illustration | advisory | A large inline SVG that builds a pictorial scene from primitive shapes reads as placeholder clip art. | — |
| `organic-clip-path` | Organic contour drawn as clip-path | — | A clip-path polygon with many arbitrary vertices is CSS approximating a torn edge, blob, or silhouette. Derive an alpha matte from the real image. | geometric masks standing in for organic contours |
| `buried-raster` | Raster buried under a wash or opacity | — | A background image under a near-opaque gradient wash never reaches the screen. Let the material show or remove the file. | a produced material must survive to the screen |
| `broken-image` | Broken or placeholder image | — | `<img>` tags with empty src, missing src, or placeholder values ship as broken-image boxes. | broken image references |

### Copy
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `em-dash-overuse` | Em-dash overuse | advisory | Em-dash saturation in body copy is an AI cadence tell. Advisory only: fires only on saturation (≥8 em-dashes at density near 1 per 500 chars). | no em dashes |
| `marketing-buzzword` | Marketing buzzword | — | Generic SaaS phrases (streamline, empower, supercharge, world-class, enterprise-grade, next-generation, cutting-edge) are instant AI tells. | marketing buzzwords |
| `aphoristic-cadence` | Aphoristic-cadence copy | — | Three or more sections landing on a short rebuttal sentence ("X. No Y." / "X. Just Y.") or manufactured-contrast aphorism reads as AI cadence. | aphoristic cadence |

### Quality
| ID | Name | Severity | Description | Guideline |
|----|------|----------|-------------|-----------|
| `script-error` | Uncaught script error on load | error | A script threw an uncaught exception or failed to parse while the page loaded. Fix the error before judging anything else. | — |
| `content-hidden-at-rest` | Content invisible at rest | error | A large share of the page text sits at opacity 0 or visibility hidden even after every reveal handler had a chance to run. | — |

---

## Category: quality (24 rules)

### Accessibility & HTML Quality
| ID | Name | Severity | Description |
|----|------|----------|-------------|
| `missing-alt-text` | Missing alt text | error | Images without alt attributes or empty alt for decorative images |
| `missing-form-label` | Missing form label | error | Input elements without associated labels |
| `heading-order` | Incorrect heading order | error | Heading levels skip (e.g., h1 → h3) |
| `missing-landmark` | Missing landmark regions | error | Page lacks main, nav, aside, footer landmarks |
| `color-contrast-aa` | Color contrast < 4.5:1 | error | Text fails WCAG AA contrast requirements |
| `color-contrast-aaa` | Color contrast < 7:1 | advisory | Text fails WCAG AAA contrast requirements |
| `focus-visible` | Missing focus indicator | error | Interactive elements without visible focus styles |
| `keyboard-trap` | Keyboard trap | error | Focus cannot escape a component |
| `aria-missing` | Missing ARIA attributes | error | Interactive elements without proper roles/states |
| `reduce-motion` | Missing prefers-reduced-motion | advisory | Animations not respecting reduced motion preference |

### Performance
| ID | Name | Severity | Description |
|----|------|----------|-------------|
| `layout-thrashing` | Layout thrashing | error | Reading/writing layout properties in loops |
| `expensive-animation` | Expensive animations | advisory | Casual layout-property animation, unbounded blur/filter/shadow |
| `missing-lazy-loading` | Missing lazy loading | advisory | Images without loading="lazy" |
| `unoptimized-assets` | Unoptimized assets | advisory | Images not in WebP/AVIF, unminified CSS/JS |
| `will-change-overuse` | will-change overuse | advisory | will-change applied broadly or left on at rest |
| `bundle-size` | Excessive bundle size | advisory | Unnecessary imports, unused dependencies |
| `render-performance` | Unnecessary re-renders | advisory | Missing React.memo, useMemo, useCallback |

### Theming
| ID | Name | Severity | Description |
|----|------|----------|-------------|
| `hardcoded-colors` | Hard-coded colors | error | Colors not using design tokens |
| `broken-dark-mode` | Broken dark mode | error | Missing dark mode variants, poor contrast in dark theme |
| `inconsistent-tokens` | Inconsistent tokens | advisory | Using wrong tokens, mixing token types |
| `theme-switching` | Theme switching issues | advisory | Values that don't update on theme change |

### Responsive Design
| ID | Name | Severity | Description |
|----|------|----------|-------------|
| `fixed-width` | Fixed widths | error | Hard-coded widths that break on mobile |
| `touch-target-size` | Touch targets < 44x44px | error | Interactive elements too small for touch |
| `touch-interaction` | Broken touch interaction | error | Custom sliders/drag surfaces failing under touch |
| `horizontal-scroll` | Horizontal scroll | error | Content overflow on narrow viewports |
| `text-scaling` | Text scaling breaks layout | advisory | Layouts breaking at 200% zoom |
| `missing-breakpoints` | Missing breakpoints | advisory | No mobile/tablet variants |

### Implementation Integrity
| ID | Name | Severity | Description |
|----|------|----------|-------------|
| `design-system-drift` | Design system drift | error | Repeated implementation shortcuts, interchangeable structure |
| `decorative-content` | Misleading decorative content | advisory | Decorative boxes, fake metrics, placeholder content |
| `component-inconsistency` | Component inconsistency | advisory | Same component rendered differently across pages |

---

## Category: african (20 rules)

### Mobile Money (5)
| ID | Name | Skill Section | Guideline |
|----|------|---------------|-----------|
| `african-mobile-money-form` | African mobile money form | Mobile Money | mobile money transaction form |
| `ussd-flow-pattern` | USSD flow pattern | Mobile Money | ussd menu navigation |
| `sim-toolkit-menu` | SIM toolkit menu | Mobile Money | sim toolkit menu |
| `mobile-money-confirmation` | Mobile money confirmation | Mobile Money | mobile money confirmation |
| `agent-cash-in-out` | Agent cash in/out | Mobile Money | agent cash in out |

### FinTech (4)
| ID | Name | Skill Section | Guideline |
|----|------|---------------|-----------|
| `african-fintech-dashboard` | African fintech dashboard | FinTech | fintech dashboard layout |
| `interbank-transfer-flow` | Interbank transfer flow | FinTech | interbank transfer flow |
| `wallet-balance-display` | Wallet balance display | FinTech | wallet balance display |
| `transaction-history-african` | Transaction history African | FinTech | transaction history african |

### Healthcare (4)
| ID | Name | Skill Section | Guideline |
|----|------|---------------|-----------|
| `african-healthcare-appointment` | African healthcare appointment | Healthcare | healthcare appointment |
| `patient-record-sms` | Patient record SMS | Healthcare | patient record sms |
| `community-health-worker-ui` | Community health worker UI | Healthcare | community health worker ui |
| `telemedicine-low-bandwidth` | Telemedicine low bandwidth | Healthcare | telemedicine low bandwidth |

### Agriculture (4)
| ID | Name | Skill Section | Guideline |
|----|------|---------------|-----------|
| `african-agri-market-prices` | African agri market prices | Agriculture | agri market prices |
| `crop-advisory-sms` | Crop advisory SMS | Agriculture | crop advisory sms |
| `farmer-registration-ussd` | Farmer registration USSD | Agriculture | farmer registration ussd |
| `weather-alert-local-language` | Weather alert local language | Agriculture | weather alert local language |

### Government (4)
| ID | Name | Skill Section | Guideline |
|----|------|---------------|-----------|
| `african-gov-service-portal` | African gov service portal | Government | government service portal |
| `digital-id-verification` | Digital ID verification | Government | digital id verification |
| `benefit-disbursement-ui` | Benefit disbursement UI | Government | benefit disbursement ui |
| `citizen-feedback-ussd` | Citizen feedback USSD | Government | citizen feedback ussd |

### General Accessibility (7)
| ID | Name | Skill Section | Guideline |
|----|------|---------------|-----------|
| `low-bandwidth-first` | Low bandwidth first | Accessibility | low bandwidth first |
| `offline-capable-indicator` | Offline capable indicator | Accessibility | offline capable indicator |
| `data-cost-aware` | Data cost aware | Accessibility | data cost aware |
| `multi-language-toggle` | Multi language toggle | Accessibility | multi language toggle |
| `high-contrast-mode` | High contrast mode | Accessibility | high contrast mode |
| `ussd_accessibility` | USSD accessibility | Accessibility | ussd accessibility |
| `voice-over-african-languages` | Voice over African languages | Accessibility | voice over african languages |

---

## African Language Quality Rules (6 detector rules)

These rules are triggered by the `clarify` command and audit dimensions:

| ID | Category | Description | Languages Affected |
|----|----------|-------------|-------------------|
| `african-language-inconsistent` | slop | Mixed terminology within a flow | All 7 languages |
| `african-honorific-missing` | slop | Missing respectful address in formal contexts | Swahili, Luganda, Yoruba, Zulu, Xhosa |
| `african-tone-marks` | slop | Missing tone marks where meaning changes | Yoruba, Hausa |
| `african-noun-class` | slop | Incorrect noun class agreement | Swahili, Luganda, Zulu, Xhosa |
| `african-rtl-layout` | slop | Missing RTL support for Hausa Ajami | Hausa (Arabic script) |
| `african-expansion-overflow` | slop | Text truncated due to insufficient expansion allowance | All 7 languages |

---

## Rule Addition Guidelines

### Adding New Rules

1. **Define the Antipattern** in `crates/foundation/src/registry.rs`:
```rust
Antipattern {
    id: "my-new-rule",
    category: "slop", // or "quality", "african"
    scopes: Some(&["layout"]), // optional
    severity: None, // or Some("advisory"), Some("error")
    name: "My New Rule",
    description: "Detailed description of what this detects and why it matters.",
    skill_section: Some("Layout & Space"),
    skill_guideline: Some("guideline keyword for skill reference"),
},
```

2. **Add Detection Logic** in appropriate crate:
- Visual rules → `crates/foundation/src/rules/`
- HTML quality → `crates/html/src/`
- African patterns → `crates/foundation/src/rules/african/`

3. **Create Skill Guideline** in `skill/reference/{section}.md`

4. **Add Tests**:
- Fixture in `tests/fixtures/{category}/`
- Oracle golden in `tests/oracle/`

5. **Update Documentation**:
- This RULE_GUIDE.md
- ARCHITECTURE_VIZ (new node/edge)
- BENCHMARK (performance test)

### Rule Categories

| Category | Purpose | Severity Defaults |
|----------|---------|-------------------|
| `slop` | AI-generated design tells | `None` (warning) or `advisory` |
| `quality` | Implementation defects | `error` for critical, `advisory` for minor |
| `african` | African market patterns | `None` (warning) or `advisory` |

### Severity Levels

| Severity | Meaning | Usage |
|----------|---------|-------|
| `error` | Blocks task completion, WCAG violation | Script errors, accessibility failures |
| `advisory` | Review recommended, not blocking | Motion preferences, subjective patterns |
| `None` (warning) | Default - fix recommended | Most slop patterns, quality issues |

---

## Rule Testing

```bash
# Test all rules
cargo test -p impeccable-detect

# Test specific category
cargo test -p impeccable-detect slop
cargo test -p impeccable-detect quality
cargo test -p impeccable-detect african

# Test specific rule
cargo test -p impeccable-detect african_mobile_money_form

# Run oracle replay for African rules
bun test tests/oracle/african.test.mjs

# Benchmark rule performance
cargo bench -p impeccable-benchmark -- detector_african
```

---

## Rule Statistics

| Metric | Count |
|--------|-------|
| Total Rules | 82 |
| Slop Rules | 38 |
| Quality Rules | 24 |
| African Rules | 20 |
| Error Severity | 8 |
| Advisory Severity | 12 |
| Warning (default) | 62 |
| With Skill Section | 78 |
| With Skill Guideline | 75 |

---

## Quick Reference Card

### Most Common Slop Patterns (Top 10)
1. `overused-font` - Inter/Roboto/Fraunces overuse
2. `ai-color-palette` - Purple/violet + cyan-on-dark
3. `nested-cards` - Cards inside cards
4. `marketing-buzzword` - Streamline/empower/supercharge
5. `gradient-text` - Decorative gradient headings
6. `cream-palette` - Beige/cream default backgrounds
7. `monotonous-spacing` - Same spacing everywhere
8. `icon-tile-stack` - Icon above every heading
9. `hero-eyebrow-chip` - Tiny label above hero
10. `flat-type-hierarchy` - Weak size hierarchy

### Most Critical Quality Rules (Top 5)
1. `script-error` - Uncaught JS errors (error)
2. `content-hidden-at-rest` - Failed reveals (error)
3. `color-contrast-aa` - WCAG AA failures (error)
4. `missing-form-label` - Unlabeled inputs (error)
5. `focus-visible` - No focus indicators (error)

### African Rules by Domain
| Domain | Rules | Key Patterns |
|--------|-------|--------------|
| Mobile Money | 5 | Forms, USSD, STK, Confirmation, Agent |
| FinTech | 4 | Dashboard, Interbank, Wallet, History |
| Healthcare | 4 | Appointment, SMS Records, CHW, Telemedicine |
| Agriculture | 4 | Market Prices, Advisory, Registration, Weather |
| Government | 4 | Portal, ID Verification, Benefits, Feedback |
| Accessibility | 7 | Bandwidth, Offline, Data Cost, Language, Contrast, USSD, Voice |

---

## Resources

- [Detector Implementation](crates/detect/)
- [Foundation Registry](crates/foundation/src/registry.rs)
- [Skill Guidelines](skill/reference/)
- [Test Fixtures](tests/fixtures/)
- [Oracle Goldens](tests/oracle/)
- [Benchmark Suite](crates/benchmark/)