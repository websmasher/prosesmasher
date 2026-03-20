# Check Structure Plan

## What prosesmasher does

CLI tool that validates prose quality. Takes markdown/text files, parses them into a structured document, runs composable checks against the parsed structure, reports results. One check = one concern. Checks are grouped by category.

## Supported Languages

English, Russian, German, Spanish, Portuguese, French, Indonesian.

All text parsing (word boundaries, sentence segmentation) must work correctly for all supported languages. Term lists and pattern checks are language-specific and loaded from config.

## Tool Stack

| Tool | Crate | What it does | Why this one |
|---|---|---|---|
| **Markdown parser** | `pulldown-cmark` | Markdown → headings, paragraphs, bold/italic spans, links, code blocks | Standard Rust markdown parser, fast, well-maintained |
| **Text segmenter** | `icu_segmenter` (ICU4X) | Locale-aware word and sentence boundaries | Unicode Consortium's official Rust impl. CLDR data for all 7 target languages. Handles abbreviations per-locale ("Dr." doesn't split English sentences, "т.д." doesn't split Russian). |
| **Case mapping** | `icu_casemap` (ICU4X) | Locale-aware upper/lower/title case | Needed for sentence-case heading check. German capitalizes nouns, Turkish has dotless-i, etc. |
| **Syllable counting** | `hyphenation` | Knuth-Liang hyphenation patterns → syllable count | Syllables ≈ hyphenation points + 1. Has TeX patterns for en/de/es/pt/fr/ru. Needed for readability scores. |
| **Readability formulas** | none (hand-rolled) | Flesch-Kincaid, Gunning Fog, Coleman-Liau | Each formula is 2-3 lines of arithmetic. No crate needed. Existing crates are either GPL (`rust_readability`), English-only (`kincaid`), or kitchen sinks. |
| **Term matching** | none (stdlib BTreeSet) | Word/phrase list scanning | BTreeSet for single words, sliding window for phrases. No dependency needed. |
| **Config** | `serde`, `serde_json` | Load check config from JSON | Standard |
| **CLI** | `clap` | Command-line interface | Standard |

**Not used:**
- No regex crate. Pattern detection uses token matching on parsed structures.
- No LLM. All checks are deterministic.
- No `Rust_Grammar` (kitchen sink, English-only).
- No `rust_readability` (GPL-3.0).
- No `unicode-segmentation` (ICU4X is strictly better for multilingual).

## Domain Types (`domain/types`)

```rust
/// A parsed document ready for checking.
pub struct Document {
    pub locale: Locale,
    pub sections: Vec<Section>,
    pub metadata: DocumentMetadata,
}

pub enum Locale {
    En, Ru, De, Es, Pt, Fr, Id,
}

pub struct Section {
    pub heading: Option<Heading>,
    pub blocks: Vec<Block>,
}

pub struct Heading {
    pub level: u8,
    pub text: String,
}

pub enum Block {
    Paragraph(Paragraph),
    List(ListBlock),
    BlockQuote(Vec<Block>),
    CodeBlock(String),
}

pub struct Paragraph {
    pub sentences: Vec<Sentence>,
    pub has_bold: bool,
    pub has_italic: bool,
    pub links: Vec<Link>,
}

pub struct Sentence {
    pub text: String,
    pub words: Vec<String>,   // ICU4X word-segmented
    pub word_count: usize,
    pub syllable_count: usize, // via hyphenation crate
}

pub struct Link {
    pub text: String,
    pub url: String,
}

pub struct ListBlock {
    pub ordered: bool,
    pub items: Vec<String>,
}

pub struct DocumentMetadata {
    pub total_words: usize,
    pub total_sentences: usize,
    pub total_syllables: usize,
    pub heading_counts: HeadingCounts,
    pub bold_count: usize,
    pub italic_count: usize,
    pub paragraph_count: usize,
    pub link_count: usize,
}

pub struct HeadingCounts {
    pub h1: usize,
    pub h2: usize,
    pub h3: usize,
    pub h4_plus: usize,
}

pub struct CheckResult {
    pub id: String,
    pub label: String,
    pub pass: bool,
    pub expected: String,
    pub found: String,
    pub severity: Severity,
}

pub enum Severity {
    Error,
    Warning,
}

pub struct Range {
    pub min: usize,
    pub max: usize,
}
```

## Check Trait

```rust
pub trait Check {
    fn id(&self) -> &str;
    fn label(&self) -> &str;
    fn supported_locales(&self) -> Option<&[Locale]>; // None = all locales
    fn run(&self, doc: &Document, config: &CheckConfig) -> CheckResult;
}
```

Runner calls `supported_locales()` and skips checks that don't match the document's locale.

## Complete Check Inventory

