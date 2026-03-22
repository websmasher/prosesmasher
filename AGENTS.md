# prosesmasher — Agent Handoff Document

## What This Is

CLI tool that validates prose quality in markdown files. Deterministic AI slop detection — no LLM calls, no regex, all checks are composable and config-driven. Takes a markdown file + a JSON config → parses → runs checks → outputs structured results (text or JSON).

```bash
prosesmasher check article.md --config config.json --format json
prosesmasher check docs/ --group terms --format text
prosesmasher check article.md --check prohibited-terms,em-dashes,word-count
```

## Current Status

**Fully functional.** Canonical config migration is in place, legacy config loading is still supported, and the active public surface is stable. 30 active checks, 541+ tests (all passing, 1 ignored for known colon-dramatic false positive). Every module converged under 4-angle adversarial test attacks.

## Architecture

Hexagonal architecture with strict dependency flow: domain ← ports ← app ← adapters.

```
crates/
├── domain/types/           — Locale, Document tree, CheckConfig, errors (pure, no deps)
├── ports/outbound/traits/  — FileReader, DocumentParser, ConfigLoader trait definitions
├── app/core/               — Check trait, runner, all 34 checks (depends on domain + low-expectations)
└── adapters/
    ├── inbound/cli/        — Clap CLI, composition root, output formatting
    └── outbound/
        ├── fs/             — FsFileReader, FsConfigLoader (serde DTOs + garde validation)
        └── parser/         — MarkdownParser (pulldown-cmark + ICU4X + hyphenation)
```

### Data Flow

```
markdown file → FsFileReader.read_to_string()
             → MarkdownParser.parse(content, locale)
               → pulldown-cmark event walker → Document tree
               → ICU4X SentenceSegmenter + WordSegmenter → Sentences/Words
               → hyphenation crate → syllable counts
             → run_checks(checks, document, config) → SuiteValidationResult
             → output_result(file, result, format) → stdout (text or JSON)
```

### External Dependencies

| Crate | What it does | Why this one |
|---|---|---|
| `pulldown-cmark` | Markdown → event stream | Standard Rust markdown parser |
| `icu_segmenter` | Sentence/word segmentation | Unicode Consortium's ICU4X, handles all 7 locales |
| `hyphenation` | Syllable counting | Knuth-Liang patterns, 6 languages embedded |
| `low-expectations` | Expectation suite (pass/fail results) | Our own GX-inspired validation engine |
| `clap` | CLI argument parsing | Standard |
| `serde` + `serde_json` | JSON config deserialization | Standard |
| `garde` | DTO validation | Runtime validation at boundaries |
| `walkdir` | Recursive directory scanning | Standard |

### Supported Locales

English, Russian, German, French, Spanish, Portuguese, Indonesian. Locale affects:
- Sentence/word segmentation (ICU4X handles automatically)
- Syllable counting (per-locale hyphenation dictionaries)
- Sentence case check (skips German/Russian which have different capitalization rules)
- Fake timestamps check (English-only — AM/PM)
- Lexical policy is configurable per locale; most heuristic anti-slop checks now use library-owned defaults with optional overrides/toggles

## The Active Checks

### Quality: Lexical / Terms (5)

| Check | ID | What it catches |
|---|---|---|
| Prohibited Terms | `prohibited-terms` | Unified prohibited lexicon: words and multi-word phrases |
| Hedge Stacking | `hedge-stacking` | 2+ hedges in one sentence ("might perhaps") |
| Simplicity | `simplicity` | Complex→simple pairs: "utilize"→"use" |
| Required Terms | `required-terms` | ALL configured terms must appear |
| Recommended Terms | `recommended-terms` | At least N from pool must appear (with optional stem matching) |

### Quality: Heuristics / Patterns (13)

