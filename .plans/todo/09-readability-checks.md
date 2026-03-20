# Plan 09: Readability Checks

## Goal

Implement 4 readability checks. Each is a simple arithmetic formula using pre-computed metadata (total_words, total_sentences, total_syllables).

## Prerequisites

- Plan 04 completed (Check trait, runner working)
- Plan 02 completed (parser computes syllable counts via `hyphenation` crate, stored in `Sentence.syllable_count` and `DocumentMetadata.total_syllables`)

## Checks to implement

Each is one file in `app/core/src/readability/`.

### 1. `flesch_kincaid.rs` — `FleschKincaidCheck`

**Formula:** `206.835 - 1.015 × (words/sentences) - 84.6 × (syllables/words)`

- Higher score = easier to read
- 60-70 = standard / easily understood
- 30-50 = college level
- 0-30 = academic
- Config threshold: `config.thresholds.flesch_kincaid_min` (e.g., 50.0)
- FAIL if score < threshold

**Note:** Flesch-Kincaid coefficients are English-specific. Other languages have adapted versions with different constants (e.g., Flesch Reading Ease for German uses different weights). For V1, use English coefficients for all languages — the relative scores are still meaningful for comparison. Add language-specific coefficients later if needed.

**Edge cases:**
- If total_sentences == 0, skip (no prose to analyze). Don't divide by zero.
- If total_words == 0, skip.
- Use `f64` arithmetic with the `float_cmp` clippy lint in mind — compare with epsilon, not `==`.

`supported_locales() -> None` (syllable counting is already locale-aware via hyphenation)

### 2. `gunning_fog.rs` — `GunningFogCheck`

**Formula:** `0.4 × ((words/sentences) + 100 × (complex_words/words))`

- "Complex word" = 3+ syllables
- Score ≈ years of education needed
- Target: < 12 for general audience
- Config threshold: `config.thresholds.gunning_fog_max` (e.g., 14.0)
- FAIL if score > threshold

**Complex word count:** Walk all sentences, count words where `syllable_count >= 3`. This requires per-word syllable counts, which are in `Sentence.words` + `Sentence.syllable_count`. Wait — `Sentence.syllable_count` is the total for the sentence, not per-word.

Per-word syllable counts are available via `Word.syllable_count` (the `Word` struct was defined in Plan 01, filled by the parser in Plan 02). Complex word count:

```rust
let complex_words = doc.sections.iter()
    .flat_map(|s| &s.blocks)
    .filter_map(|b| if let Block::Paragraph(p) = b { Some(p) } else { None })
    .flat_map(|p| &p.sentences)
    .flat_map(|s| &s.words)
    .filter(|w| w.syllable_count >= 3)
    .count();
```

Total syllables for formulas: `words.iter().map(|w| w.syllable_count).sum::<usize>()` (compute from Document, don't rely on metadata).

`supported_locales() -> None`

### 3. `coleman_liau.rs` — `ColemanLiauCheck`

**Formula:** `0.0588 × L - 0.296 × S - 15.8`
- L = average number of letters per 100 words
- S = average number of sentences per 100 words

**Does NOT need syllable counting.** Uses character (letter) count and sentence count only.

- Compute L: `(total_letters / total_words) * 100`
- Compute S: `(total_sentences / total_words) * 100`
- total_letters = sum of alphabetic characters across all words

No specific config threshold defined yet. Could add `coleman_liau_max` to thresholds. For now, report the score as info/warning.

`supported_locales() -> None`

### 4. `avg_sentence_length.rs` — `AvgSentenceLengthCheck`

**Formula:** `total_words / total_sentences`

- Simpler than readability indices but still useful
- Config threshold: `config.thresholds.avg_sentence_length_max` (e.g., 25 words)
- WARN if average exceeds threshold (not FAIL — it's a style signal)

`supported_locales() -> None`

## Update `readability/mod.rs`

Add all 4 checks to `all_checks()`.

## Arithmetic and float comparison notes

All readability formulas involve `f64` arithmetic. The workspace clippy config denies:
- `arithmetic_side_effects` — use safe division helpers: check denominator != 0 before dividing
- `float_cmp` — never compare floats with `==`. Use `>`, `<`, `>=` for threshold checks (these are NOT affected by `float_cmp` — only `==` and `!=` are). For test assertions, compare with epsilon.
- `as_conversions` — use `f64::from(u32::try_from(count).unwrap_or(u32::MAX))` to convert usize → f64

**Skip behavior:** When `total_sentences == 0` or `total_words == 0`, don't add any expectation to the suite (the check is silently not applicable). This means the runner won't show a PASS or FAIL for that check — it simply won't appear in results.

**Coleman-Liau threshold:** Add `coleman_liau_max: Option<f64>` to `Thresholds` in domain types (Plan 01). Default: `None` (informational only). If threshold is `None`, add the expectation as a warning (report the score but don't fail).

## Tests

- flesch-kincaid: doc with short sentences, simple words → high score (> 60) → PASS
- flesch-kincaid: doc with long sentences, complex words → low score (< 30) → FAIL (if min is 50)
- flesch-kincaid: empty doc (0 sentences) → skip, no crash
- gunning-fog: doc with no 3+ syllable words → low fog → PASS
- gunning-fog: doc with many 3+ syllable words → high fog → FAIL
- coleman-liau: compute and verify against a known text sample
- avg-sentence-length: 100 words / 5 sentences = 20 → PASS (if max 25)
- avg-sentence-length: 100 words / 3 sentences = 33.3 → WARN (if max 25)

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