### Group: `terms/` — Term list scanning

All use the same mechanism: ICU4X word-segmented tokens checked against BTreeSet (single words) or sliding window (phrases). Language-independent mechanism, per-locale term lists from config.

| File | Check ID | What it catches | Locale |
|---|---|---|---|
| `banned_words.rs` | `banned-words` | AI writing tells: "actually", "navigate", "leverage", etc. | All (per-locale lists) |
| `banned_phrases.rs` | `banned-phrases` | AI phrase tells: "let's dive in", "here's the thing", etc. | All (per-locale lists) |
| `gendered_terms.rs` | `gendered-terms` | "mama", "mommy", "girly" etc. | All (per-locale lists) |
| `forbidden_terms.rs` | `forbidden-terms` | Business-specific forbidden terms | All (per-locale lists) |
| `race_terms.rs` | `race-terms` | Ethnicity/race descriptors | All (per-locale lists) |
| `hedge_words.rs` | `hedge-stacking` | 2+ hedges in one sentence: "might", "perhaps", "somewhat", "possibly", "potentially" | All (per-locale lists) |
| `simplicity.rs` | `simplicity` | Complex word used when simple one works: "utilize"→"use", "facilitate"→"help". Reports the simpler alternative. | All (per-locale synonym pairs in config) |

### Group: `patterns/` — AI slop / structural anti-patterns

Operate on parsed Sentence and Paragraph structures. Most are English-first with per-locale expansion via config lists.

| File | Check ID | What it catches | Detection method | Locale |
|---|---|---|---|---|
| `em_dashes.rs` | `em-dashes` | U+2014 em-dash characters | `str::contains('\u{2014}')` | All |
| `smart_quotes.rs` | `smart-quotes` | Curly quotes U+201C/U+201D etc. | char scan | All |
| `exclamation_density.rs` | `exclamation-density` | Too many `!` per paragraph | count per paragraph, threshold from config | All |
| `negation_reframe.rs` | `negation-reframe` | "Not X. It's Y." rhetorical move | Two token lists (negation signals + reframe signals). Scan consecutive clause pairs split on `.;but`. Flag if clause A has negation signal and clause B has reframe signal. | En first, expandable per-locale via config signal lists |
| `triple_repeat.rs` | `triple-repeat` | "It's X. It's Y. It's Z." | 3 consecutive sentences starting with same 1-2 words | All (word comparison is language-independent) |
| `fake_timestamps.rs` | `fake-timestamps` | "5:47 PM", "at 3 AM" | State machine on word tokens: digit+colon+digit+AM/PM | En (AM/PM). Other locales could add 24h patterns. |
| `colon_dramatic.rs` | `colon-dramatic` | "And then it hit me: everything changed." | Split sentence on `:`, flag if trailing clause < 6 words and starts lowercase | Most Latin-script languages |
| `llm_openers.rs` | `llm-openers` | "The interesting part is...", "What people miss is..." | `starts_with` on first sentence of section against phrase list from config | Per-locale phrase lists |
| `affirmation_closers.rs` | `affirmation-closers` | "and that's the key", "and that makes all the difference" | `ends_with` on last sentence of section against phrase list from config | Per-locale phrase lists |
| `summative_closer.rs` | `summative-closer` | "And that's what makes this approach so powerful." | Last sentence of section matches summative patterns from config list | Per-locale phrase lists |
| `false_question.rs` | `false-question` | "And isn't that what we all want?" — rhetorical padding questions | Last sentence ends with `?` and matches padding patterns from config | Per-locale pattern lists |
| `humble_bragger.rs` | `humble-bragger` | "In my experience...", "As someone who has...", "Having worked with..." — credentialing frames | `contains` on sentence text against phrase list from config | Per-locale phrase lists |
| `jargon_faker.rs` | `jargon-faker` | "debugging your morning routine", "optimizing for authenticity" — tech verbs in non-tech context | Phrase list: tech verb + non-tech noun combinations from config | Per-locale phrase lists |

### Group: `structure/` — Document-level structural checks

Operate on DocumentMetadata and parsed heading/paragraph structures. All language-independent (thresholds from config).

