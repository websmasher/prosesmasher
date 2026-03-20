# Plan 02: Parser Adapter

## Goal

Implement the markdown parser adapter that converts raw markdown into a `Document`. Uses pulldown-cmark for markdown structure, ICU4X for sentence/word segmentation, and hyphenation for syllable counting.

## Prerequisites

- Plan 01 completed (domain types, port traits, parser crate stub exist)
- `DocumentParser` trait defined in `ports/outbound/traits`
- `Document`, `Section`, `Paragraph`, `Sentence`, `Heading`, `Block`, `Locale` types defined in `domain/types`

## What to do

### 1. Add external deps to parser crate

`crates/adapters/outbound/parser/Cargo.toml`:
```toml
[dependencies]
prosesmasher-domain-types = { path = "../../../domain/types" }
prosesmasher-ports-outbound-traits = { path = "../../../ports/outbound/traits" }
pulldown-cmark = { workspace = true }
icu_segmenter = { workspace = true }
icu_casemap = { workspace = true }
hyphenation = { workspace = true }
```

### 2. Implement `markdown.rs`

Walk pulldown-cmark events and build the document tree:
- Track current heading level → split into `Section`s at H2 boundaries
- Track bold/italic state → set `has_bold`/`has_italic` on paragraphs
- Collect links with text and URL
- Collect list items (ordered/unordered)
- Skip code blocks (record them as `Block::CodeBlock`)
- Accumulate paragraph text for later sentence segmentation

Output: a pre-structured document with paragraph text as raw strings (not yet segmented into sentences).

**Important: `Word` struct lifecycle.** The parser must return complete `Word` objects with `syllable_count` already populated. The segmenter internally calls the syllables module for each word before returning sentences. The syllables module is an internal implementation detail — not exposed in the adapter's public API. Do NOT return `Word` objects with `syllable_count: 0` to be filled later.

### 3. Implement `segmenter.rs`

Given raw paragraph text and a `Locale`:
- Create `SentenceSegmenter` for the locale
- Split paragraph text into sentence strings
- For each sentence, create `WordSegmenter` for the locale
- Split into words, count words
- For each word, call `syllables.rs` to get syllable count
- Return `Vec<Sentence>` with complete `Word` objects (text + syllable_count populated)

### 4. Implement `syllables.rs`

Given a word and a `Locale`:
- Load the appropriate hyphenation dictionary for the locale
- Hyphenate the word → count hyphenation points + 1 = syllable count
- Handle unknown words gracefully (fallback: count vowel clusters)
- Handle Indonesian: if no hyphenation dictionary available, use vowel counting fallback

### 5. Implement `lib.rs` — `MarkdownParser`

Struct that implements `DocumentParser` trait:

```rust
pub struct MarkdownParser;

impl DocumentParser for MarkdownParser {
    fn parse(&self, markdown: &str, locale: &Locale) -> Result<Document, ParseError> {
        // 1. pulldown-cmark → raw sections with paragraph text
        // 2. For each paragraph → segmenter → sentences with words
        // 3. For each sentence → syllables → syllable counts
        // 4. Compute DocumentMetadata (totals from all sections)
        // 5. Return Document
    }
}
```

### 6. Compute `DocumentMetadata`

After all sections are built, walk the tree and compute:
- `total_words` — sum of all sentence word counts
- `total_sentences` — sum of all sentence counts across all paragraphs
- `total_syllables` — sum of all sentence syllable counts
- `heading_counts` — count headings by level
- `bold_count` — count paragraphs where `has_bold == true` (or count individual bold spans if trackable)
- `italic_count` — same for italic
- `paragraph_count` — total paragraphs
- `link_count` — total links

### 7. Tests

Test with sample markdown inputs:

```markdown
## First section

This is a paragraph with **bold** and *italic*.

Another paragraph with a [link](https://example.com).

## Second section

- Item one
- Item two

Short paragraph.
```

Verify:
- Correct number of sections (2)
- Headings have correct level and text
- Paragraphs have correct sentence count and word count
- Bold/italic flags set correctly
- Links extracted with text and URL
- Lists parsed correctly
- Metadata totals are accurate
- Locale-specific segmentation: test with a Russian paragraph, verify word/sentence boundaries differ from English

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## What this plan does NOT do

- No CLI integration (parser is called programmatically in tests)
- No config loading
- No check implementations
