# AFRICAN_DESIGN_GUIDE.md

# African Design Patterns Reference - Impeccable Africa Enhanced

## Overview

This guide documents the 20 African design patterns added to the impeccable detector registry. These patterns address the unique digital realities of African markets: mobile money dominance, linguistic diversity, connectivity constraints, and cultural interaction patterns.

**Category**: `african` (new category alongside `slop` and `quality`)
**Total Rules**: 20
**Languages Covered**: 7 major African languages + English
**Mobile Money Providers**: 18+ supported

---

## Mobile Money Patterns (5 Rules)

### 1. african-mobile-money-form
**ID**: `african-mobile-money-form`
**Category**: `african`
**Skill Section**: Mobile Money
**Guideline**: `mobile money transaction form`

**Description**: Transaction forms for African mobile money services must show provider branding, local currency, fee transparency, USSD fallback, and data cost estimates.

**Detection Criteria**:
- Missing provider selector/badge (M-Pesa, MTN MoMo, Airtel, Orange, Wave, EcoCash, Paga, Opay, Flutterwave, Chipper Cash, WorldRemit, Paystack, PayPal Africa, Vodacom M-Pesa, Tigo Pesa, Halopesa, EazzyPay, M-Pawa)
- Amount field without local currency (NGN, KES, ZAR, GHS, UGX, XOF, XAF, EGP, MAD, TZS)
- No fee estimate display
- No USSD fallback option
- No data cost indicator

**Correct Pattern**:
```html
<form class="mobile-money-form">
  <select name="provider" data-providers="m-pesa|mtn-momo|airtel|orange|wave|ecocash|paga|opay|flutterwave|chipper-cash|worldremit|paystack|paypal-africa|vodacom-mpesa|tigo-pesa|halopesa|eazzy-pay|m-pawa">
    <option value="m-pesa">M-Pesa 🇰🇪</option>
    <option value="mtn-momo">MTN MoMo 🇺🇬</option>
    <!-- ... -->
  </select>
  <input type="tel" name="recipient" placeholder="07XX XXX XXX" pattern="[0-9]{10}">
  <div class="amount-field">
    <input type="number" name="amount" step="1">
    <span class="currency">KES</span>
  </div>
  <div class="fee-estimate">Fee: KES 12 (est.)</div>
  <div class="data-cost">Data: ~0.5 MB ≈ KES 2</div>
  <button type="submit" data-ussd="*334#">Send via App</button>
  <button type="button" class="ussd-fallback" onclick="window.location='tel:*334#'">USSD *334#</button>
</form>
```

**Anti-pattern Example**:
```html
<!-- ❌ Generic form - no African context -->
<form>
  <input name="email" placeholder="Email">
  <input name="amount" placeholder="Amount (USD)">
  <button>Pay with Card</button>
</form>
```

---

### 2. ussd-flow-pattern
**ID**: `ussd-flow-pattern`
**Category**: `african`
**Skill Section**: Mobile Money
**Guideline**: `ussd menu navigation`

**Description**: USSD flows must follow hierarchical menu patterns with clear breadcrumbs, numeric shortcuts, session timeout handling, and graceful degradation to SMS.

**Detection Criteria**:
- No breadcrumb trail in multi-level menus
- Missing numeric shortcuts (1., 2., 3.)
- No session timeout warning
- No SMS fallback for failed USSD
- Missing "Back" (00) and "Home" (0) options

**Correct Pattern**:
```
Welcome to M-Pesa
1. Send Money
2. Withdraw Cash
3. Buy Airtime
4. Loans & Savings
5. My Account
0. Exit

Reply with number (1-5) or 00 to go back
Session expires in 180 seconds
```

---

### 3. sim-toolkit-menu
**ID**: `sim-toolkit-menu`
**Category**: `african`
**Skill Section**: Mobile Money
**Guideline**: `sim toolkit menu`

**Description**: STK (SIM Toolkit) menus must work within 160-character SMS limits, use proactive commands, and handle terminal responses correctly.

**Detection Criteria**:
- Menu items exceeding 160 characters
- Missing proactive command handling
- No terminal response processing
- Incompatible with 2G SIM browsers

---

### 4. mobile-money-confirmation
**ID**: `mobile-money-confirmation`
**Category**: `african`
**Skill Section**: Mobile Money
**Guideline**: `mobile money confirmation`

