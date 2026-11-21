Run systematic **technical** quality checks and generate a comprehensive report. Don't fix issues; document them for other commands to address.

This is a code-level audit, not a design critique. Check what's measurable and verifiable in the implementation.

**Web only.** Native platforms (`ios` / `android` / `adaptive`) route to [audit.native.md](audit.native.md) instead; if the project is native, switch to it now.

## African Accessibility Audit (Additional Dimensions)

For projects targeting African markets, run these additional checks alongside the standard 5 dimensions:

### 6. Low-Bandwidth Optimization

**Check for:**
- **Critical path size**: Initial load > 50KB on 2G/3G (target < 30KB)
- **Image optimization**: WebP/AVIF with srcset, appropriate dimensions, lazy loading
- **JavaScript blocking**: No blocking scripts in critical path; defer non-essential JS
- **Service worker**: Cache-first strategy for static assets, background sync for mutations
- **Data cost transparency**: Show estimated data cost for user actions (stream, download, sync)
- **Low-data mode**: Toggle reduces quality, disables auto-play, compresses images

**Score 0-4**: 0=Desktop-only, no bandwidth awareness, 1=Some optimization, >100KB critical path, 2=Partial (<50KB, some gaps), 3=Good (<30KB, SW cache, low-data toggle), 4=Excellent (sub-20KB, full offline, data cost shown)

### 7. Offline-First Capability

**Check for:**
- **Offline indicator**: Persistent online/offline/syncing status visible
- **Local mutation queue**: Writes queued locally with conflict resolution strategy
- **Critical data cached**: Balances, alerts, contacts cached for 24h+ offline access
- **Sync behavior**: Background sync on connectivity, manual retry option, conflict UI
- **Graceful degradation**: Read-only mode when offline, clear messaging

**Score 0-4**: 0=No offline support, 1=Basic caching only, 2=Read-only offline, no queue, 3=Full queue + sync, minor conflicts, 4=Excellent (seamless offline, smart conflict resolution)

### 8. High Contrast & Visual Accessibility

**Check for:**
- **WCAG AAA contrast**: 7:1 for body text, 4.5:1 for large text (toggleable)
- **Forced colors mode**: `prefers-contrast: more` and `forced-colors: active` support
- **Large text scaling**: Layout holds at 200% zoom (WCAG 1.4.4)
- **Reduce motion**: `prefers-reduced-motion` respected, no essential info in motion only
- **Screen reader (African languages)**: ARIA labels in Swahili, Luganda, Amharic, Hausa, Yoruba, Zulu, Xhosa; `lang` attributes per element

**Score 0-4**: 0=No high contrast, breaks at zoom, 1=AA only, no African SR, 2=AAA toggle, partial African SR, 3=Full AAA, African SR labels, 4=Excellent (forced colors, all 7 languages SR tested)

### 9. Data Cost Awareness

**Check for:**
- **Data cost estimates**: Show MB/cost for video, download, sync actions
- **OS data saver respect**: Detect `navigator.connection.saveData` and adapt
- **Compression defaults**: Images compressed, API responses gzipped/brotli
- **Selective sync**: User chooses what to sync (Wi-Fi only default)
- **Usage dashboard**: Show data used this session/month

**Score 0-4**: 0=No data awareness, 1=Basic compression only, 2=Data saver detected, no estimates, 3=Estimates + selective sync, 4=Full dashboard, predictive, respects OS setting

### 10. Multi-Language & RTL Support

