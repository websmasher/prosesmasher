# Aggregate multi-instance checks

**Date:** 2026-03-22 20:48
**Scope:** `apps/prosesmasher/crates/app/core/src/quality/flow/paragraph_length.rs`, `apps/prosesmasher/crates/app/core/src/quality/flow/paragraph_length_tests.rs`, `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs`, `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition_tests.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/exclamation_density.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/exclamation_density_tests.rs`, `apps/prosesmasher/crates/app/core/src/quality/lexical/hedge_words.rs`, `apps/prosesmasher/crates/app/core/src/quality/lexical/hedge_words_tests.rs`

## Summary
Converted the noisiest multi-instance checks from per-incident expectations to one aggregated expectation per check. `word-repetition`, `paragraph-length`, `hedge-stacking`, and `exclamation-density` now emit one failing constraint with many evidence items instead of dozens of separate results.

## Context & Problem
After fixing the negation-reframe false positive, the user asked whether the remaining `failed` count semantics made sense. They did not. The system was mixing two different models:
- some checks emitted one failing expectation containing multiple bad matches
- some checks emitted one failing expectation per bad paragraph, word, or sentence

That made `failed` hard to interpret in both text and JSON output. The article run on `co-regulation.md` was the clearest example: even after removing the negation false positive, it still reported 11 failed checks, largely because repeated words and long paragraphs were counted as separate failing expectation rows.

## Decisions Made

### Make `failed` mean failing constraints
- **Chose:** Aggregate the four high-noise multi-instance checks so they each emit one result per check ID.
- **Why:** This makes the top-level `failed` count correspond to “how many constraint families are currently violated,” which is much easier to reason about in a rewrite loop.
- **Alternatives considered:**
  - Keep the hybrid model and only explain it in docs — rejected because the output shape itself was the problem.
  - Aggregate everything indiscriminately — rejected because some checks, like `h2-count` and `h3-count`, are genuinely distinct constraints even though they share a label.

### Keep detailed rewrite targets in `evidence`
- **Chose:** Preserve all offending incidents inside `partial_unexpected_list` / JSON `evidence`.
- **Why:** The rewrite loop still needs the concrete paragraphs, words, or sentences to fix. Aggregation should simplify the constraint layer, not throw away actionable detail.
- **Alternatives considered:**
  - Collapse to only a count — rejected because that would make the rewrite payload weaker.
  - Keep one failure per incident and derive a grouped view later in the output adapter — rejected because the suite statistics would remain misleading.

### Update pass semantics for ignored/no-incident cases
- **Chose:** When these checks are enabled, emit one passing aggregate expectation even if there are zero violations.
- **Why:** This keeps the evaluation surface consistent: the check ran, and it passed.
- **Alternatives considered:**
  - Emit zero expectations when nothing qualifies (for example, all words excluded) — rejected because that makes “check skipped” and “check passed cleanly” harder to distinguish.

## Architectural Notes
This is not an output-only change. The aggregation happens at the check layer so:
- suite statistics become meaningful
- text output gets shorter automatically
- JSON `failed` aligns with “failing constraints”

The new model is:
- `failed` counts failing checks/constraints
- `evidence` counts offending instances within those constraints

That makes the rewrite contract cleaner:
- one failure object per family
- one evidence list per family

The article example became a good sanity check:
- before aggregation: 11 failed checks
- after aggregation: 6 failed checks

The remaining 6 are the real failing constraint families:
- `h2-count`
- `h3-count`
- `heading-hierarchy`
- `paragraph-length`
- `word-count`
- `word-repetition`

## Information Sources
- Real validation run on:
  - `/Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md`
- Existing implementations and tests:
  - `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs`
  - `apps/prosesmasher/crates/app/core/src/quality/flow/paragraph_length.rs`
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/exclamation_density.rs`
  - `apps/prosesmasher/crates/app/core/src/quality/lexical/hedge_words.rs`
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- check /Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md --preset article-en`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- check /Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md --preset article-en --format json`

## Open Questions / Future Considerations
- `heading-counts` is still intentionally split into `h2-count` and `h3-count`. That currently feels right, but it is the main remaining place where one label maps to multiple failure IDs.
- Other checks may still have per-incident result models that deserve the same treatment later if they become noisy in real rewrite loops.
- The text output is much cleaner now, but it may still be worth making text mode explicitly failure-focused in a later pass.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs` — now emits one aggregate repetition result with per-word evidence
- `apps/prosesmasher/crates/app/core/src/quality/flow/paragraph_length.rs` — now emits one aggregate paragraph-length result with per-paragraph evidence
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/exclamation_density.rs` — now emits one aggregate exclamation-density result with per-paragraph evidence
- `apps/prosesmasher/crates/app/core/src/quality/lexical/hedge_words.rs` — now emits one aggregate hedge-stacking result with per-sentence evidence
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition_tests.rs` — updated expectations for aggregate repetition results
- `apps/prosesmasher/crates/app/core/src/quality/flow/paragraph_length_tests.rs` — updated expectations for aggregate paragraph results
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/exclamation_density_tests.rs` — updated expectations for aggregate exclamation results
- `apps/prosesmasher/crates/app/core/src/quality/lexical/hedge_words_tests.rs` — updated expectations for aggregate hedge results
- `.worklogs/2026-03-22-203744-tighten-negation-reframe-heuristic.md` — preceding fix that prompted the failed-count discussion

## Next Steps / Continuation Plan
1. Re-run more real drafts from `steady-parent/content/substack/output/` to see whether any other checks still feel too granular or too noisy in text/JSON output.
2. Decide whether `heading-counts` should remain split into two IDs or be rolled into one aggregate check with H2/H3 evidence.
3. Consider a text-output mode that hides passing checks by default and focuses on the aggregated failures plus evidence summaries.