**Description**: Confirmation screens must show recipient name/number masking, amount in local currency, fee breakdown, PIN/biometric prompt, and transaction reference.

**Detection Criteria**:
- No recipient verification (masked number/name)
- Amount not in local currency
- Missing fee breakdown
- No PIN/biometric authentication step
- No transaction reference generation

**Correct Pattern**:
```
Confirm Payment
To: John D. (0722***123)
Amount: KES 1,500
Fee: KES 12
Total: KES 1,512

Enter M-Pesa PIN: [****]
Reference: MP261121.1432.A1B2

[Confirm]  [Cancel]
```

---

### 5. agent-cash-in-out
**ID**: `agent-cash-in-out`
**Category**: `african`
**Skill Section**: Mobile Money
**Guideline**: `agent cash in out`

**Description**: Agent cash in/out flows must show agent verification (ID badge, location), float balance, denomination breakdown, receipt generation, and SMS confirmation.

**Detection Criteria**:
- No agent ID/location verification
- Missing float balance display
- No denomination selector
- No printed/SMS receipt
- No transaction reversal option

---

## FinTech Patterns (4 Rules)

### 6. african-fintech-dashboard
**ID**: `african-fintech-dashboard`
**Category**: `african`
**Skill Section**: FinTech
**Guideline**: `fintech dashboard layout`

**Description**: African fintech dashboards must display multi-currency balances, mobile money wallet integration, provider badges, low-data mode toggle, and offline indicators.

**Detection Criteria**:
- Single currency only (USD/EUR)
- No mobile money wallet balances
- Missing provider badges
- No low-data mode toggle
- No offline/cached indicator
- Data usage > 100KB initial load

**Required Elements**:
- 10 African currencies (NGN, KES, ZAR, GHS, UGX, XOF, XAF, EGP, MAD, TZS)
- 12+ mobile money providers
- Real-time FX rates with caching
- Low-data mode (< 30KB)
- Offline balance caching (24h)
- Data cost transparency

---

### 7. interbank-transfer-flow
**ID**: `interbank-transfer-flow`
**Category**: `african`
**Skill Section**: FinTech
**Guideline**: `interbank transfer flow`

**Description**: Interbank transfers must support NIBSS (Nigeria), PesaLink (Kenya), BankservAfrica (South Africa), GHIPSS (Ghana), and show real-time status, fees, and reversal options.

**Detection Criteria**:
- No NIBSS/PesaLink/BankservAfrica integration
- Missing real-time status tracking
- No fee transparency
- No reversal/recall option
- Settlement time not displayed

---

### 8. wallet-balance-display
**ID**: `wallet-balance-display`
**Category**: `african`
**Skill Section**: FinTech
**Guideline**: `wallet balance display`

**Description**: Wallet balances must show consolidated view across mobile money, bank, crypto, and cash with primary currency conversion and expandable details.

**Detection Criteria**:
- Single wallet view only
- No consolidated total
- Missing currency conversion
- No expandable provider details

---

### 9. transaction-history-african
**ID**: `transaction-history-african`
**Category**: `african`
**Skill Section**: FinTech
**Guideline**: `transaction history african`

**Description**: Transaction history must show provider badges, currency tags, fee breakdown, status icons, USSD/SMS reference codes, and export to CSV/PDF.

**Detection Criteria**:
- Generic transaction list
- Missing provider identification
- No fee column
- No USSD reference codes
- No export functionality

---

## Healthcare Patterns (4 Rules)

### 10. african-healthcare-appointment
**ID**: `african-healthcare-appointment`
**Category**: `african`
**Skill Section**: Healthcare
**Guideline**: `healthcare appointment`

**Description**: Appointment booking for African healthcare must support CHW referrals, SMS/USSD reminders, facility tier selection (clinic/hospital/referral), and insurance integration (NHIF, NHIS, etc.).

**Detection Criteria**:
- No CHW referral workflow
- No SMS/USSD reminder option
- Missing facility tier selection
- No NHIF/NHIS integration
- No offline booking queue

---

### 11. patient-record-sms
**ID**: `patient-record-sms`
**Category**: `african`
**Skill Section**: Healthcare
**Guideline**: `patient record sms`

**Description**: Patient records accessible via SMS must use structured formats, support local languages, include facility codes, and provide opt-out mechanisms.

