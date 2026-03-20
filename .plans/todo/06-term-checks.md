# Plan 06: Remaining Term Checks

## Goal

Implement the 6 remaining term checks. All follow the same pattern as `banned_words.rs` from Plan 04 — collect words from Document, match against term set from config.

## Prerequisites

- Plan 04 completed (Check trait, runner, banned-words check working)
- low-expectations non-regex methods available (`expect_terms_absent`, `expect_phrases_absent`)

## Checks to implement

Each is one file in `app/core/src/terms/`. Each struct implements `Check`. All return `supported_locales() -> None` (all locales, per-locale data comes from config).

### 1. `banned_phrases.rs` — `BannedPhrasesCheck`

- Collect all words from Document (same as banned-words)
- Use `low_expectations::text::build_phrase_list(&config.terms.banned_phrases)`
- Call `suite.expect_phrases_absent("banned-phrases", &all_words, &phrases)`
- Sliding window matches multi-word phrases

### 2. `gendered_terms.rs` — `GenderedTermsCheck`

- Same mechanism as banned-words but uses `config.terms.gendered_terms`
- `suite.expect_terms_absent("gendered-terms", &all_words, &gendered_set)`

### 3. `forbidden_terms.rs` — `ForbiddenTermsCheck`

- Uses `config.terms.forbidden_terms`
- `suite.expect_terms_absent("forbidden-terms", &all_words, &forbidden_set)`

### 4. `race_terms.rs` — `RaceTermsCheck`

- Uses `config.terms.race_terms`
- `suite.expect_terms_absent("race-terms", &all_words, &race_set)`

### 5. `hedge_words.rs` — `HedgeStackingCheck`

- Different from the others: checks for 2+ hedge words **in a single sentence**, not across the whole document
- Walk each sentence individually
- For each sentence, count how many of `config.terms.hedge_words` appear
- If count >= `config.thresholds.max_hedges_per_sentence` (default 2), flag it
- Report which sentence has the stacking

### 6. `simplicity.rs` — `SimplicityCheck`

- Uses `config.terms.simplicity_pairs` which is `Vec<(String, String)>` — (complex_word, simple_alternative)
- Build a BTreeMap from complex → simple
- Walk all words, check if any match a complex word
- Report: `"utilize" found, use "use" instead`

## Update `terms/mod.rs`

Add all 6 new checks to the `all_checks()` function.

## Tests

For each check, at minimum:
- One test with a match (should fail)
- One test with clean text (should pass)

Specific test cases:
- banned-phrases: "so let's dive in here" → FAIL (catches "let's dive in")
- banned-phrases: "so let's go here" → PASS
- hedge-stacking: "it might perhaps work" (2 hedges in 1 sentence) → FAIL
- hedge-stacking: "it might work. perhaps later." (1 hedge per sentence) → PASS
- simplicity: "we utilize this" → FAIL, report "use 'use' instead"

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