**Check for:**
- **Language selector**: Flag + native name + English, persists choice
- **RTL for Arabic-script**: Hausa Ajami uses `dir=rtl`, logical CSS properties, mirrored icons
- **Font fallback**: Amharic (Ge'ez/Fidel), N'Ko, Tifinagh fonts with Latin fallback
- **Expansion allowance**: Containers handle 20-40% text expansion, `text-wrap: balance`
- **Code-switching detection**: Consistent terminology within a flow (detector rule)

**Score 0-4**: 0=English only, 1=Language toggle, no RTL, 2=RTL works, fonts ok, 3=Full 7 languages, expansion handled, 4=Excellent (code-switching clean, all scripts tested)

## Diagnostic Scan

Run comprehensive checks across 5 dimensions. Score each dimension 0-4 using the criteria below.

### 1. Accessibility (A11y)

**Check for**:
- **Contrast issues**: Text contrast ratios < 4.5:1, or < 3:1 for large text (or 7:1 for AAA)
- **Motion sensitivity**: `prefers-reduced-motion` needs an intentional alternative that preserves state change and hierarchy; flag a global `0.01ms` kill that destroys useful feedback, flashing above threshold, and motion that blocks focus, reading, or task completion
- **Missing ARIA**: Interactive elements without proper roles, labels, or states
- **Keyboard navigation**: Missing focus indicators, illogical tab order, keyboard traps
- **Semantic HTML**: Improper heading hierarchy, missing landmarks, divs instead of buttons
- **Alt text**: Missing or poor image descriptions
- **Form issues**: Inputs without labels, poor error messaging, missing required indicators

**Score 0-4**: 0=Inaccessible (fails WCAG A), 1=Major gaps (few ARIA labels, no keyboard nav), 2=Partial (some a11y effort, significant gaps), 3=Good (WCAG AA mostly met, minor gaps), 4=Excellent (WCAG AA fully met, approaches AAA)

### 2. Performance

**Check for**:
- **Layout thrashing**: Reading/writing layout properties in loops
- **Expensive animations**: Casual layout-property animation, unbounded blur/filter/shadow effects, or effects that visibly drop frames
- **Missing optimization**: Images without lazy loading, unoptimized assets
- **will-change overuse**: `will-change` applied broadly or left on at rest (it is a targeted hint for known expensive animations, not a baseline requirement)
- **Bundle size**: Unnecessary imports, unused dependencies
- **Render performance**: Unnecessary re-renders, missing memoization

**Score 0-4**: 0=Severe issues (layout thrash, unoptimized everything), 1=Major problems (no lazy loading, expensive animations), 2=Partial (some optimization, gaps remain), 3=Good (mostly optimized, minor improvements possible), 4=Excellent (fast, lean, well-optimized)

### 3. Theming

**Check for**:
- **Hard-coded colors**: Colors not using design tokens
- **Broken dark mode**: Missing dark mode variants, poor contrast in dark theme
- **Inconsistent tokens**: Using wrong tokens, mixing token types
- **Theme switching issues**: Values that don't update on theme change

**Score 0-4**: 0=No theming (hard-coded everything), 1=Minimal tokens (mostly hard-coded), 2=Partial (tokens exist but inconsistently used), 3=Good (tokens used, minor hard-coded values), 4=Excellent (full token system, dark mode works perfectly)

### 4. Responsive Design

**Check for**:
- **Fixed widths**: Hard-coded widths that break on mobile
- **Touch targets**: Interactive elements < 44x44px. WCAG AA (SC 2.5.8) requires at least 24x24 CSS px unless an exception applies: spacing, an equivalent control, inline text, user-agent styling, or an essential size. Spacing passes when a 24 CSS px circle centered on the target's bounding box overlaps no other target and no other undersized target's circle
- **Broken touch interaction**: Custom sliders, drag surfaces, and scrollable control strips whose primary gesture fails under touch, that swallow page scroll or lose the drag to it, or that stay stuck after an interrupted gesture. Code tells: mouse-only handlers, no `touch-action` on a pointer-event drag surface, drag state that nothing clears on cancel, lost capture, or blur. Exercise the gesture when a browser tool can synthesize touch (a rendered viewport proves layout, not the gesture), then say what produced the evidence (emulated viewport, synthesized touch, which engine, physical device) and what stayed untested
- **Horizontal scroll**: Content overflow on narrow viewports
- **Text scaling**: Layouts that break when text size increases
- **Missing breakpoints**: No mobile/tablet variants

On RTL-declared pages, each of the following is a signal to verify the rendered result, not a defect in itself; flag only where the rendering is actually wrong:
- **Physical directional properties**: `margin-left/right`, `padding-left/right`, `text-align: left/right`, `float: left/right`, `border-left/right` where logical equivalents (`margin-inline-start/end`, `padding-inline-start/end`, `text-align: start/end`, `float: inline-start/end`, `border-inline-start/end`) would be expected. A direction-scoped override like `[dir="rtl"] .x { margin-right:1rem }` is correct only when it also clears the conflicting base property (e.g. `margin-left: 0`); without that, both margins apply
- **Physical positioning on positioned elements**: `left:` or `right:` instead of `inset-inline-start` / `inset-inline-end`. Note: `left:50%; transform:translateX(-50%)` is a direction-agnostic centering pattern that works correctly in both LTR and RTL; replacing `left` with `inset-inline-start` there breaks the centering because `translateX` is physical and does not mirror
- **Directional icons and assets**: arrows, chevrons, and navigation icons that point the wrong direction for the reading order. An RTL-only page may ship assets already drawn correctly without a `[dir="rtl"]` flip; verify the rendered icon rather than assuming a missing `transform: scaleX(-1)` is a defect

**RTL scope**: flag physical directional properties only on pages that declare `dir="rtl"` on the `<html>` or `<body>` element. A `lang` attribute alone does not set direction (browsers take direction from `dir`, never from `lang`), but treat an RTL-script `lang` (`ar`, `fa`, `ur`, `ps`, `ckb`, `he`, `yi`, `ug`, `sd`, `dv`) without an explicit `dir` as a signal worth noting.

**Score 0-4**: 0=Desktop-only (breaks on mobile), 1=Major issues (some breakpoints, many failures), 2=Partial (works on mobile, rough edges), 3=Good (responsive, minor touch target or overflow issues), 4=Excellent (fluid, all viewports, proper touch targets, gestures work under touch). On RTL-declared pages, reduce by 1 for pervasive rendered RTL failures

### 5. Implementation Integrity (CRITICAL)

Run the bundled detector and verify each finding in context. Look for repeated implementation shortcuts, design-system drift, misleading or decorative content, and structure that is interchangeable with an unrelated product. Keep deterministic findings separate from visual judgment and call out false positives.

**Desktop apps** (Electron, NW.js, CEF, WebView2): launch the app with a local DevTools port (`--remote-debugging-port=9222`, or `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222` for WebView2; keep it on 127.0.0.1, never exposed), seed the state to audit, keep the window on screen (a hidden one skips the pixel contrast reads), then run `{{scripts_path}}/impeccable detect --cdp 9222 --target <part of the page URL or title> --json`. It returns the same JSON as a URL scan and leaves the window as found. Take screenshots through the harness's CDP tooling, and leave the window as you found it too. Tauri on macOS (WKWebView) and on Linux (WebKitGTK) expose no CDP: scan the source and say the rendered check did not run.

**Score 0-4**: 0=systemic drift, 1=major repeated failures, 2=several verified issues, 3=minor isolated issues, 4=coherent and intentional

## Generate Report

### Audit Health Score

| # | Dimension | Score | Key Finding |
|---|-----------|-------|-------------|
| 1 | Accessibility | ? | [most critical a11y issue or "--"] |
| 2 | Performance | ? | |
| 3 | Theming | ? | |
| 4 | Responsive Design | ? | |
| 5 | Implementation Integrity | ? | |
| 6 | Low-Bandwidth Optimization | ? | |
| 7 | Offline-First Capability | ? | |
| 8 | High Contrast & Visual Accessibility | ? | |
| 9 | Data Cost Awareness | ? | |
| 10 | Multi-Language & RTL Support | ? | |
| **Total** | | **??/40** | **[Rating band]** |

**Rating bands**: 36-40 Excellent (minor polish), 28-35 Good (address weak dimensions), 20-27 Acceptable (significant work needed), 12-19 Poor (major overhaul), 0-11 Critical (fundamental issues)

### Implementation Integrity Verdict
**Start here.** Pass/fail: does the implementation express a coherent product-specific system? Cite verified evidence and detector findings.

### Executive Summary
- Audit Health Score: **??/40** ([rating band])
- Total issues found (count by severity: P0/P1/P2/P3)
- Top 3-5 critical issues
- Recommended next steps

### Detailed Findings by Severity

Tag every issue with **P0-P3 severity**:
- **P0 Blocking**: Prevents task completion. Fix immediately
- **P1 Major**: Significant difficulty or WCAG AA violation. Fix before release
- **P2 Minor**: Annoyance, workaround exists. Fix in next pass
- **P3 Polish**: Nice-to-fix, no real user impact. Fix if time permits

For each issue, document:
- **[P?] Issue name**
- **Location**: Component, file, line
- **Category**: Accessibility / Performance / Theming / Responsive / Implementation Integrity
- **Impact**: How it affects users
- **WCAG/Standard**: Which standard it violates (if applicable)
- **Recommendation**: How to fix it
- **Suggested command**: Which command to use (prefer: {{available_commands}})

### Patterns & Systemic Issues

Identify recurring problems that indicate systemic gaps rather than one-off mistakes:
- "Hard-coded colors appear in 15+ components, should use design tokens"
- "Touch targets consistently too small (<44px) throughout mobile experience"

### Positive Findings

Note what's working well: good practices to maintain and replicate.

## Recommended Actions

List recommended commands in priority order (P0 first, then P1, then P2):

1. **[P?] `{{command_prefix}}command-name`**: Brief description (specific context from audit findings)
2. **[P?] `{{command_prefix}}command-name`**: Brief description (specific context)

**Rules**: Only recommend commands from: {{available_commands}}. Map findings to the most appropriate command. End with `{{command_prefix}}impeccable polish` as the final step if any fixes were recommended.

After presenting the summary, tell the user:

> You can ask me to run these one at a time, all at once, or in any order you prefer.
>
> Re-run `{{command_prefix}}impeccable audit` after fixes to see your score improve.

Keep P3s few: a long tail of minor issues buries the ones that matter.

**Avoid**:
- Report issues without explaining impact (why does this matter?)
- Provide generic recommendations (be specific and actionable)
- Skip positive findings (celebrate what works)
- Forget to prioritize (everything can't be P0)
- Report false positives without verification

## Save the Report

After the report is delivered, save a copy so the findings outlive this conversation: write it, from the score table through Recommended Actions, to a temp file, then run the write command with the scores as metadata and delete the temp file:

```bash
IMPECCABLE_REPORT_META='{"target":"<user phrasing, or .>","total_score":<n>,"max_score":20,"p0_count":<n>,"p1_count":<n>,"p2_count":<n>,"p3_count":<n>}' \
  {{scripts_path}}/impeccable audit-storage write "<resolved target, or . for the whole project>" <temp-file>
```

The helper prints the path it wrote, which you mention in one line; `REPORTS OFF:` means the project opted out, and you mention nothing. Saving never replaces delivering: the user reads the report in chat first.
