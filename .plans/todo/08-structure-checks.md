# Plan 08: Structure Checks

## Goal

Implement all 8 structure checks. These operate on `DocumentMetadata` and the heading/paragraph tree — no text content analysis needed.

## Prerequisites

- Plan 04 completed (Check trait, runner working)
- Domain types with `DocumentMetadata`, `HeadingCounts`, `Section`, `Heading`, `Paragraph`

## Checks to implement

Each is one file in `app/core/src/structure/`.

### 1. `word_count.rs` — already done in Plan 04

### 2. `heading_hierarchy.rs` — `HeadingHierarchyCheck`

Walk all headings in document order. Check:
- No H1 headings (H1 is the page title, not in body prose)
- No H4+ headings (only H2/H3 allowed)
- No skipped levels: H2 → H4 is invalid, H2 → H3 is valid, H3 → H2 is valid (going back up)

Implementation: collect all heading levels in order. For each consecutive pair, if `next.level > prev.level + 1`, that's a skip. Also check absolute levels.

Report: which heading violated and why (e.g., "H4 found: 'Details about X'", "Skipped level: H2 → H4 at 'Deep subsection'")

`supported_locales() -> None`

### 3. `heading_counts.rs` — `HeadingCountsCheck`

- Check `metadata.heading_counts.h2` against `config.thresholds.h2_count` range
- Check `metadata.heading_counts.h3` against `config.thresholds.h3_min`
- Two separate expectations in the suite (one for H2 count, one for H3 count)

`supported_locales() -> None`

### 4. `bold_density.rs` — `BoldDensityCheck`

- Check `metadata.bold_count` against `config.thresholds.bold_min`
- Use `expect_value_to_be_at_least`

`supported_locales() -> None`

### 5. `paragraph_length.rs` — `ParagraphLengthCheck`

- Walk all paragraphs
- For each, check `paragraph.sentences.len()` against `config.thresholds.max_paragraph_sentences`
- If any paragraph exceeds, flag it with sentence count and first few words for identification

`supported_locales() -> None`

### 6. `sentence_case.rs` — `SentenceCaseCheck`

- Walk all headings
- For each heading, split text into words
- Skip first word (always capitalized)
- For remaining words: if a word starts with an uppercase letter, flag it as potential Title Case
- Need to handle exceptions: proper nouns, acronyms (all-caps words like "API"), words after colons
- Heuristic: flag if 3+ non-first words are capitalized (avoids false positives on single proper nouns)
- Use ICU4X `icu_casemap` for locale-aware case checking

Algorithm:
```
for each heading:
  words = heading.text.split_whitespace()
  skip first word (always capitalized in sentence case)
  skip all-uppercase words (acronyms like "API", "AWS")
  for each remaining word:
    if word.chars().next() is uppercase:
      capitalized_count += 1
  if capitalized_count >= 3:
    flag as Title Case
```

ICU4X `icu_casemap::CaseMapper` usage: construct once, use `.lowercase()` to get canonical lowercase form, compare against original. If they differ and the word isn't all-caps, it's capitalized.

`supported_locales() -> Some(&[Locale::En, Locale::Es, Locale::Pt, Locale::Fr, Locale::Id])`
Skip German (capitalizes nouns) and Russian (different capitalization rules).

### 7. `code_fences.rs` — `CodeFencesCheck`

- Walk all blocks in the document
- Count `Block::CodeBlock` occurrences
- If count > 0, flag (prose content shouldn't have code fences)

`supported_locales() -> None`

### 8. `word_repetition.rs` — `WordRepetitionCheck`

- Collect all words from all sentences (lowercased)
- Build frequency map: `BTreeMap<String, usize>`
- Filter out stop words from `config.terms.stop_words`
- Filter out short words (< 4 chars) to avoid flagging "the", "and", etc. even if not in stop list
- If any word appears more than `config.thresholds.word_repetition_max` times, flag it
- Report: "word 'actually' appears 7 times (max: 5)"
- Severity: Warning (repetition is a style signal, not always wrong)

`supported_locales() -> None` (per-locale stop word lists in config)

## Clippy and type notes

- `Sentence.words` is `Vec<Word>` where `Word { text: String, syllable_count: usize }`. Always access `word.text` for the string, `word.syllable_count` for syllables.
- No indexing (`vec[i]`): use `.get(i)` or iterators.
- No `as` casts: use `i64::try_from()`.
- No `println!`: checks add to `ExpectationSuite`.
- When a config threshold is `None`, skip the check (don't add any expectation to the suite).

## Update `structure/mod.rs`

Add all 7 new checks (word-count already exists) to `all_checks()`.

## Tests

- heading-hierarchy: doc with H1 → FAIL
- heading-hierarchy: doc with H2 → H4 skip → FAIL
- heading-hierarchy: doc with H2 → H3 → H2 (going back up) → PASS
- heading-counts: doc with 1 H2, config min 2 → FAIL
- bold-density: doc with 1 bold, config min 3 → FAIL
- paragraph-length: paragraph with 6 sentences, config max 4 → FAIL
- sentence-case: heading "Why Saying Nothing Is Bad" → FAIL (3 capitalized non-first words)
- sentence-case: heading "Why saying nothing is bad" → PASS
- sentence-case: heading "Working with the AWS API" → PASS (proper nouns/acronyms)
- code-fences: doc with a CodeBlock → FAIL
- word-repetition: "actually" appears 7 times, max 5 → WARN

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
