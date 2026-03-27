# Add Abstract Frames, Group Generalization, and Empty Labels

**Date:** 2026-03-27 17:41
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_07_universalizing_claims.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`, sibling synthetic tests, reviewed fixture baselines, workspace version metadata, `.plans/2026-03-27-abstract-frames-group-generalization-empty-label-plan.md`

## Summary
Added three new slop segments that were showing up in coaching-style and explainer prose: abstract evaluative frames, strong group-behavior generalization openers, and deictic empty virtue labels. The pass stayed on the existing evidence-first architecture: construction matchers with constrained part families, synthetic parity tests, reviewed baseline refreshes only where real additive hits were judged good, and a full release bump to `0.3.11`.

## Context & Problem
The current `llm-slop` coverage was already catching most overt coaching negation and signposting patterns, but several vacuous constructions were still passing:

- `The result worth caring about ...`
- `The bigger win is ...`
- `The useful alternatives are ...`
- `The point is to ...`
- `What matters is ...`
- `What helps is ...`
- `Most parents keep reaching ...`
- `That is discipline.`

The user explicitly asked to generalize these the same way we generalized negation: not raw phrase blacklists, but constrained rhetorical constructions assembled from curated parts. The precision bar stayed the same as earlier passes: low-noise by design, with concrete negatives for good prose and reviewed baseline updates only after manual inspection.

## Decisions Made

### Use construction families instead of phrase-by-phrase additions
- **Chose:** Added small matcher families for abstract evaluation frames in `generic-signposting`, a strong `most + group + keep + gerund` opener in `universalizing-claims`, and a narrow `that/this is discipline` branch in `empty-emphasis`.
- **Why:** These lines are repetitive rhetorical moves, not isolated strings. The combinatorial structure lets the rule grow from real evidence without reopening generic grammar matching.
- **Alternatives considered:**
  - Direct phrase bags for each example — rejected because it repeats the old duplication problem and hides the real rhetorical family.
  - Broad open-ended noun/verb slots — rejected because it would swallow legitimate prose too quickly.

### Keep each family narrowly curated
- **Chose:** Curated abstract nouns, modifiers, verbs, and tail starters instead of matching any `the X is ...` or `what Y is ...` form.
- **Why:** The goal was better coverage without losing the current false-positive posture. High-quality prose uses these skeletons too; the curated part families are what keep the rules rhetorical rather than grammatical.
- **Alternatives considered:**
  - Accept any noun after `the` or any verb after `what` — rejected because it would blur into normal exposition and technical writing.
  - Move these all into a new standalone rule — rejected because the ownership boundaries were already clear: meta frames stay in `generic-signposting`, human-generalization stays in `universalizing-claims`, empty relabel stays in `empty-emphasis`.

### Treat group-behavior generalization as strong evidence
- **Chose:** Let `Most parents keep reaching ...`-style openers fail on their own instead of only as an accumulative count.
- **Why:** In practice this is not a neutral statistic opener; it is a sweeping human-generalization frame used to tee up a rhetorical correction. Leaving it under the ordinary accumulation threshold would miss exactly the class we wanted.
- **Alternatives considered:**
  - Keep it accumulative-only — rejected because real lines of this form are already strong enough to be slop without repetition.
  - Put it under `generic-signposting` — rejected because the semantics are about population generalization, not meta framing.

### Refresh only reviewed baselines
- **Chose:** Snapshotted approved additive hits in explainers, Instagram, generated `gpt_5_4_mini`, and later one reviewed `sonnet_4_6` article after a fresh compare revealed `What matters is never missing twice.`
- **Why:** The baseline workflow is meant to preserve reviewed regression floors, not blindly rewrite every corpus after a rule expansion.
- **Alternatives considered:**
  - Blind snapshot of all corpora immediately after implementation — rejected because it would hide false positives or unwanted drift.
  - Leave the approved new `sonnet_4_6` hit unrecorded — rejected because the compare would remain noisy even though the hit was clearly in scope.

## Architectural Notes
- The pass follows the newer matcher style adopted in recent negation and slop refactors: helper functions model constrained rhetorical constructions, not regexes and not free-form grammar.
- `generic-signposting` now has a cleaner internal split:
  - legacy phrase families
  - strong meta evidence
  - abstract evaluation frame helpers
- `universalizing-claims` now has two evidence modes:
  - prior repeated broad-human framing
  - new single-hit strong behavior-generalization opener
- `empty-emphasis` remains intentionally narrow; the new `that/this is discipline` branch is a tiny sibling to the existing deictic significance lines rather than a general abstract-label matcher.
- Formatting-only leftovers from the earlier construction-refactor files stayed in this release because they were already dirty and benign, and the full suite plus corpus compares stayed green.

## Information Sources
- User-provided slop clusters in the live session, especially the vacuous frame list (`The bigger win is ...`, `What matters is ...`, `That is discipline.`).
- Existing matcher patterns in:
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_07_universalizing_claims.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`
- Existing corpus baseline tooling:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`
  - `scripts/social_fixture_failures.py`
- Prior worklogs that established the construction-first direction:
  - `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md`
  - `.worklogs/2026-03-27-164252-contrastive-aphorism-construction-refactor.md`
  - `.worklogs/2026-03-27-165356-response-wrapper-and-authority-padding-construction-refactor.md`

## Open Questions / Future Considerations
- `Most parents keep reaching ...` is now structurally supported, but this family is still synthetic-first. It needs more real approved fixture examples before further broadening.
- The abstract evaluation frame matcher is still intentionally conservative. There are nearby candidates like `The core issue is ...` and `The real task is ...`, but they should come from reviewed corpus evidence rather than expansion by intuition.
- `That is discipline.` is the first empty virtue label. If more variants appear (`That is leadership`, `That is parenting`, etc.), they should only be added with explicit false-positive negatives and real examples.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — meta-framing matcher plus the new abstract evaluation frame construction helpers.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs` — synthetic parity and negative tests for abstract frames.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_07_universalizing_claims.rs` — broad-human generalization rule, now with the strong group-behavior opener branch.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_07_universalizing_claims_tests/synthetic.rs` — tests showing the strong opener fires while concrete `Most parents keep ...` statements still pass.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs` — deictic emphasis rule with the new empty virtue label branch.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs` — synthetic guardrails for `That is discipline.` versus concrete definitional prose.
- `.plans/2026-03-27-abstract-frames-group-generalization-empty-label-plan.md` — plan for these three segments and their intended rule ownership.
- `scripts/generated_fixture_failures.py` — generated corpus compare/snapshot flow used to review additive hits.
- `scripts/explainer_fixture_failures.py` — explainer baseline workflow used in this pass.
- `scripts/social_fixture_failures.py` — social corpus baseline workflow used in this pass.
- `apps/prosesmasher/CHANGELOG.md` — public release summary for `0.3.11`.
- `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md` — prior design work that motivated construction-plus-parts matching.

## Next Steps / Continuation Plan
1. Continue harvesting real examples for the new abstract frame family from LinkedIn/Instagram-style corpora before broadening it further; focus on reviewed additions like `The real task is ...` or `The core issue is ...` only if they show up repeatedly.
2. Revisit `universalizing-claims` once more real `Most X keep Y-ing ...` examples are pinned in checked-in corpora, then decide whether to add adjacent verbs or subject families.
3. If more empty moral relabels emerge, extend `empty-emphasis` one virtue label at a time with concrete negatives, not as an open-ended abstract noun matcher.
4. Keep following the construction-unification plan for the remaining matcher-heavy rules, but only after real reviewed misses justify new part families.