| Check | ID | What it catches |
|---|---|---|
| Em-Dashes | `em-dashes` | U+2014 characters (AI slop signal) |
| Smart Quotes | `smart-quotes` | Curly quote characters |
| Exclamation Density | `exclamation-density` | Too many `!` per paragraph |
| Negation-Reframe | `negation-reframe` | "Not X. It's Y." rhetorical pattern |
| Triple Repeat | `triple-repeat` | "It's X. It's Y. It's Z." opener repetition |
| Fake Timestamps | `fake-timestamps` | "5:47 PM" fabricated specificity (English only) |
| Colon Dramatic | `colon-dramatic` | "And then: everything changed." |
| LLM Openers | `llm-openers` | "The interesting part is..." section openers |
| Affirmation Closers | `affirmation-closers` | "and that's the key" section endings |
| Summative Closer | `summative-closer` | "And that's what makes..." patterns |
| False Question | `false-question` | "And isn't that what we all want?" |
| Humble Bragger | `humble-bragger` | "In my experience..." credentialing |
| Jargon Faker | `jargon-faker` | "debugging your morning routine" |

### Document Policy / Structure (8)

| Check | ID | What it checks |
|---|---|---|
| Word Count | `word-count` | Total words within configured range |
| Heading Hierarchy | `heading-hierarchy` | No H1 in body, no H4+, no level skips |
| Heading Counts | `heading-counts` | H2/H3 count within range |
| Bold Density | `bold-density` | Minimum bold paragraphs for scannability |
| Paragraph Length | `paragraph-length` | Max sentences per paragraph |
| Sentence Case | `sentence-case` | Headings use sentence case (not Title Case) |
| Code Fences | `code-fences` | Flags code blocks in prose content |
| Word Repetition | `word-repetition` | Same word appearing too many times |

### Readability (4) — arithmetic formulas on word/sentence/syllable counts

| Check | ID | Formula |
|---|---|---|
| Flesch-Kincaid | `flesch-kincaid` | 206.835 - 1.015×(w/s) - 84.6×(syl/w) |
| Gunning Fog | `gunning-fog` | 0.4×((w/s) + 100×(complex/w)) |
| Coleman-Liau | `coleman-liau` | 0.0588×L - 0.296×S - 15.8 |
| Avg Sentence Length | `avg-sentence-length` | words / sentences |

## Config Format

Canonical shape:

```json
{
  "locale": "en",
  "quality": {
    "lexical": {
      "prohibitedTerms": {
        "defaults": true,
        "add": ["live coaching calls"],
        "remove": ["actually"]
      },
      "prohibitedSubstrings": {
        "defaults": false,
        "add": [],
        "remove": []
      },
      "requiredTerms": ["ownership", "borrowing"],
      "requiredSubstrings": [],
      "recommendedTerms": {
        "terms": ["ownership", "borrowing", "lifetimes", "traits", "async"],
        "minCount": 3,
        "allowInflections": true
      },
      "simplicityPairs": {
        "defaults": true,
        "add": [["utilize", "use"]],
        "remove": []
      }
    },
    "heuristics": {
      "exclamationDensity": { "maxPerParagraph": 1 },
      "hedgeStacking": { "maxPerSentence": 2 },
      "wordRepetition": {
        "max": 5,
        "excludedTerms": {
          "defaults": true,
          "add": [],
          "remove": []
        }
      },
      "paragraphLength": { "maxSentences": 4 },
      "readability": {
        "fleschKincaidMin": 50.0,
        "gunningFogMax": 14.0,
        "avgSentenceLengthMax": 25,
        "colemanLiauMax": 12.5
      }
    }
  },
  "documentPolicy": {
    "wordCount": { "min": 650, "max": 1000 },
    "headingCounts": {
      "h2": { "min": 2, "max": 6 },
      "h3Min": 0
    },
    "boldDensity": { "min": 3 },
    "headingHierarchy": { "enabled": true },
    "sentenceCaseHeadings": { "enabled": true },
    "codeFences": { "allowed": false }
  }
}
```

Legacy `terms` / `thresholds` configs are still accepted by the FS loader and normalized into the canonical domain model. Config uses camelCase (JSON convention). Domain types use snake_case. DTO layer in the FS adapter handles conversion. Serde stays out of domain types — they're pure.

## Test Infrastructure

