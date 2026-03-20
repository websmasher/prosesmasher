# Plan 04: Check Trait + Runner + First 3 Checks

## Goal

Define the `Check` trait, implement the runner, and build the first 3 checks (word-count, em-dashes, banned-words) as proof of the pattern. Tests use hand-built `Document` structs — no parser dependency.

## Prerequisites

- Plan 01 completed (domain types exist)
- **BLOCKING:** `low-expectations` must have the non-regex methods added BEFORE this plan runs. See `/Users/tartakovsky/Projects/websmasher/low-expectations/.plans/todo/no-regex-update.md`. The methods needed: `expect_terms_absent()`, `expect_phrases_absent()`, `expect_stems_absent()`, plus helpers `build_term_set()`, `build_phrase_list()`, `build_stem_set()`. Without these, this plan will not compile.
- Need to add `low-expectations` as a dependency to `app/core`

## What to do

### 1. Add low-expectations dependency to app/core

Add to `crates/app/core/Cargo.toml`:
```toml
[dependencies]
prosesmasher-domain-types = { path = "../../domain/types" }
low-expectations = { path = "../../../../low-expectations/crates/low-expectations" }
```

Note: exact path depends on where low-expectations repo lives relative to prosesmasher. May need to be a git dependency instead. Check at implementation time.

### 2. Implement `check.rs` — the Check trait

```rust
use prosesmasher_domain_types::{Document, CheckConfig, Locale};
use low_expectations::ExpectationSuite;

pub trait Check {
    /// Unique check ID (e.g., "banned-words", "em-dashes").
    fn id(&self) -> &str;

    /// Human-readable label (e.g., "Banned Words").
    fn label(&self) -> &str;

    /// Which locales this check supports. None = all locales.
    /// Checks returning Some(&[Locale::En]) are skipped for non-English documents.
    fn supported_locales(&self) -> Option<&[Locale]>;

    /// Run the check, adding expectations to the suite.
    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite);
}
```

### 3. Implement `runner.rs`

```rust
pub fn run_checks(
    checks: &[&dyn Check],
    doc: &Document,
    config: &CheckConfig,
) -> SuiteValidationResult {
    let mut suite = ExpectationSuite::new("prosesmasher");

    for check in checks {
        // Skip checks that don't support this document's locale
        if let Some(locales) = check.supported_locales() {
            if !locales.contains(&doc.locale) {
                continue;
            }
        }
        check.run(doc, config, &mut suite);
    }

    suite.into_suite_result()
}
```

### 4. Implement `structure/word_count.rs`

```rust
pub struct WordCountCheck;

impl Check for WordCountCheck {
    fn id(&self) -> &str { "word-count" }
    fn label(&self) -> &str { "Word Count" }
    fn supported_locales(&self) -> Option<&[Locale]> { None } // all locales

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if let Some(ref range) = config.thresholds.word_count {
            suite.expect_value_to_be_between(
                "word-count",
                doc.metadata.total_words as i64,  // use try_from, not as
                range.min as i64,
                range.max as i64,
            ).label("Word Count").checking("total prose words");
        }
    }
}
```

Note: use `i64::try_from()` not `as` casts — `as_conversions` is denied by clippy.

### 5. Implement `patterns/em_dashes.rs`

```rust
pub struct EmDashCheck;

impl Check for EmDashCheck {
    fn id(&self) -> &str { "em-dashes" }
    fn label(&self) -> &str { "No Em-Dashes" }
    fn supported_locales(&self) -> Option<&[Locale]> { None }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        // Walk all sentences, count em-dash characters
        let mut count: usize = 0;
        for section in &doc.sections {
            for block in &section.blocks {
                if let Block::Paragraph(p) = block {
                    for sentence in &p.sentences {
                        count = count.saturating_add(
                            sentence.text.chars().filter(|c| *c == '\u{2014}').count()
                        );
                    }
                }
            }
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        suite.expect_value_to_be_between(
            "em-dashes",
            count_i64,
            0, 0,  // expected: exactly 0
        ).label("No Em-Dashes").checking("em-dash characters");
    }
}
```

### 6. Implement `terms/banned_words.rs`

```rust
pub struct BannedWordsCheck;

impl Check for BannedWordsCheck {
    fn id(&self) -> &str { "banned-words" }
    fn label(&self) -> &str { "Banned Words" }
    fn supported_locales(&self) -> Option<&[Locale]> { None }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        // Collect all words from all sentences
        // Note: Sentence.words is Vec<Word> where Word { text, syllable_count }.
        // Extract text for term matching.
        let all_words: Vec<&str> = doc.sections.iter()
            .flat_map(|s| &s.blocks)
            .filter_map(|b| if let Block::Paragraph(p) = b { Some(p) } else { None })
            .flat_map(|p| &p.sentences)
            .flat_map(|s| s.words.iter().map(|w| w.text.as_str()))
            .collect();

        let banned = low_expectations::text::build_term_set(&config.terms.banned_words);
        suite.expect_terms_absent("banned-words", &all_words, &banned)
            .label("Banned Words")
            .checking("AI writing tells");
    }
}
```

### 7. Wire up group mod.rs files

**`terms/mod.rs`**:
```rust
mod banned_words;
pub use banned_words::BannedWordsCheck;

pub fn all_checks() -> Vec<Box<dyn super::Check>> {
    vec![Box::new(BannedWordsCheck)]
}
```

**`patterns/mod.rs`**:
```rust
mod em_dashes;
pub use em_dashes::EmDashCheck;

pub fn all_checks() -> Vec<Box<dyn super::Check>> {
    vec![Box::new(EmDashCheck)]
}
```

**`structure/mod.rs`**:
```rust
mod word_count;
pub use word_count::WordCountCheck;

pub fn all_checks() -> Vec<Box<dyn super::Check>> {
    vec![Box::new(WordCountCheck)]
}
```

**`readability/mod.rs`** — empty for now:
```rust
pub fn all_checks() -> Vec<Box<dyn super::Check>> {
    vec![]
}
```

**`lib.rs`**:
```rust
pub mod check;
pub mod runner;
pub mod terms;
pub mod patterns;
pub mod structure;
pub mod readability;
```

### 8. Tests

Build `Document` structs by hand (no parser needed):

```rust
fn make_doc(text: &str, word_count: usize) -> Document {
    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence {
                    text: text.to_owned(),
                    words: text.split_whitespace()
                        .map(|w| Word { text: w.to_owned(), syllable_count: 1 })
                        .collect(),
                    word_count,
                }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words: word_count,
            total_sentences: 1,
            ..Default::default()
        },
    }
}
```

Test cases:
- word-count: doc with 500 words, config min 650 → FAIL
- word-count: doc with 800 words, config min 650 max 1000 → PASS
- em-dashes: doc with "hello — world" → FAIL (1 found)
- em-dashes: doc with "hello, world" → PASS
- banned-words: doc with "we actually need this" → FAIL ("actually")
- banned-words: doc with "we need this" → PASS
- runner: locale filtering — En check skipped for Ru document

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## What this plan does NOT do

- No CLI integration
- No parser usage (hand-built Documents)
- No remaining checks (Plans 06-09)