**Detection Criteria**:
- Records only in app/web (no SMS access)
- English only
- No facility codes
- No opt-out mechanism

---

### 12. community-health-worker-ui
**ID**: `community-health-worker-ui`
**Category**: `african`
**Skill Section**: Healthcare
**Guideline**: `community health worker ui`

**Description**: CHW interfaces must work offline-first, sync on connectivity, use icon-heavy navigation for low literacy, support voice input, and have emergency escalation.

**Detection Criteria**:
- Requires constant connectivity
- Text-heavy, no icons
- No voice input
- No emergency button
- No background sync

---

### 13. telemedicine-low-bandwidth
**ID**: `telemedicine-low-bandwidth`
**Category**: `african`
**Skill Section**: Healthcare
**Guideline**: `telemedicine low bandwidth`

**Description**: Telemedicine must offer audio-only mode, async consultation (store-and-forward), image compression, and work on 2G/Edge networks.

**Detection Criteria**:
- Video-only consultation
- No async option
- No image compression
- Requires 3G+/WiFi

---

## Agriculture Patterns (4 Rules)

### 14. african-agri-market-prices
**ID**: `african-agri-market-prices`
**Category**: `african`
**Skill Section**: Agriculture
**Guideline**: `agri market prices`

**Description**: Market price displays must show mandi/market prices, grade/quality indicators, price trends, SMS/USSD access, and local language support.

**Detection Criteria**:
- Single market price only
- No grade/quality info
- No trend visualization
- No SMS/USSD access
- English only

---

### 15. crop-advisory-sms
**ID**: `crop-advisory-sms`
**Category**: `african`
**Skill Section**: Agriculture
**Guideline**: `crop advisory sms`

**Description**: Advisory SMS must be actionable, timed to crop calendar, use local language, include input sourcing info, and have feedback loop.

**Detection Criteria**:
- Generic advice not crop-specific
- Not timed to calendar
- English only
- No input sourcing
- No feedback mechanism

---

### 16. farmer-registration-ussd
**ID**: `farmer-registration-ussd`
**Category**: `african`
**Skill Section**: Agriculture
**Guideline**: `farmer registration ussd`

**Description**: USSD registration must capture geo-location, crop types, land size, ID verification, and link to extension services.

**Detection Criteria**:
- No geo-location capture
- Missing crop/land details
- No ID verification
- No extension service linkage

---

### 17. weather-alert-local-language
**ID**: `weather-alert-local-language`
**Category**: `african`
**Skill Section**: Agriculture
**Guideline**: `weather alert local language`

**Description**: Weather alerts must be in local language, use indigenous indicators, provide actionable advice, and support multiple delivery channels (SMS, radio, app).

**Detection Criteria**:
- English only
- No indigenous knowledge integration
- Generic advice not location-specific
- Single delivery channel

---

## Government Patterns (4 Rules)

### 18. african-gov-service-portal
**ID**: `african-gov-service-portal`
**Category**: `african`
**Skill Section**: Government
**Guideline**: `government service portal`

**Description**: Government portals must provide service catalog, eligibility checker, document checklist, application tracker, multi-channel payment (mobile money/card/cash), certificate download, low-data mode, and WCAG AA accessibility with local language screen readers.

**Detection Criteria**:
- No service catalog
- Missing eligibility checker
- No document checklist
- No application tracking
- Single payment method
- No certificate download
- No low-data mode
- Accessibility < WCAG AA

---

### 19. digital-id-verification
**ID**: `digital-id-verification`
**Category**: `african`
**Skill Section**: Government
**Guideline**: `digital id verification`

**Description**: ID verification must support NIN (Nigeria), Ghana Card, Huduma Namba (Kenya), NIDA (Tanzania), and biometric/OTP verification with offline fallback.

**Detection Criteria**:
- No national ID integration
- No biometric option
- No OTP fallback
- No offline verification

---

### 20. benefit-disbursement-ui
**ID**: `benefit-disbursement-ui`
**Category**: `african`
**Skill Section**: Government
**Guideline**: `benefit disbursement ui`

**Description**: Benefit disbursement must show program, amount, cycle, payment method (mobile money/bank/cash), collection point, status, last paid date, USSD balance check, grievance link, and language toggle.

**Detection Criteria**:
- Missing program details
- No payment method choice
- No collection point info
- No status tracking
- No USSD balance check
- No grievance mechanism
- Single language

