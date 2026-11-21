> **Additional context needed**: audience knowledge and emotional state.

Rewrite unclear interface text so users understand what happened, what matters, and what to do next. Preserve factual meaning, product terminology, and brand voice.

## Audit the language

Read the entire interaction path, not isolated strings. Identify:

- ambiguous nouns, verbs, and actions;
- internal jargon or assumed knowledge;
- vague labels, outcomes, and system states;
- missing consequences, recovery, or timing;
- inconsistent terminology and capitalization;
- redundant headings, intros, helper text, and confirmations;
- text that breaks at realistic widths or in translation;
- tone that ignores stress, risk, success, or urgency.

Infer audience and task from product context and surrounding UI. Ask before changing factual claims, legal meaning, or a term that may be domain-specific.

## Set the message hierarchy

For each state, decide:

1. the one fact the user needs now;
2. the action available next;
3. supporting context that changes the decision;
4. the appropriate tone for this moment.

Say each idea once. If the heading already explains the state, the introduction should add new information or disappear.

## Rewrite by function

### Actions and navigation

Use a specific verb and object when the outcome is not already obvious. Labels should describe what will happen, not the gesture used to trigger it. Keep the same noun and verb for the same concept throughout the product.

For destructive actions, name the object and consequence. Prefer undo over confirmation when recovery is safe. When confirmation is necessary, name the action on both the message and button instead of using `Yes`, `No`, `OK`, or `Submit`.

### Forms

Use persistent labels; placeholders are examples, not labels. Put format and eligibility requirements before submission. Explain why information is requested only when it is not obvious. Required and optional treatment should be consistent.

Validation says what needs attention and how to correct it without blaming the user. Keep related instructions near the field and announce errors accessibly.

### Errors and permissions

An actionable error answers:

1. what failed;
2. why, when known and useful;
3. how to recover or what alternative remains.

Do not expose internal codes as the primary message. Do not promise a cause or resolution the system cannot know. Treat privacy, payment, deletion, access loss, and blocked work seriously; warmth is welcome, jokes are not.

### Loading, empty, and success states

Loading text names the real operation and sets an honest expectation when the wait is meaningful. Show determinate progress when available; never invent progress.

An empty state distinguishes first use, no results, filters, permissions, and failure. Explain the state and provide the next useful action.

Success confirms the completed outcome and mentions the next consequence only when it changes what the user should do. Routine success should be brief.

### Help and instructional text

Helper text answers an implicit question instead of restating the control. Use progressive disclosure for uncommon detail. Link text must make sense out of context; icon-only controls need accessible names.

## Voice, accessibility, and localization

Voice stays consistent; tone adapts to the moment. Use plain language without flattening terminology the audience genuinely knows.

- Write complete translatable messages rather than concatenated fragments.
- Keep variables and numbers structured so translators can reorder them.
- Allow expansion instead of abbreviating prematurely.
- Make alt text convey the image's information; use empty alt for decoration.
- Keep screen-reader names aligned with visible labels and outcomes.
- Do not rely on punctuation, color, or iconography to carry the message alone.

### African Language Support (7 Languages)

The `clarify` command supports rewriting UX copy in seven major African languages:

| Language | Region | Script | Key Considerations |
|----------|--------|--------|-------------------|
| **Swahili (Kiswahili)** | East Africa (TZ, KE, UG, RW, BI, CD) | Latin | Noun classes (m/wa, ki/vi, n/n), formal vs. informal address, avoid Anglicisms |
| **Luganda** | Uganda (Central) | Latin | Noun classes (mu/ba, ki/bi, n/n), respectful address (taata/maama), Luganda-English code-switching common |
| **Amharic** | Ethiopia | Ge'ez (Fidel) | Right-to-left not needed, complex verb morphology, honorifics (ato/woizero), Ge'ez numerals optional |
| **Hausa** | West Africa (NG, GH, NE, NG) | Latin (Boko), Arabic (Ajami) | Tone marks optional in UI, gender (m/f), respectful forms (malam/hajiya), RTL for Ajami |
| **Yoruba** | Nigeria, Benin, Togo | Latin | Tone marks (do/re/mi) critical for meaning, respectful address (baba/mama), tonal nouns |
| **Zulu** | Southern Africa (ZA, ZW, MW) | Latin | Noun classes (um/aba, isi/izi, in/izin), click consonants (c, q, x), respectful address (baba/mama) |
| **Xhosa** | South Africa, Zimbabwe | Latin | Noun classes (um/aba, isi/izi, in/izin), click consonants (c, q, x), respectful address (tata/mama) |

**Implementation guidelines for African languages:**

1. **Preserve meaning over literal translation** — idioms, proverbs, and cultural metaphors must be adapted, not translated word-for-word
2. **Respect address forms** — use appropriate honorifics and avoid over-familiar tone in financial/health/government contexts
3. **Handle expansion** — African languages often need 20-40% more characters than English; design for text wrapping
4. **Support mixed scripts** — Hausa Ajami (Arabic script) requires RTL layout; Amharic Ge'ez needs Fidel font stack
5. **Screen reader compatibility** — provide `lang` attributes per element; test with NVDA/Orca in each language
6. **Code-switching awareness** — urban users mix English with local language; detector flags inconsistent terminology within a flow
7. **Numeric formatting** — use local digit groupings (e.g., Indian numbering in East Africa: 1,00,000 vs 100,000)

**Detector integration:** The slop detector includes rules for African language copy quality:
- `african-language-inconsistent` — mixed terminology within a flow
- `african-honorific-missing` — missing respectful address in formal contexts
8|    `african-tone-marks` — missing tone marks in Yoruba/Hausa where meaning changes
9|    `african-noun-class` — incorrect noun class agreement in Swahili/Luganda/Zulu/Xhosa
10|    `african-rtl-layout` — missing RTL support for Hausa Ajami
11|    `african-expansion-overflow` — text truncated due to insufficient expansion allowance

Maintain a short terminology glossary when inconsistency spans the product. Do not vary words for literary effect in an interface.

## Verify

Read the flow in context and test:

- comprehension without hidden product knowledge;
- actionability at errors, empty states, and decision points;
- factual accuracy and consistent terminology;
- scanability at target widths and 200% zoom;
- long names, localization expansion, pluralization, and dynamic values;
- accessible names and announced state changes;
- tone appropriate to consequence and emotional context.

The final copy is as short as it can be without removing meaning or recovery.

When the language reads cleanly, hand off to `{{command_prefix}}impeccable polish` for the final pass.
