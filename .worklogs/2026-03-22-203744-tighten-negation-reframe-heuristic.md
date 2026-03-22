# Tighten negation-reframe heuristic

**Date:** 2026-03-22 20:37
**Scope:** `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs`

## Summary
Reworked the negation-reframe check so it detects corrective contrast rather than any adjacent negation-plus-reframe tokens. The new heuristic still catches short rhetorical relabeling and inline `X, not Y` constructions, but it no longer flags narrative action negation like “I could not fix the banana.”

## Context & Problem
Running `prosesmasher` against a real draft in `steady-parent/content/substack/output/co-regulation.md` exposed a clear false positive:
- `I could not fix the banana.`
- `My second instinct was to explain that bananas sometimes break and this is fine...`

The old implementation matched this because it simply looked for:
- any negation signal in sentence A (`not`)
- any reframe signal in sentence B (`this is`)

That was too broad. The user correctly pointed out that the real target is not generic negation; it is corrective `X not Y` rhetoric. The heuristic needed to distinguish “relabeling/contrast” from “narration/action failure.”

## Decisions Made

### Detect corrective contrast, not generic negation
- **Chose:** Reframe the heuristic around two target shapes:
  - inline corrective contrast such as `X, not Y`
  - short adjacent relabeling such as `This isn't defiance. It's developmental.`
- **Why:** Those are the actual rhetorical forms that read as AI-slop or sloganized explanatory framing.
- **Alternatives considered:**
  - Keep token matching but narrow the signal lists — rejected because the underlying problem was structural, not just vocabulary breadth.
  - Remove the check entirely — rejected because the rhetorical pattern is still useful and prominent when detected correctly.

### Exclude action negation explicitly
- **Chose:** Add a blocklist of action-negation phrases such as `could not`, `did not`, `cannot`, `won't`, etc.
- **Why:** These forms are common in narration, process descriptions, and factual statements. They are not corrective relabeling by themselves.
- **Alternatives considered:**
  - Rely only on sentence length limits — rejected because short factual negation would still slip through.
  - Ban first-person subjects like `I` / `we` — rejected because corrective contrast can still appear in first person.

### Allow short nominal relabels as the second half
- **Chose:** Permit adjacent patterns where the second sentence is either an affirmative copular start (`It's`, `This is`, `That's`) or a very short nominal label.
- **Why:** Real corrective reframes often compress the second half into a short label instead of a full clause.
- **Alternatives considered:**
  - Require a copular starter in every second sentence — rejected because it would miss very short compressed restatements.

### Add regression coverage for both positive and negative cases
- **Chose:** Add tests for:
  - canonical adjacent corrective reframe
  - inline `X, not Y`
  - banana-style narration false positive
- **Why:** This heuristic is fragile by nature. The test set needs to encode both what to catch and what not to catch.
- **Alternatives considered:**
  - Only add the banana regression — rejected because it would protect against one false positive while leaving the intended target shape underspecified.

## Architectural Notes
This remains a heuristic, not a parser-level semantic analysis. The implementation still works locally on sentence text, but it now does so with a structural bias:
- short corrective contrast
- copular/classificatory framing
- explicit rejection of action narration

That is a better fit for `prosesmasher`’s role: deterministic prose anti-pattern detection without pretending to do full semantic understanding.

The check is still English-first. The new logic intentionally restricts itself to the built-in English rhetorical shape rather than trying to generalize across locales without evidence.

## Information Sources
- The real failing draft:
  - `/Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md`
- Existing heuristic implementation:
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs`
- Existing tests:
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs`
- Verification runs:
  - `cargo test -q -p prosesmasher-app-core negation_reframe`
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- check /Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md --preset article-en --format json`

## Open Questions / Future Considerations
- The current heuristic now handles corrective contrast better, but it still does not model shared referents explicitly. If more subtle false positives appear, coreference-lite checks may be the next useful refinement.
- Inline `X, not Y` is intentionally narrow and copular-biased. If real prose data shows more valid corrective shapes, the inline detector may need additional patterns.
- If this check becomes too noisy again, renaming it publicly from “Negation-Reframe Pattern” to something closer to “Corrective Contrast” may better match its actual semantics.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs` — the heuristic logic and evidence payload
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs` — canonical positives and regressions
- `/Users/tartakovsky/Projects/steady-parent/content/substack/output/co-regulation.md` — real-world draft that exposed the banana false positive
- `.worklogs/2026-03-22-201136-add-preset-cli-and-config-dump.md` — immediately preceding CLI work that made this real-world validation easy to run

## Next Steps / Continuation Plan
1. Re-run the validator on additional real drafts from `steady-parent/content/substack/output/` to see whether the narrower heuristic still catches real corrective slop patterns in the wild.
2. If more false positives cluster around specific constructions, add them as regression tests before broadening the matcher.
3. Consider whether the check label should be renamed from `Negation-Reframe Pattern` to a more semantically accurate phrase such as “Corrective Contrast.”