---

### 21. citizen-feedback-ussd
**ID**: `citizen-feedback-ussd`
**Category**: `african`
**Skill Section**: Government
**Guideline**: `citizen feedback ussd`

**Description**: USSD feedback must support anonymous reporting, category selection, geo-tagging, reference number generation, and follow-up via SMS.

**Detection Criteria**:
- No anonymity option
- No category selection
- No geo-tagging
- No reference number
- No SMS follow-up

---

## General Accessibility Patterns (7 Rules)

### 22. low-bandwidth-first
**ID**: `low-bandwidth-first`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `low bandwidth first`

**Description**: Critical path must load in < 30KB on 2G/3G. Use WebP/AVIF with srcset, lazy loading, defer non-essential JS, cache-first service worker, and show data cost estimates.

**Thresholds**:
- Excellent: < 20KB critical path, full offline, data cost shown
- Good: < 30KB, SW cache, low-data toggle
- Acceptable: < 50KB, some gaps
- Poor: > 100KB, no bandwidth awareness

---

### 23. offline-capable-indicator
**ID**: `offline-capable-indicator`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `offline capable indicator`

**Description**: Persistent online/offline/syncing status must be visible. Local mutation queue with conflict resolution, critical data cached 24h+, background sync, graceful read-only degradation.

---

### 24. data-cost-aware
**ID**: `data-cost-aware`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `data cost aware`

**Description**: Show estimated data cost for user actions (stream, download, sync). Respect OS data saver (`navigator.connection.saveData`). Compression defaults. Selective sync (Wi-Fi only). Usage dashboard.

---

### 25. multi-language-toggle
**ID**: `multi-language-toggle`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `multi language toggle`

**Description**: Language selector with flag + native name + English, persists choice. RTL for Hausa Ajami. Font fallback for Amharic Ge'ez, N'Ko, Tifinagh. 20-40% expansion allowance. Code-switching detection.

---

### 26. high-contrast-mode
**ID**: `high-contrast-mode`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `high contrast mode`

**Description**: WCAG AAA contrast (7:1 body, 4.5:1 large). Forced colors mode (`prefers-contrast: more`, `forced-colors: active`). Large text scaling (200% zoom). Reduce motion. Screen reader labels in 7 African languages.

---

### 27. ussd-accessibility
**ID**: `ussd_accessibility`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `ussd accessibility`

**Description**: SMS/USSD flows must be accessible: numeric shortcuts announced, session timeout spoken, error messages in local language, DTMF alternative for voice menus.

---

### 28. voice-over-african-languages
**ID**: `voice-over-african-languages`
**Category**: `african`
**Skill Section**: Accessibility
**Guideline**: `voice over african languages`

**Description**: TTS support for Swahili, Luganda, Amharic, Hausa, Yoruba, Zulu, Xhosa. Proper pronunciation of click consonants (Zulu/Xhosa), tone marks (Yoruba/Hausa), noun class agreements.

---

## Implementation Checklist

For each African pattern implemented:

- [ ] Detector rule added to `crates/foundation/src/registry.rs`
- [ ] Skill guideline added to relevant `.md` in `skill/reference/`
- [ ] Test fixture in `tests/fixtures/african/`
- [ ] Oracle golden in `tests/oracle/african/`
- [ ] Documentation in `AFRICAN_DESIGN_GUIDE.md`
- [ ] MCP tool integration (if applicable)
- [ ] Benchmark for performance regression
- [ ] Architecture visualization node/edge

## Testing African Patterns

```bash
# Run African-specific detector tests
cargo test -p impeccable-detect african

# Run clarify language tests
bun test tests/skill-behavior/clarify-african.test.mjs

# Run audit dimension tests
bun test tests/audit-african.test.mjs

# Run benchmark for African rules
cargo bench -p impeccable-benchmark -- detector_african
```

## Resources

- [GSMA Mobile Money Deployment Tracker](https://www.gsma.com/mobilemoneymetrics/)
- [W3C Internationalization Guidelines](https://www.w3.org/International/)
- [WCAG 2.1 AAA](https://www.w3.org/WAI/WCAG21/quickref/)
- [African Union Digital Transformation Strategy](https://au.int/en/documents/20200518/digital-transformation-strategy-africa-2020-2030)