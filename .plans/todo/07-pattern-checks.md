# Plan 07: Pattern Checks

## Goal

Implement all 13 pattern checks. These detect AI writing anti-patterns by analyzing parsed Sentence and Paragraph structures — no regex.

## Prerequisites

- Plan 04 completed (Check trait, runner, em-dashes check working as reference)
- Domain types with `Sentence.words`, `Sentence.text`, `Paragraph.sentences`, `Section.heading`

## Checks to implement

Each is one file in `app/core/src/patterns/`. Each struct implements `Check`.

### 1. `em_dashes.rs` — already done in Plan 04

### 2. `smart_quotes.rs` — `SmartQuotesCheck`

- Scan all sentence text for curly quote characters: U+201C (`"`), U+201D (`"`), U+2018 (`'`), U+2019 (`'`)
- Count total occurrences
- `supported_locales() -> None` (all locales)

### 3. `exclamation_density.rs` — `ExclamationDensityCheck`

- For each paragraph, count `!` characters across all sentences
- If count > `config.thresholds.max_exclamations_per_paragraph`, flag it
- Report which paragraph (by index or first few words)
- `supported_locales() -> None`

### 4. `negation_reframe.rs` — `NegationReframeCheck`

- Load negation signals from `config.terms.negation_signals` (e.g., "not", "isn't", "aren't", "forget")
- Load reframe signals from `config.terms.reframe_signals` (e.g., "it's", "that's", "this is")
- For each paragraph, iterate consecutive sentence pairs (sentence A, sentence B)
- Also split within sentences on clause boundaries: ". ", "; ", " but "
- Check: does clause A contain any negation signal AND clause B contain any reframe signal?
- "Contains" means case-insensitive substring search on the clause text (not word-level — signals like "isn't" may appear as one token or split by ICU4X)
- If yes, flag with the matched text
- `supported_locales()` — depends on config; if signals are empty for a locale, check does nothing gracefully

**Important:** All word iteration must use `word.text` (not `word` directly) since `Sentence.words` is `Vec<Word>` where `Word { text, syllable_count }`.

### 5. `triple_repeat.rs` — `TripleRepeatCheck`

- For each paragraph, iterate sentence triples (A, B, C)
- Extract first 1-2 words of each sentence (lowercased)
- If all three share the same opening, flag it
- Example: "It's fast. It's reliable. It's revolutionary." → first word "it's" repeated 3x
- `supported_locales() -> None` (word comparison works for all languages)

### 6. `fake_timestamps.rs` — `FakeTimestampCheck`

- For each sentence, walk the `words` list with a state machine:
  - If word is a digit string (1-2 digits), check next word for ":" then digits then "AM"/"PM"
  - Also check: word is "at", next word is digit, next is "AM"/"PM"
- Flag any match with the timestamp text
- `supported_locales() -> Some(&[Locale::En])` (AM/PM is English)

### 7. `colon_dramatic.rs` — `ColonDramaticCheck`

- For each sentence, find `:` in the text
- Split on first `:`
- Count words after the colon
- If words_after < 6 AND the first word after colon starts with lowercase → flag it
- Informational colons typically have lists or longer clauses; dramatic colons are short
- `supported_locales() -> None` (colon usage is similar across Latin-script languages)
- Consider: skip if text after colon contains commas (likely a list, not dramatic)

### 8. `llm_openers.rs` — `LlmOpenersCheck`

- For each section, get the first sentence
- Check if it starts with any phrase from `config.terms.llm_openers` (case-insensitive `starts_with`)
- Flag with the matched phrase
- `supported_locales()` — depends on config; per-locale phrase lists

### 9. `affirmation_closers.rs` — `AffirmationClosersCheck`

- For each section, get the last sentence
- Check if it ends with any phrase from `config.terms.affirmation_closers` (case-insensitive `ends_with`)
- Flag with the matched phrase
- `supported_locales()` — depends on config

### 10. `summative_closer.rs` — `SummativeCloserCheck`

- For each section, get the last sentence
- Check if it starts with any phrase from `config.terms.summative_patterns` (case-insensitive `starts_with`)
- These are "And that's what makes..." / "That's why this..." patterns
- Flag with the matched phrase
- `supported_locales()` — depends on config

### 11. `false_question.rs` — `FalseQuestionCheck`

- For each section, get the last sentence
- Check if it ends with `?`
- If yes, check if the sentence text contains any phrase from `config.terms.false_question_patterns`
- Padding questions ("isn't that what we all want?") match; genuine provocative questions don't
- Flag with the matched text
- `supported_locales()` — depends on config

### 12. `humble_bragger.rs` — `HumbleBraggerCheck`

- Walk all sentences
- Check if sentence text contains any phrase from `config.terms.humble_bragger_phrases` (case-insensitive `contains`)
- "In my experience", "As someone who has", "Having worked with"
- Flag with the matched phrase
- `supported_locales()` — depends on config

### 13. `jargon_faker.rs` — `JargonFakerCheck`

- Walk all sentences
- Check if sentence text contains any phrase from `config.terms.jargon_faker_phrases` (case-insensitive `contains`)
- "debugging your", "optimizing for", "iterating on your"
- Flag with the matched phrase
- `supported_locales()` — depends on config

## Update `patterns/mod.rs`

Add all 12 new checks (em-dashes already exists) to `all_checks()`.

## Tests

At minimum per check:
- One match case (should fail/warn)
- One clean case (should pass)

Specific test cases:
- negation-reframe: "This isn't defiance. It's developmental." → FAIL
- negation-reframe: "It works more like a philosophy than a tool." → PASS
- triple-repeat: "It's fast. It's reliable. It's revolutionary." → FAIL
- triple-repeat: "It's fast. The engine purrs. Nothing breaks." → PASS
- fake-timestamps: "At 5:47 PM I realized" → FAIL
- fake-timestamps: "Later that evening I realized" → PASS
- colon-dramatic: "And then it hit me: everything changed." → FAIL
- colon-dramatic: "There are three types: red, blue, and green." → PASS (list, >6 words or has commas)
- llm-openers: "The interesting part is that..." → FAIL
- affirmation-closers: "...and that's the key." → FAIL
- false-question: "And isn't that what we all want?" → FAIL
- false-question: "So who's going to build the alternative?" → PASS (not in pattern list)

Additional test cases for checks not covered above:
- smart-quotes: text with `\u{201C}hello\u{201D}` → FAIL; text with `"hello"` → PASS
- exclamation-density: paragraph with 3 `!`, config max 1 → FAIL; paragraph with 1 `!` → PASS
- summative-closer: section ending with "And that's what makes this approach so powerful." → FAIL; section ending with "The data backs this up." → PASS
- humble-bragger: "In my experience working with startups..." → FAIL; "Startups often struggle with..." → PASS
- jargon-faker: "We need to debug our morning routine" → FAIL; "We need to fix our morning routine" → PASS

## Clippy notes

- All word access: `word.text` not `word` (words are `Vec<Word>`)
- No indexing: use `.get(i)` and handle `Option`, or use iterator `.windows()` / `.zip()`
- No `as` casts: use `i64::try_from()` or `usize::try_from()`
- No `println!`/`eprintln!`: checks add to `ExpectationSuite`, no direct output

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