| File | Check ID | What it catches | Detection method | Locale |
|---|---|---|---|---|
| `word_count.rs` | `word-count` | Total prose too short/long | `metadata.total_words` vs config min/max | All |
| `heading_hierarchy.rs` | `heading-hierarchy` | H1 in body, H4+, skipped levels (H2→H4) | Walk heading list, check transitions | All |
| `heading_counts.rs` | `heading-counts` | H2/H3 count outside range | `metadata.heading_counts` vs config ranges | All |
| `bold_density.rs` | `bold-density` | Too few bold statements for scannability | `metadata.bold_count` vs config minimum | All |
| `paragraph_length.rs` | `paragraph-length` | Paragraph has too many sentences | `paragraph.sentences.len()` vs config max | All |
| `sentence_case.rs` | `sentence-case` | Heading uses Title Case instead of sentence case | ICU4X case mapping: check if non-first, non-proper-noun words are capitalized. Locale-aware (skip for German which capitalizes nouns). | En, Es, Pt, Fr, Id (skip De, Ru) |
| `code_fences.rs` | `code-fences` | Code fences in prose content | pulldown-cmark `CodeBlock` events | All |
| `word_repetition.rs` | `word-repetition` | Same non-stop-word appearing too many times | Word frequency count, flag words above threshold (config). Needs per-locale stop word list. | All (per-locale stop words) |

### Group: `readability/` — Readability metrics

Formulas are 2-3 lines of arithmetic each. Inputs: `metadata.total_words`, `metadata.total_sentences`, `metadata.total_syllables`. Syllable count comes from `hyphenation` crate, computed during parsing.

| File | Check ID | Formula | What it measures | Locale |
|---|---|---|---|---|
| `flesch_kincaid.rs` | `flesch-kincaid` | `206.835 - 1.015×(words/sentences) - 84.6×(syllables/words)` | Reading ease (higher = easier). Target: 60-70 for general audience. | All (`hyphenation` has patterns for en/de/es/pt/fr/ru) |
| `gunning_fog.rs` | `gunning-fog` | `0.4 × ((words/sentences) + 100×(complex_words/words))` | Years of education needed. Complex word = 3+ syllables. Target: < 12. | All |
| `coleman_liau.rs` | `coleman-liau` | `0.0588×L - 0.296×S - 15.8` (L=letters/100words, S=sentences/100words) | Grade level. No syllable counting needed — uses character count. | All |
| `avg_sentence_length.rs` | `avg-sentence-length` | `words / sentences` | Raw average. Target depends on content type. | All |

**Note on Indonesian:** `hyphenation` may not have Indonesian patterns. Fallback: estimate syllables by vowel counting (less accurate but functional). Check availability at implementation time.

## Detection Philosophy

All checks are deterministic. No LLM calls. We maintain lists of known surface forms and match on word tokens. Won't catch every instance of every pattern — that's fine. We expand lists as new patterns slip through in review. **False negatives are acceptable; false positives are not.**

## Multilingual Strategy

**Language-independent mechanism, per-locale data:**

The check engine (clause pair scanning, sliding window matching, word comparison) is the same for all languages. What changes per locale:
- Term lists (banned words, hedge words, LLM openers — different AI slop per language)
- Signal lists (negation signals, reframe signals — different contractions/syntax)
- Segmentation rules (ICU4X handles this automatically from locale)
- Capitalization rules (sentence-case check skips German/Russian)
- Readability formula coefficients (Flesch-Kincaid has language-specific constants)

**Config structure for multilingual:**

```json
{
  "locale": "en",
  "terms": {
    "bannedWords": ["actually", "essentially"],
    "bannedPhrases": ["let's dive in"],
    "hedgeWords": ["might", "perhaps", "somewhat"],
    "simplicityPairs": [["utilize", "use"], ["facilitate", "help"]],
    "genderedTerms": ["mama", "mommy"],
    "negationSignals": ["not", "isn't", "aren't", "forget", "stop thinking of"],
    "reframeSignals": ["it's", "that's", "this is", "what matters is"],
    "llmOpeners": ["the interesting part is", "what people miss is"],
    "affirmationClosers": ["and that's the key", "and that makes all the difference"],
    "summativePatterns": ["and that's what makes", "that's why this"],
    "falseQuestionPatterns": ["isn't that what", "don't we all"]
  },
  "thresholds": {
    "wordCount": { "min": 650, "max": 1000 },
    "h2Count": { "min": 2, "max": 6 },
    "h3Min": 0,
    "boldMin": 3,
    "maxParagraphSentences": 4,
    "maxExclamationsPerParagraph": 1,
    "maxHedgesPerSentence": 1,
    "fleschKincaidMin": 50,
    "gunningFogMax": 14,
    "avgSentenceLengthMax": 25
  }
}
```

Separate config file per locale. CLI takes `--config` to select.

## Ports

```rust
pub trait FileReader {
    fn read_to_string(&self, path: &Path) -> Result<String, ReadError>;
}

pub trait DocumentParser {
    fn parse(&self, markdown: &str, locale: &Locale) -> Result<Document, ParseError>;
}

pub trait ConfigLoader {
    fn load_config(&self, path: &Path) -> Result<CheckConfig, ConfigError>;
}
```

## Adapters

**CLI** (`adapters/inbound/cli`): Clap-based. Composition root.

