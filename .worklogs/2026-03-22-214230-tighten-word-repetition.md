# Tighten Word Repetition

**Date:** 2026-03-22 21:42
**Scope:** `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs`, `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition_tests.rs`

## Summary
Refined the `word-repetition` heuristic so it detects awkward local repetition within prose paragraphs instead of counting repeated words across an entire document. Also excluded markup-style MDX component paragraphs from the check to avoid noise from prop-heavy blobs like `BlogFAQ`.

## Context & Problem
Real article runs showed that the existing global frequency model was producing low-value failures. It flagged topical words like `adhd` and ordinary glue words spread across long drafts, which made the check read as arbitrary rather than useful. After switching other output to be cleaner and more rewrite-oriented, `word-repetition` stood out as the remaining noisy check.

The second issue surfaced during the redesign: paragraph-local repetition reduced the topical-word noise, but MDX component invocations were still parsed as paragraphs and generated nonsense evidence such as repeated `question` tokens inside `<BlogFAQ ... />`. Those are not prose failures and should not be sent back into the rewrite loop.

## Decisions Made

### Make repetition paragraph-local instead of document-global
- **Chose:** Count repeated words independently within each paragraph and emit evidence only for words that exceed the configured threshold in that paragraph.
- **Why:** Repetition quality problems are usually local. A long article repeating `adhd`, `sensory`, or `child` across sections is topical consistency, not bad prose. Paragraph-local repetition better matches “this paragraph sounds clunky.”
- **Alternatives considered:**
  - Keep document-global counting and just expand the exclusion list — rejected because it still cannot distinguish topical content words from awkward repetition.
  - Add document-length scaling to the threshold — rejected because it reduces noise somewhat but still preserves the wrong global mental model.

### Skip markup-style paragraphs
- **Chose:** Ignore paragraphs whose reconstructed text starts with `<`, which catches MDX/JSX component invocations for this check.
- **Why:** Component prop blobs are not prose, and counting their repeated attribute names or question labels is pure noise.
- **Alternatives considered:**
  - Leave component paragraphs in place and rely on exclusions — rejected because the vocabulary is open-ended and project-specific.
  - Fix this in the parser first — rejected for this change because the immediate issue was localized to `word-repetition` and the parser change would be broader.

## Architectural Notes
The check now walks the document block tree and evaluates each paragraph independently, including blockquoted paragraphs, rather than building one global frequency table. Evidence remains aggregated into a single `word-repetition` failure, but each evidence item is now tied to the paragraph text that contains the local repetition.

This is a heuristic shift, not just an output change: `word-repetition` now models local prose awkwardness rather than global topical density.

## Information Sources
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs` — prior implementation and current runtime behavior
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition_tests.rs` — existing test coverage
- `apps/prosesmasher/crates/domain/types/src/config.rs` — default repetition threshold and exclusions
- Real validation runs on:
  - `/Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md`
  - `/Users/tartakovsky/Projects/steady-parent/apps/landing/content/blog/posts/en/adhd-sensory-asd-signs.mdx`
- Prior related worklogs:
  - `.worklogs/2026-03-22-204817-aggregate-multi-instance-checks.md`
  - `.worklogs/2026-03-22-213522-clean-output-contract.md`

## Open Questions / Future Considerations
- Other checks still see MDX component blobs as prose-like paragraphs, especially `paragraph-length`. That may need either parser-level MDX awareness or per-check filtering for markup-style paragraphs.
- If `word-repetition` still proves noisy in real content, the next likely refinement is semantic filtering of low-information words inside prose paragraphs, but that should be driven by more real failures rather than guessed stop-word expansion.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs` — current paragraph-local repetition heuristic
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition_tests.rs` — regression coverage for local repetition and MDX blob skipping
- `apps/prosesmasher/crates/domain/types/src/config.rs` — repetition threshold and default exclusions
- `.worklogs/2026-03-22-204817-aggregate-multi-instance-checks.md` — when multi-instance checks were aggregated
- `.worklogs/2026-03-22-213522-clean-output-contract.md` — output cleanup that made this remaining noise more obvious

## Next Steps / Continuation Plan
1. Re-run the validator on more real `steady-parent` drafts and note whether `word-repetition` now only catches clearly clunky local phrasing.
2. Audit `paragraph-length` and any other prose checks against MDX-heavy articles; decide whether component paragraphs should be filtered at the parser layer or per-check.
3. If MDX noise shows up in multiple checks, add a shared “markup-like paragraph” predicate in core instead of duplicating this local filter.
