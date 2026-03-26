# Tighten Boilerplate Framing Multi-Hit Detection

**Date:** 2026-03-26 13:33
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing_tests/synthetic.rs`, `fixtures/medicaloutline/what-cancers-cannot-be-cured.expected.general-en.json`

## Summary
Tightened `boilerplate-framing` so it can emit multiple findings from a single sentence when that sentence genuinely contains multiple framing moves. This unlocked the missed Medical Outline article `what-cancers-cannot-be-cured.md`, which now fails the rule additively without disturbing earlier fixture expectations.

## Context & Problem
After the first `boilerplate-framing` rollout, manual review showed a real miss: `what-cancers-cannot-be-cured.md` contains both `there are certain types ...` and `some examples ... include` setup scaffolding, but the rule did not fail the file. The root cause was structural, not just lexical: the implementation returned at most one framing match per sentence. That made any same-sentence double setup invisible to the accumulative threshold.

## Decisions Made

### Allow Multiple Framing Hits Per Sentence
- **Chose:** change the rule to collect a vector of matches from each sentence instead of stopping at the first match.
- **Why:** one sentence can contain multiple distinct rhetorical setup moves, and the accumulative rule should count each distinct move.
- **Alternatives considered:**
  - Keep one-hit-per-sentence and broaden the matcher until other sentences tripped the threshold — rejected because it would distort the heuristic instead of fixing the real counting bug.
  - Lower the threshold to force one hit to fail — rejected because that would make the rule too noisy overall.

### Add a Separate Existence Frame
- **Chose:** add `existence-frame` for `there are certain/common <category>` structures.
- **Why:** this is a genuine setup frame that is rhetorically different from `some examples ... include`.
- **Alternatives considered:**
  - Fold it into `enumeration-preface` — rejected because the evidence would become less precise and the family boundary blurrier.

### Update Only the Newly Affected Fixture
- **Chose:** add `boilerplate-framing` only to `fixtures/medicaloutline/what-cancers-cannot-be-cured.expected.general-en.json`.
- **Why:** that is the one real fixture whose failure status changed under the corrected matcher.
- **Alternatives considered:**
  - Rewrite the whole Medical Outline corpus — rejected because fixture sidecars are supposed to be additive regression floors.

## Architectural Notes
The important architectural correction here is that accumulative rhetorical rules cannot assume “one sentence = one signal.” `boilerplate-framing` now owns its own traversal and evidence collection so it can emit multiple evidence items for a single sentence. That keeps the more specialized multi-hit behavior local to the rule instead of forcing it into shared `llm-slop` support before there is a second proven use case.

## Information Sources
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs` — original implementation and matcher structure.
- `fixtures/medicaloutline/what-cancers-cannot-be-cured.md` — real missed case that exposed the one-hit-per-sentence limitation.
- `.worklogs/2026-03-26-132155-add-boilerplate-framing-rule.md` — initial rule rollout and rationale.

## Open Questions / Future Considerations
- If more accumulative rhetorical rules need multi-hit-per-sentence behavior, the traversal pattern may be worth extracting into shared family support. Right now that would be premature.
- `what-food-is-not-good-for-eczema.md` still stays below threshold, which looks correct under the current rule boundary.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs` — updated matcher and per-sentence multi-hit evidence collection.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing_tests/synthetic.rs` — synthetic proof that two distinct framing moves in one sentence both count.
- `fixtures/medicaloutline/what-cancers-cannot-be-cured.expected.general-en.json` — newly added real-world regression expectation.
- `.worklogs/2026-03-26-132155-add-boilerplate-framing-rule.md` — initial rollout context for this rule.

## Next Steps / Continuation Plan
1. Move on to the next `llm-slop` family, likely `llm-vocabulary` or `softening-language`.
2. Keep reviewing real fixture misses immediately after each new rule, because the Medical Outline corpus is proving more useful than synthetic coverage alone for finding threshold and evidence-shape mistakes.
3. Add the planned black-box fixture regression harness so these sidecars become enforced contracts instead of manual review artifacts.
