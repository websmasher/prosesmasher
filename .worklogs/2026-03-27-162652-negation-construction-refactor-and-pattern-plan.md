# Refactor Negation Patterns Into Construction Helpers

**Date:** 2026-03-27 16:26
**Scope:** `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`, `.plans/2026-03-27-pattern-construction-unification-plan.md`, reviewed fixture baselines under `fixtures/**`, workspace `Cargo.toml` files, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Refactored `negation-reframe` away from ad hoc tuple enumeration toward construction-level helper functions with constrained subject and copular families, then expanded the rule to cover reviewed same-subject corrective patterns already present in the corpora. Also wrote a follow-up plan for applying the same construction-plus-parts approach to the other matcher-heavy slop rules, refreshed approved baselines, and released the workspace as `0.3.7`.

## Context & Problem
The user pushed on a real weakness in the current negation matcher: several branches were still spelled as repeated string-pair tuples like `("they are not looking for ", "they are looking for ")`. That style is brittle, hard to expand, and hides the actual rhetorical construction being detected.

At the same time, the user explicitly did **not** want free-form grammatical generalization. The goal was not “match any negation sentence.” The goal was:
- keep matching the same rhetorical construction
- decompose it into constrained parts
- support natural variants like pronoun swaps, negation-form swaps, and reversed clause order where the rhetoric is truly symmetric
- avoid broadening into normal technical or explanatory contrast prose

The immediate pressure was practical, not theoretical. The current matcher was still missing reviewed slop examples such as:
- `Your job is not ... Your job is ...`
- `They are not looking for ... They are looking for ...`
- `The best result is usually not ... It is ...`
- `The useful alternatives are not ... They are ...`
- `People do not procrastinate because ... They procrastinate because ...`

Those were already judged good catches from real corpora, so the work needed to both clean up the matcher shape and widen coverage in the same pass without regressing existing reviewed baselines.

## Decisions Made

### Refactor Around Rhetorical Constructions, Not Free Cartesian Products
- **Chose:** Introduce construction-level helpers such as `same_subject_copular_corrective(...)` and use constrained subject and copular families (`CORRECTIVE_PLURAL_SUBJECTS`, `PRESENT_COPULAR_NEGATION_FORMS`) instead of hand-enumerated string-pair tuples.
- **Why:** This preserves the actual rhetorical intent of the matcher while removing needless duplication. It also matches the user’s requested abstraction level: a construction composed from allowed parts, not a list of raw strings and not a generic grammar engine.
- **Alternatives considered:**
  - Keep adding tuple pairs one by one — rejected because the rule was already becoming unreadable and every new reviewed variant multiplied duplication.
  - Generalize all the way to “any subject + any negation form + any verb” — rejected because that would turn the rule into generic contrast detection and produce obvious false positives in technical and explanatory prose.

### Expand Only Reviewed Negation Families
- **Chose:** Add new same-subject families that were already justified by user examples or reviewed corpus hits:
  - repeated abstract frames: `job`, `best result`, `replacement`, `useful alternative(s)`
  - same-subject `looking for`
  - human-subject `do not need` / `they need`
  - human-subject corrective followups that repeat the rhetorical subject family through `they` / `it`
- **Why:** The repo already had enough evidence that these are real slop constructions. The right move was to encode the reviewed families explicitly rather than keep missing them.
- **Alternatives considered:**
  - Only refactor structure without changing behavior — rejected because the user asked to commit and implement, and the existing misses were already approved as desired catches.
  - Broaden every uncaught negation-like line immediately — rejected because some remaining patterns (`not just`, single-sentence contrasts, generic technical contrasts) still have unclear false-positive boundaries.

### Keep Human-Followup Logic Constrained
- **Chose:** Build human corrective followups around explicit human-subject detection and constrained followup pronouns (`they`, `it`) instead of any noun phrase followed by any affirmative clause.
- **Why:** This is where the false-positive risk rises fastest. The reviewed desired hits are overwhelmingly human-behavior rhetoric, so the matcher should remain aligned with that domain instead of claiming all noun-based contrast is slop.
- **Alternatives considered:**
  - Allow any subject once the surface pattern matches — rejected because it would incorrectly absorb technical prose like system/problem/component reframes.
  - Skip the human-subject branch entirely — rejected because several reviewed true positives depended on exactly that behavior.

### Refresh Only Approved Baselines And Re-verify All Corpus Gates
- **Chose:** Refresh only the baselines for the generated, explainer, and social fixtures that picked up reviewed additive negation catches, then re-run all three compare scripts until they returned `[]`.
- **Why:** The user’s review posture has been consistent: additive is acceptable when the new catches are genuinely bad. That means the safe release gate is not “unchanged output,” but “unchanged except for reviewed good additions.”
- **Alternatives considered:**
  - Bulk-refresh every baseline immediately after the code change — rejected because it would hide whether the new negation branches leaked into unrelated corpora.
  - Require all corpora to remain byte-for-byte unchanged — rejected because the point of the pass was to catch reviewed slop already present in those corpora.

