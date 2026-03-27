# Refactor Contrastive Aphorism Into Construction Helpers

**Date:** 2026-03-27 16:42
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism_tests/synthetic.rs`, workspace `Cargo.toml` files, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Refactored `contrastive-aphorism` away from ad hoc token-slice branches toward a small construction-plus-parts matcher vocabulary, then added the two reviewed advisory contrast lines the user had already identified as real misses. Released the result as `0.3.8` after verifying the full workspace and all generated, explainer, and social corpora stayed clean.

## Context & Problem
After the `negation-reframe` refactor, the next obvious matcher with the same code smell was `contrastive-aphorism`. It still relied on a grab bag of branch-local `matches!(tokens.as_slice(), ...)` expressions:
- imperative contrast
- human-plural `get X in Y, not Z`
- `watch for a pattern, not one bad week`
- `like a problem, not a problem`

Those branches were not yet terrible, but they were already drifting toward the same maintenance problem:
- each new reviewed rhetorical variant wanted another hand-written shape
- the shared parts were implicit instead of named
- the user had just pushed exactly on this issue: move the matcher to constrained parts and rhetorical constructions, not duplicated raw strings

There were also two reviewed misses already approved by the user as good catches:
- `I would give one anchor, not a buffet.`
- `I would expect repetition, not elegance.`

That made this a good pass to do both:
1. clean up the matcher shape
2. add one new construction family that fit the same rule cleanly

## Decisions Made

### Introduce A Small Local Pattern Vocabulary Instead Of More Slice Macros
- **Chose:** Add a local `TokenPart` matcher enum with:
  - `Exact`
  - `OneOf`
  - `Article`
  - `Any`
  and a shared `matches_pattern(...)` helper.
- **Why:** This keeps the abstraction at the rhetorical-construction level without turning the rule into a generic grammar engine. It also matches the direction set in the new pattern-unification plan: compose reviewed constructions from constrained part families.
- **Alternatives considered:**
  - Keep the old `matches!(tokens.as_slice(), ...)` branches and only rename constants — rejected because that would preserve the duplication and not actually change the matcher model.
  - Build a fully generic token-pattern framework shared across the whole repo in this pass — rejected because this rule only needed a small local helper and a repo-wide framework would be premature.

### Refactor The Existing Contrast Families To Use Construction Helpers
- **Chose:** Convert the current imperative, `get ... in ... not ...`, `watch for ... not ...`, and `like a problem ... not a problem` branches to use helper functions built on the new token-part matcher.
- **Why:** These are the exact kinds of branches that should become declarative constructions with named part families. It makes future reviewed variants easier to add without slipping back into one-off token arrays.
- **Alternatives considered:**
  - Only use the helper for the new advisory branch and leave the old branches untouched — rejected because that would create two matcher styles in the same rule and defeat the point of the cleanup.

### Add A Narrow Advisory-Contrast Construction
- **Chose:** Add `matches_modal_article_noun_contrast(...)` for:
  - subject family: `["i"]`
  - modal: `would`
  - verb family: `["give", "expect"]`
  - negative abstract noun family: `["buffet", "elegance"]`
- **Why:** The user had already explicitly approved those two lines as slop, and they fit the rule’s existing rhetorical function: short coaching-style compressed contrasts.
- **Alternatives considered:**
  - Fold them into `negation-reframe` — rejected because these are not corrective reframes; they are short aphoristic contrasts.
  - Generalize to any `I would <verb> ... not ...` construction — rejected because that would immediately catch legitimate advisory or technical contrasts like `I would use one parser, not a buffer.`

### Support Both Article And Article-Less Reviewed Advisory Forms
- **Chose:** Let the new advisory helper match both:
  - `I would give one anchor, not a buffet.`
  - `I would expect repetition, not elegance.`
- **Why:** The second reviewed miss failed the first implementation because it lacked an article on the right-hand noun. That was a concrete proof that the initial construction was too narrow for the approved examples.
- **Alternatives considered:**
  - Force all advisory contrasts to keep articles — rejected because it would keep missing one of the reviewed target sentences.
  - Open the right-hand side to any noun phrase length — rejected because that would over-broaden the construction without evidence.

### Keep The Release Regression Bar Strict
- **Chose:** Re-run:
  - targeted `contrastive_aphorism` tests
  - full workspace tests
  - generated fixture compares
  - explainer fixture compare
  - social fixture compare
  and keep the pass only if the real corpora stayed clean.
- **Why:** This was a structural refactor plus a narrow behavior change. The user wanted function-style cleanup, but not at the cost of new false positives.
- **Alternatives considered:**
  - Skip corpus gates because the intended additions were only synthetic — rejected because structural refactors can still drift into existing corpora accidentally.
  - Snapshot baselines blindly if anything moved — rejected because there was no reason to accept unreviewed real-corpus changes for this pass.

## Architectural Notes
- `contrastive-aphorism` now follows the same design direction as the recent `negation-reframe` refactor:
  - construction helper
  - constrained part families
  - explicit synthetic negatives that pin the false-positive boundary
- The new local pattern vocabulary is intentionally file-scoped. It is enough for this rule, but not yet a shared runtime utility. That keeps the abstraction cheap while the broader repo-level direction is still being validated rule by rule.
- This pass does **not** yet attempt the broader next refactors listed in the unification plan:
  - `response-wrapper`
  - `authority-padding`
  Those remain the next likely candidates once this rule’s shape is established.

## Information Sources
- Current rule and tests:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism_tests/synthetic.rs`
- Plan from the previous pass:
  - `.plans/2026-03-27-pattern-construction-unification-plan.md`
- User-reviewed target lines from the current conversation:
  - `I would give one anchor, not a buffet.`
  - `I would expect repetition, not elegance.`
- Regression tooling reused from the rest of the project:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`
  - `scripts/social_fixture_failures.py`
- Prior worklog for the same abstraction direction:
  - `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md`

## Open Questions / Future Considerations
- If more approved advisory contrasts accumulate, the advisory branch should grow by reviewed predicate/noun families rather than by one-off phrase literals.
- The local `TokenPart` helper may eventually want to migrate into shared matcher utilities, but only after at least one or two more rules prove the same shape is genuinely reusable.
- `contrastive-aphorism` still has a few branches (`curriculum` pair, `part-that-sticks`, `it changes everything`) that remain simple direct matchers. That is acceptable for now; there is no reason to force them into the pattern helper unless they start accreting variants.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs` — the refactored construction-level matcher and new advisory-contrast branch
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism_tests/synthetic.rs` — reviewed positives and negatives that define the new branch boundary
- `.plans/2026-03-27-pattern-construction-unification-plan.md` — broader plan for applying the same approach to other rules
- `apps/prosesmasher/CHANGELOG.md` — `0.3.8` release note for this refactor
- `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md` — previous pass that established the same abstraction direction in `negation-reframe`

## Next Steps / Continuation Plan
1. Refactor [slop_02_response_wrapper.rs](/Users/tartakovsky/Projects/websmasher/prosesmasher/apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs) to construction-plus-parts helpers next.
   Use families like:
   - first-person subject
   - limitation/capability auxiliary
   - action family
   - object family
2. After that, do the same for [slop_12_authority_padding.rs](/Users/tartakovsky/Projects/websmasher/prosesmasher/apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding.rs), where the current matcher still behaves more like a phrase-family bag.
3. Keep the same release gate for both:
   - targeted rule tests
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - generated, explainer, and social compare scripts
4. Only snapshot real-corpus baselines if the additive hits are manually reviewed and clearly belong to the intended rule family.