- **541 tests** across 46 test files
- **Test organization**: R-TEST-09 compliant — tests in `_tests.rs` sibling files, not inline in source
- **Test helpers**: `make_doc`, `make_doc_with_word_count`, `make_doc_in_blockquote`, `make_doc_code_only`, `make_doc_multi_section` in `test_helpers.rs`
- **Fixture files**: 5 adversarial markdown fixtures (~25k words total) for parser integration tests, 8 JSON config fixtures for config loader
- **Adversarial testing**: Every module went through 1-3 rounds of 4-angle attacks (completeness, missing scenarios, pattern parity, false positive audit) until convergence
- **Clippy**: Extremely strict — pedantic + nursery + cargo all denied, plus 20+ individual lint denials (no unwrap, no indexing, no as-casts, no string slices, etc.)

## Known Limitations

1. **Silent-e syllable over-counting**: "fire", "smile" counted as 2 syllables instead of 1. The hyphenation dictionary and vowel-cluster fallback can't distinguish "dictionary knows it's 1" from "dictionary doesn't know the word." Readability scores ~10% inflated.

2. **Colon-dramatic false positives**: Factual label:value patterns like "Time: 3 hours" are flagged as dramatic colons. The heuristic only checks clause length, not intent. Test `#[ignore]`d.

3. **PermissionDenied → ConfigError::NotFound**: `ConfigError` has no Io variant, so permission errors on config files are reported as "not found" with a descriptive message. Known semantic gap.

4. **Dictionary caching**: `Standard::from_embedded()` in syllable counting is called per-word, not cached. Medium performance issue for large documents. Should cache in `MarkdownParser` struct.

5. **Outer list item text lost on nesting**: When a list item contains a nested list, the outer item's text before the inner list is lost because `list_item_text` is not stacked. Known limitation of flat `list_item_text`.

## File Structure

```
apps/prosesmasher/
├── Cargo.toml                          — workspace root
├── clippy.toml                         — strict lint config (guardrail3 generated)
└── crates/
    ├── domain/types/src/
    │   ├── locale.rs                   — Locale enum (7 variants)
    │   ├── document.rs                 — Document, Section, Block, Paragraph, Sentence, Word
    │   ├── metadata.rs                 — DocumentMetadata, HeadingCounts
    │   ├── config.rs                   — CheckConfig, TermLists, TermPool, Thresholds, Range
    │   └── error.rs                    — ReadError, ParseError, ConfigError
    ├── ports/outbound/traits/src/
    │   ├── file_reader.rs              — FileReader trait
    │   ├── document_parser.rs          — DocumentParser trait
    │   └── config_loader.rs            — ConfigLoader trait
    ├── app/core/src/
    │   ├── check.rs                    — Check trait + BoxedCheck type alias
    │   ├── runner.rs                   — run_checks() orchestrator
    │   ├── test_helpers.rs             — shared test document builders
    │   ├── terms/                      — 9 term checks
    │   ├── patterns/                   — 13 pattern checks
    │   ├── structure/                  — 8 structure checks
    │   └── readability/                — 4 readability checks
    └── adapters/
        ├── inbound/cli/src/
        │   ├── args.rs                 — Clap argument parsing
        │   ├── checks.rs              — Check collection + filtering
        │   ├── output.rs              — Text + JSON output formatting
        │   ├── lib.rs                 — collect_files + module exports
        │   └── main.rs                — Composition root
        ├── outbound/fs/src/
        │   ├── file_reader.rs         — FsFileReader
        │   ├── config_dto.rs          — Serde DTOs + garde validation + domain conversion
        │   └── config_loader.rs       — FsConfigLoader
        └── outbound/parser/src/
            ├── syllables.rs           — Hyphenation-based syllable counting
            ├── segmenter.rs           — ICU4X sentence/word segmentation
            ├── markdown.rs            — pulldown-cmark → Document tree
            └── lib.rs                 — MarkdownParser (DocumentParser impl)
```

## Next Steps

Potential improvements not yet implemented:

- **Colored terminal output** — PASS in green, FAIL in red
- **Default config discovery** — auto-find `.prosesmasher.json` in cwd/parent dirs
- **`prosesmasher init`** — generate a starter config
- **CI integration** — GitHub Action wrapper
- **Dictionary caching** — load hyphenation dictionaries once per locale, not per word
- **Fix colon-dramatic** — improve heuristic to skip factual label:value patterns
- **Publishing** — crates.io / binary releases
- **`walkdir` error handling** — warn on permission errors instead of silently skipping
- **Additional checks** — more AI slop patterns as they're discovered in the wild