### Write A Forward Plan For Construction Unification
- **Chose:** Add `.plans/2026-03-27-pattern-construction-unification-plan.md` documenting which rules should adopt the same construction-plus-parts approach next.
- **Why:** `negation-reframe` was the first obvious candidate, but not the last. The repo now has enough matcher-heavy rules that the abstraction style should be deliberate and reusable rather than rediscovered ad hoc later.
- **Alternatives considered:**
  - Leave the broader cleanup implicit — rejected because future passes would otherwise drift back toward tuple lists and one-off phrase bags.
  - Refactor multiple other rules in the same commit — rejected because the user asked to commit and implement this pass, and `negation-reframe` was the clearest immediate target. The broader sweep deserves its own reviewed follow-up.

## Architectural Notes
- `negation-reframe` is now closer to the preferred matcher shape for this codebase:
  - rhetorical construction
  - constrained part families
  - explicit ordering logic
  - synthetic negatives that pin the false-positive boundary
- The current refactor is intentionally partial. It improves the worst hardcoded branch and extends the pattern inventory, but it does **not** yet implement full reversible-order matching for every construction. That is recorded as follow-up work in the new plan.
- The new plan explicitly ranks future migration candidates:
  1. `negation-reframe` follow-through
  2. `contrastive-aphorism`
  3. `response-wrapper`
  4. `authority-padding`
  5. lower-priority cleanup for `generic-signposting` and `lesson-framing`
- This keeps the codebase aligned with the broader project rule: abstract the rhetoric, but keep predicate families constrained to reviewed slop rather than free-form grammar.

## Information Sources
- User-reviewed negation examples from the current conversation, especially:
  - `Your job is not ... Your job is ...`
  - `They are not looking for ... They are looking for ...`
  - `Most new moms do not need ... They need ...`
  - `The useful alternatives are not ... They are ...`
  - `The best result is usually not ... It is ...`
- Existing matcher implementation in:
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`
- Synthetic tests in:
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`
- Regression tooling:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`
  - `scripts/social_fixture_failures.py`
- Prior project state:
  - `.worklogs/2026-03-27-134731-add-short-form-slop-families.md`
  - `.worklogs/2026-03-27-141640-setup-binstall-github-release-surface.md`
  - `.worklogs/2026-03-27-143047-fix-dist-macos-release-packaging.md`

## Open Questions / Future Considerations
- Reverse-order support for same-subject corrective pairs is conceptually approved (`you are looking for X / you are not looking for Y`) but not yet implemented as a generalized helper across all branches.
- Predicate-family harvesting should continue from real reviewed slop examples rather than expanding to arbitrary verbs. The user is right that there are already enough bad verbs to justify families, but the families still need to be curated rather than opened up mechanically.
- Some still-uncaught inline or single-sentence contrasts (`not just`, short `X, not Y` lines, `It does not ... It does ...` pairs) may belong to other families instead of `negation-reframe`. Future expansion should keep that boundary explicit.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — current construction-level negation matcher and the new helper/constant shape
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — approved positives and guardrail negatives for the new negation families
- `.plans/2026-03-27-pattern-construction-unification-plan.md` — follow-up plan for construction-plus-parts refactors across other slop rules
- `scripts/generated_fixture_failures.py` — generated-corpus regression gate
- `scripts/explainer_fixture_failures.py` — explainer-corpus regression gate
- `scripts/social_fixture_failures.py` — short-form social-corpus regression gate
- `apps/prosesmasher/CHANGELOG.md` — `0.3.7` release note for this matcher/refactor pass
- `.worklogs/2026-03-27-134731-add-short-form-slop-families.md` — prior work on corpus-based slop family expansion
- `.worklogs/2026-03-27-141640-setup-binstall-github-release-surface.md` — prior release-surface setup affecting versioning and public install flow

## Next Steps / Continuation Plan
1. Implement reversible-order support in `negation-reframe` where the rhetorical family is truly symmetric.
   Read and modify:
   - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`
   - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`
   Add both positive and technical-prose negative tests before broadening the matcher.
2. Move `contrastive-aphorism` to the same construction-plus-parts style next.
   Read:
   - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`
   Use the new plan file as the decision boundary for subject, predicate, and order families.
3. Apply the same cleanup to `response-wrapper` and `authority-padding`, but only after collecting the existing reviewed phrase clusters into explicit predicate families.
4. Keep the same release gate on future matcher changes:
   - run `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - run all three compare scripts
   - review additive hits manually before refreshing any sidecars
5. If the rule surface changes materially again, bump the workspace version and changelog in the same pass so the installed CLI version stays aligned with the actual released matcher set.
