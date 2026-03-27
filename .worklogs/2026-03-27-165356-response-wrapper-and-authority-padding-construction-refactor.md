# Refactor Response Wrapper And Authority Padding

**Date:** 2026-03-27 16:53
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding_tests/synthetic.rs`, workspace `Cargo.toml` files, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Refactored `response-wrapper` and `authority-padding` from phrase-bag matchers into constrained construction helpers built from subject, auxiliary, action, object, and predicate families. Released the result as `0.3.9` after keeping the full workspace green and all generated, explainer, and social corpus compares clean.

## Context & Problem
After `negation-reframe` and `contrastive-aphorism`, the next matcher-heavy rules still stuck in the older style were:
- `response-wrapper`
- `authority-padding`

Both rules still worked, but they were expressed mainly as:
- repeated full-sentence prefix strings
- branch-local `starts_with` / `contains` phrase bags
- special cases that encoded the construction implicitly instead of naming its parts

That created the same maintenance problem the user had been pushing on:
- the code made the rhetorical construction hard to see
- new reviewed variants would push the matcher back toward ad hoc string accumulation
- there was no explicit boundary between “construction family” and “raw phrase list”

The goal in this pass was not to broaden either rule aggressively. It was:
1. move both rules to the same construction-plus-parts style
2. add just enough synthetic coverage to pin the new helper paths
3. preserve real-corpus behavior unless a new hit was clearly intentional

## Decisions Made

### Refactor `response-wrapper` Into Subject/Auxiliary/Action/Object Families
- **Chose:** Replace the big phrase arrays with helper functions:
  - `matches_subject_aux_action_objects(...)`
  - `matches_subject_aux_objects(...)`
  - `matches_subject_prefix_action_objects(...)`
  and explicit families for:
  - first-person subjects
  - capability auxiliaries
  - limitation auxiliaries
  - ability-limitation prefixes
  - action families
  - information / advice / diagnosis object families
- **Why:** The rule is fundamentally about a small number of assistant-wrapper constructions, not about memorizing whole strings. The refactor makes that explicit and makes future reviewed variants cheaper to add without changing the rule boundary.
- **Alternatives considered:**
  - Keep the current phrase lists and only rename them — rejected because that would preserve the same matcher shape problem.
  - Build a shared generic matcher framework in `support.rs` immediately — rejected because these helper shapes are still local enough that a file-scoped refactor is clearer and lower-risk.

### Keep `response-wrapper` Behavior Narrow
- **Chose:** Preserve the current boundary:
  - first-person assistant/service framing only
  - no broadening to plain consultation prose
  - no attempt to absorb general first-person explanation prose
- **Why:** This rule already has a clean purpose. The refactor should improve structure, not silently start flagging “I can explain how X works” lines.
- **Alternatives considered:**
  - Broaden capability detection to any first-person capability claim — rejected because that would create obvious false positives on normal educational prose.
  - Fold consultation-style language into the same construction set — rejected because the rule intentionally leaves ordinary “consult a professional” prose alone.

### Add Explicit Synthetic Coverage For `medical expertise`
- **Chose:** Add a direct positive for `I do not have medical expertise ...` and a direct negative for concrete first-person capability prose.
- **Why:** The refactor preserved this special-case family by turning it into a subject/aux/object construction. The new tests pin both the intended positive and the key false-positive boundary.
- **Alternatives considered:**
  - Rely on the pre-existing tests only — rejected because they did not exercise the `medical expertise` helper path at all.

### Refactor `authority-padding` Into Subject/Predicate Families
- **Chose:** Replace repeated `starts_with_any(...)` buckets with explicit families:
  - `EVIDENCE_SUBJECTS`
  - `RESEARCH_SUBJECTS`
  - `RESEARCHER_SUBJECTS`
  - `EVIDENCE_PREDICATES`
  - `RESEARCH_PREDICATES`
  - `RESEARCHER_PREDICATES`
  plus a shared `matches_subject_predicate_family(...)`.
- **Why:** This rule is structurally about “authority subject + vague credibility predicate.” Naming those families directly makes the matcher much easier to extend and reason about.
- **Alternatives considered:**
  - Keep the old string arrays and just group them better — rejected because the construction would still be implicit.
  - Try to normalize every prestige/evidence/research form into one giant universal family — rejected because evidence, research, and prestige still have distinct boundaries and should stay separate.

### Preserve The Prestige And Reviewed Special Cases Without Overgeneralizing
- **Chose:** Keep:
  - the possessive prestige suffix (`work is famous for a reason`)
  - `the strongest recent evidence points ...`
  - `the broader research backs ...`
 as deliberate reviewed branches around the new subject/predicate family core.
- **Why:** Those are already proven useful and readable as small explicit exceptions. Forcing them into a bigger abstraction would not improve the rule today.
- **Alternatives considered:**
  - Force every branch into the same helper — rejected because it would add abstraction noise without improving the actual boundary.

### Add Synthetic Coverage For `researchers keep finding`
- **Chose:** Add a positive for `Researchers keep finding ...` and a negative technical sentence that uses `the research points ...` in a domain-specific, non-sloppy way.
- **Why:** This locks down both the new helper family and one likely false-positive edge after the refactor.
- **Alternatives considered:**
  - Skip new tests because existing authority-padding coverage already existed — rejected because the new helper families introduced a new path that deserved direct guardrails.

## Architectural Notes
- This pass continues the repo’s current matcher direction:
  - rule-local construction helpers
  - constrained part families
  - synthetic tests that pin both the positive family and the likely boundary negatives
- No real-corpus baselines changed in this pass.
  - generated per-model compares: `[]`
  - explainer compare: `[]`
  - social compare: `[]`
- That means this is a structural cleanup plus synthetic hardening release, not a corpus-expansion release.

## Information Sources
- Runtime files:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding.rs`
- Synthetic tests:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper_tests/synthetic.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding_tests/synthetic.rs`
- Prior worklogs that established the same design direction:
  - `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md`
  - `.worklogs/2026-03-27-164252-contrastive-aphorism-construction-refactor.md`
- Regression tooling:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`
  - `scripts/social_fixture_failures.py`

