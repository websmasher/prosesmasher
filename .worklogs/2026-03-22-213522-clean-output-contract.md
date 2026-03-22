# Clean output contract

**Date:** 2026-03-22 21:35
**Scope:** `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs`

## Summary
Cleaned the JSON/text output contract so it reflects user-meaningful information instead of internal implementation details. Readability failures now use clearer wording, `score_x100` is removed from output, scalar failures no longer duplicate themselves in `evidence`, and internal index fields are stripped from evidence payloads.

## Context & Problem
Running the validator on real drafts surfaced several output-contract problems:
- readability messages like “Readability is above threshold” read backwards to humans
- `score_x100` fields were leaking internal scaling mechanics into user-facing JSON
- scalar formula checks were repeating the same blob in both `observed` and `evidence`
- evidence for heuristic matches included internal navigation indices (`section_index`, `paragraph_index`, `sentence_index`) that had no practical meaning because the output has no numbered UI

The user explicitly called these out as product issues rather than implementation trivia. The goal was not to change validation behavior, only to make the output contract sharper and more usable for rewrite loops.

## Decisions Made

### Treat internal numeric scaling as internal only
- **Chose:** Remove `score_x100` from output and normalize max/min thresholds back to human-readable decimal scores.
- **Why:** `39.2 vs 50.0` is obvious; `3917 vs 5000` is not. The integer scaling exists only to support deterministic comparisons and should stay inside the engine.
- **Alternatives considered:**
  - Keep both decimal and x100 forms — rejected because it still leaks irrelevant internals.
  - Change the underlying check implementations first — rejected because the product problem was at the output boundary.

### Remove duplicate scalar evidence
- **Chose:** Suppress `evidence` for scalar readability checks (`flesch-kincaid`, `gunning-fog`, `coleman-liau`, `avg-sentence-length`).
- **Why:** For those checks, `observed` already contains the complete useful payload. Repeating the same data under `evidence` adds noise without improving rewrite guidance.
- **Alternatives considered:**
  - Suppress evidence for every check with one evidence item — rejected because many localizable checks still need that one evidence object.
  - Leave duplication and expect consumers to deduplicate — rejected because the contract itself should be sane.

### Strip internal index fields from evidence
- **Chose:** Remove `section_index`, `paragraph_index`, `sentence_index`, `sentence_index_next`, `sentence_index_third`, and `pattern_type` from output evidence.
- **Why:** These values are only useful in a UI that maps them back to numbered source locations. The current product does not have that, and the offending text itself is already included.
- **Alternatives considered:**
  - Keep indexes for machine consumers — rejected because they are unstable and unhelpful without a separate location model.
  - Remove all metadata from evidence — rejected because matched text and offending sentences are still very useful.

### Fix readability wording to describe real failure semantics
- **Chose:** Use language like:
  - `Readability complexity is above the allowed maximum.`
  - `Readability is below the allowed minimum.`
- **Why:** These read as actual failures instead of sounding like “more readable = bad.”
- **Alternatives considered:**
  - Keep the existing wording and only adjust docs — rejected because the text itself was misleading.

## Architectural Notes
This is an output-layer normalization pass. Validation results from `low-expectations` are left intact; `output.rs` now sanitizes and reshapes them before printing.

That keeps the boundary clean:
- core checks can keep whatever internal fields or scaled values they need
- the CLI output contract exposes only what is helpful to humans and orchestration code

The sanitization logic now does three jobs:
1. drop known internal fields
2. normalize x100 threshold keys to decimal equivalents
3. suppress redundant evidence for readability-style scalar checks

## Information Sources
- Real draft runs on:
  - `/Users/tartakovsky/Projects/steady-parent/apps/landing/content/blog/posts/en/adhd-sensory-asd-signs.mdx`
- Existing output layer:
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`
- Existing tests:
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs`
- Prior worklogs that shaped the current rewrite contract:
  - `.worklogs/2026-03-22-201136-add-preset-cli-and-config-dump.md`
  - `.worklogs/2026-03-22-204817-aggregate-multi-instance-checks.md`
- Verification runs:
  - `cargo test -q -p prosesmasher-adapters-inbound-cli output`
  - `cargo clippy -q --all-targets --all-features`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- check /Users/tartakovsky/Projects/steady-parent/apps/landing/content/blog/posts/en/adhd-sensory-asd-signs.mdx --preset article-en --format json`

## Open Questions / Future Considerations
- Readability checks still include `total_words`, `total_sentences`, and similar arithmetic context in `observed`. That seems useful now, but if rewrite prompts get too noisy we may later want a more compact summary mode.
- `word-repetition` is still conceptually too blunt for long articles. This work intentionally left that logic unchanged and only cleaned the output contract.
- Some remaining failures still use opaque IDs (for example sentence-case IDs that embed heading text). Those may deserve a similar output cleanup later.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — sanitization and message shaping for text/JSON output
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs` — coverage for readability cleanup and evidence stripping
- `.worklogs/2026-03-22-204817-aggregate-multi-instance-checks.md` — prior pass that made `failed` count coherent
- `.worklogs/2026-03-22-203744-tighten-negation-reframe-heuristic.md` — prior pass that refined one of the evidence-heavy heuristic checks

## Next Steps / Continuation Plan
1. Re-run additional real drafts and inspect whether any remaining output fields feel similarly implementation-driven rather than user-driven.
2. Revisit `word-repetition` logic itself so common function words and article-length effects stop creating low-value failures.
3. Consider a failure-focused text mode that hides `PASS` rows entirely unless explicitly requested.