```
prosesmasher check file.md                        # all checks, default config
prosesmasher check file.md --config substack.json  # custom thresholds + term lists
prosesmasher check file.md --group terms           # only term checks
prosesmasher check file.md --group patterns        # only pattern checks
prosesmasher check file.md --group readability     # only readability scores
prosesmasher check dir/                            # all .md files in directory
```

**FS** (`adapters/outbound/fs`): Implements `FileReader` and `ConfigLoader`.

**Parser** (`adapters/outbound/parser`, new crate): Implements `DocumentParser`.
1. `pulldown-cmark` → markdown events → headings, paragraphs, bold/italic spans, links
2. Per paragraph text → `icu_segmenter::SentenceSegmenter` → sentences
3. Per sentence → `icu_segmenter::WordSegmenter` → words + word count
4. Per sentence → `hyphenation` → syllable count
5. Assemble `Document` with all metadata pre-computed

## Dependencies Summary

| Crate | External deps |
|---|---|
| domain/types | none |
| ports/outbound/traits | domain/types |
| app/core | domain/types, ports |
| adapters/outbound/fs | ports, serde, serde_json |
| adapters/outbound/parser (new) | ports, domain/types, pulldown-cmark, icu_segmenter, icu_casemap, hyphenation |
| adapters/inbound/cli | all + clap |

**Total external dependencies: 6** (pulldown-cmark, icu_segmenter, icu_casemap, hyphenation, serde/serde_json, clap). Plus garde already in workspace.

## File Tree

```
app/core/src/
├── lib.rs
├── check.rs                    # Check trait
├── runner.rs                   # Vec<Box<dyn Check>> → Vec<CheckResult>
│
├── terms/
│   ├── mod.rs
│   ├── banned_words.rs
│   ├── banned_phrases.rs
│   ├── gendered_terms.rs
│   ├── forbidden_terms.rs
│   ├── race_terms.rs
│   ├── hedge_words.rs
│   └── simplicity.rs
│
├── patterns/
│   ├── mod.rs
│   ├── em_dashes.rs
│   ├── smart_quotes.rs
│   ├── exclamation_density.rs
│   ├── negation_reframe.rs
│   ├── triple_repeat.rs
│   ├── fake_timestamps.rs
│   ├── colon_dramatic.rs
│   ├── llm_openers.rs
│   ├── affirmation_closers.rs
│   ├── summative_closer.rs
│   ├── false_question.rs
│   ├── humble_bragger.rs
│   └── jargon_faker.rs
│
├── structure/
│   ├── mod.rs
│   ├── word_count.rs
│   ├── heading_hierarchy.rs
│   ├── heading_counts.rs
│   ├── bold_density.rs
│   ├── paragraph_length.rs
│   ├── sentence_case.rs
│   ├── code_fences.rs
│   └── word_repetition.rs
│
└── readability/
    ├── mod.rs
    ├── flesch_kincaid.rs
    ├── gunning_fog.rs
    ├── coleman_liau.rs
    └── avg_sentence_length.rs
```

**33 checks total** across 4 groups (7 terms + 13 patterns + 8 structure + 4 readability + 1 word-repetition).

## Output Format

```
$ prosesmasher check essay.md --config substack-en.json

essay.md
  FAIL  banned-words         "actually" found (expected: none)
  FAIL  em-dashes            2 found (expected: none)
  FAIL  negation-reframe     1 match: "isn't defiance. It's developmental."
  PASS  word-count           812 words (expected: 650-1000)
  PASS  heading-hierarchy    correct
  PASS  gendered-terms       none found
  PASS  flesch-kincaid       63.2 (expected: >= 50)
  WARN  gunning-fog          13.1 (expected: < 14)
  PASS  simplicity           no violations

9 checks: 6 passed, 3 failed, 1 warning
```

## Implementation Order

1. Domain types (Document, Locale, Sentence with words/syllables, CheckResult, Severity)
2. Check trait + runner in app/core
3. DocumentParser port + pulldown-cmark + ICU4X + hyphenation adapter
4. FileReader + ConfigLoader ports + fs adapter
5. First 3 checks: word-count, em-dashes, banned-words
6. CLI adapter with clap (end-to-end working)
7. Remaining term checks (banned-phrases, gendered, forbidden, race, hedge, simplicity)
8. Pattern checks (negation-reframe, triple-repeat, fake-timestamps, colon-dramatic, llm-openers, affirmation-closers, summative-closer, false-question, smart-quotes, exclamation-density)
9. Structure checks (heading-hierarchy, heading-counts, bold-density, paragraph-length, sentence-case, code-fences)
10. Readability checks (flesch-kincaid, gunning-fog, coleman-liau, avg-sentence-length)