## Open Questions / Future Considerations
- `response-wrapper` still has an intentionally narrow actor boundary around explicit first-person assistant-style framing. If future reviewed corpora show broader but still precise wrapper constructions, they should be added as new families, not by loosening the subject/action universe arbitrarily.
- `authority-padding` can now absorb additional reviewed subject/predicate families cleanly. The next real expansion should come from reviewed corpus examples, not from theoretical generalization.
- The next obvious matcher to refactor in this style is still `generic-signposting` or `lesson-framing`, but they are lower leverage than the two rules handled here.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs` — refactored capability/limitation construction matcher
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper_tests/synthetic.rs` — response-wrapper positives and new boundary negatives
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding.rs` — refactored authority subject/predicate matcher
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding_tests/synthetic.rs` — authority-padding positives and technical false-positive guardrails
- `apps/prosesmasher/CHANGELOG.md` — `0.3.9` release note
- `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md` — previous pass setting the abstraction direction
- `.worklogs/2026-03-27-164252-contrastive-aphorism-construction-refactor.md` — previous matcher refactor in the same style

## Next Steps / Continuation Plan
1. Refactor `generic-signposting` or `lesson-framing` only if there is actual duplication pressure, not just for symmetry.
2. When new reviewed corpora produce additional `response-wrapper` or `authority-padding` misses, add them as new part families on top of these helpers rather than returning to phrase-list branches.
3. Keep the same release gate:
   - targeted rule tests
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - generated per-model compares
   - explainer compare
   - social compare
4. If a future matcher refactor does create additive corpus hits, review those before any baseline refresh instead of assuming structural changes are behavior-neutral.
