# Release 0.1.5 Corrective Expression Branch

**Date:** 2026-03-23 18:01
**Scope:** `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs`, workspace versioning and changelog

## Summary
This release adds a narrow new `negation-reframe` branch for internal-state-to-expression corrective pairs such as `don't stop having feelings` followed by `they stop showing them`. It ships as `0.1.5` after verifying that the target fixture is caught while a structurally similar ordinary behavior pair is not.

## Context & Problem
The previous release correctly handled copular corrective patterns, same-root framing-verb pairs, and infinitive corrective pairs, but still missed a specific slop-style move in `ignoring-tantrums.md`:

- `Kids who learn that crying gets no response don't stop having feelings.`
- `They stop showing them.`

This is a real corrective reinterpretation, but it is neither:
- copular `not X / it's Y`
- same-root framing verb `does not mean / it means`
- infinitive `Not to X. To Y.`

The risk was that a broad structural `don't X. They Y.` matcher would immediately overfire on normal prose.

## Decisions Made

### Added a lexically constrained `don't x -> they y` branch
- **Chose:** detect the pattern only when sentence A combines `don't stop` with internal-state nouns and sentence B is a short pronoun-led outward-expression sentence.
- **Why:** this catches the actual slop-style correction without opening the detector to generic two-sentence behavior descriptions.
- **Alternatives considered:**
  - Generic `don't X. They Y.` matching — rejected because it would obviously catch normal prose.
  - Folding the pattern into the same-root framing-verb branch — rejected because the verbs do not share a framing root and the semantics are different.

### Kept the new branch semantic-by-lexicon, not purely structural
- **Chose:** require internal-state terms like `feelings`, `emotion`, `distress`, `fear`, `anger`, `pain`, plus outward-expression phrases like `stop showing`, `hide it`, `suppress it`.
- **Why:** the semantic guardrails are what prevent obvious false positives like road directions or ordinary action descriptions.
- **Alternatives considered:**
  - Subject/pronoun and sentence-length checks only — rejected because that would not be discriminative enough.
  - Broad emotional vocabulary — rejected because it would be harder to reason about and more likely to sprawl.

## Architectural Notes
This work stays inside the existing `negation-reframe` check instead of creating another top-level heuristic. That keeps the public check surface stable while continuing the existing design pattern: one user-facing corrective-rhetoric family with multiple narrowly-scoped internal matcher branches.

## Information Sources
- Fixture under review:
  - `fixtures/ignoring-tantrums.md`
- Existing corrective-pattern implementation and tests:
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs`
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs`
- Prior heuristic release context:
  - `.worklogs/2026-03-23-174033-release-0-1-4-heuristic-tightening.md`

## Open Questions / Future Considerations
- If more examples of this family appear, the internal-state and expression lexicons may need extension, but they should stay intentionally small.
- Inline `X, not Y` remains broader than the newer adjacent corrective branches and may still deserve future narrowing if engineering prose false positives accumulate.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs` — full corrective-rhetoric matcher family
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs` — regression coverage including the new `don't x -> they y` branch
- `apps/prosesmasher/CHANGELOG.md` — released surface summary
- `.worklogs/2026-03-23-174033-release-0-1-4-heuristic-tightening.md` — prior heuristic release decisions and boundaries

## Next Steps / Continuation Plan
1. Publish `0.1.5` across the full crate graph and reinstall the crates.io binary.
2. Verify the installed binary catches `ignoring-tantrums.md` with the new `don't x -> they y` branch.
3. Continue hardening `negation-reframe` only through similarly narrow semantic branches, not broad structural widening.
