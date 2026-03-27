# Narrow Negation Gap Closure

**Date:** 2026-03-27 17:13
**Scope:** `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs`, reviewed baseline sidecars under `fixtures/gpt_5_4_mini/` and `fixtures/linkedin/gpt_5_4_mini/`, workspace/root `Cargo.toml` files, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Closed another narrow batch of approved negation gaps without widening into generic explanatory `..., not ...` prose. The pass adds three new corrective families in `negation-reframe`, one short dismissive meta line in `empty-emphasis`, refreshes only the reviewed baselines that gained good catches, and releases the workspace as `0.3.10`.

## Context & Problem
The user provided another bundle of negation-heavy slop lines and asked specifically to skip unrelated cliche imagery like the “blue cup” line. After the earlier `0.3.9` construction refactors, the bundle still had six misses, but only four of them were safe enough to absorb without broadening the matcher into ordinary explanatory contrast prose:

- `So the answer is not tougher energy. It is steadier energy.`
- `That does not make the hitting okay. It does explain the pattern.`
- `Hitting back teaches fear and confusion. It does not teach regulation.`
- `What helps is not brilliant.`

Two remaining misses were different:
- `This works because the baby gets a supported transition, not a sudden disappearance.`
- `need supervision, coaching, and environmental changes, not hopeful speeches.`

Those are single-sentence inline `..., not ...` contrasts. A corpus scan showed that this shape appears constantly in normal prose across explainers and generated articles, so folding it into `negation-reframe` right now would mean turning a narrow corrective matcher into a broad generic-contrast detector. The correct choice in this pass was to close the clearly approved adjacent families and leave the higher-risk inline shapes out of scope.

## Decisions Made

### Add `answer` To The Abstract Corrective Frame Family
- **Chose:** Extend the existing abstract frame families with `answer` so the existing `goal/point/aim/best result/... is not X -> it is Y` construction also covers `The answer is not X. It is Y.`
- **Why:** This is the same rhetorical family as the already-approved `goal` and `best result` lines. It was a real reviewed miss, not a speculative new branch.
- **Alternatives considered:**
  - Add a dedicated one-off `answer` branch — rejected because it is structurally identical to the existing abstract-frame helper and would just reintroduce duplication.
  - Generalize to any noun before `is not` — rejected because that would immediately widen into ordinary definitional or technical contrasts.

### Add A Narrow `does not make X okay -> it does explain Y` Family
- **Chose:** Add `looks_like_make_okay_explain_contrast_sentence(...)` for lines like `That does not make the hitting okay. It does explain the pattern.`
- **Why:** The user explicitly marked this as slop, and the construction is specific enough to keep precision high:
  - demonstrative subject (`this/that/it`)
  - `does not make`
  - explicit `okay`
  - paired `does explain`
- **Alternatives considered:**
  - Treat it as a generic `does not X -> it does Y` family — rejected because that would be far too broad.
  - Put it in `contrastive-aphorism` — rejected because this is not a compressed slogan line; it is a corrective explanation pair.

### Add A Narrow `X teaches Y. It does not teach regulation` Family
- **Chose:** Add `looks_like_teaches_not_teach_regulation(...)` for the specific moralized training contrast:
  - first sentence contains `teaches`
  - second sentence starts with `it does not teach ...`
  - object is constrained to `regulation`, `self-control`, `restraint`, or `repair`
- **Why:** This directly covers the reviewed `Hitting back teaches fear and confusion. It does not teach regulation.` line while keeping the object family explicitly tied to this coaching-slop rhetoric.
- **Alternatives considered:**
  - Match any `X teaches Y. It does not teach Z.` pair — rejected because that would spill into ordinary educational and technical prose.
  - Move it under `contrastive-aphorism` — rejected because the sentence pair is explanatory and corrective, not sloganized.

### Add `What helps is not brilliant.` To `empty-emphasis`
- **Chose:** Add one new exact short dismissive meta line in `empty-emphasis`.
- **Why:** The sentence is pure empty coaching compression and fits the existing rule better than `negation-reframe`. It is not a real corrective pair; it is deictic filler that dismisses complexity before the real content.
- **Alternatives considered:**
  - Force it into `negation-reframe` because it contains `not` — rejected because the rhetorical function is wrong.
  - Generalize to all `what helps is not X` lines — rejected because we only had one approved phrasing and broader variants could start hitting concrete explanatory prose.

### Keep Inline `..., not ...` Causal/Fragment Contrasts Out Of Scope
- **Chose:** Leave the two remaining one-sentence misses uncaught:
  - `This works because ... , not ...`
  - `need ..., not ...`
- **Why:** Repo-wide fixture search showed too many legitimate `..., not ...` uses in normal prose. Those shapes need their own future constrained families if they are worth catching, not an opportunistic widening of `negation-reframe`.
- **Alternatives considered:**
  - Add a broad inline clause branch immediately — rejected because the false-positive surface is obviously too large.
  - Add one-off literal phrase checks for `supported transition` / `hopeful speeches` — rejected because that would be overfit and architecturally backwards after the recent construction-helper work.

### Refresh Only Reviewed Baselines
- **Chose:** Snapshot only the approved additive hits in:
  - `fixtures/linkedin/gpt_5_4_mini/why-modern-work-makes-people-feel-constantly-behind.baseline.general-en.json`
  - `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.baseline.general-en.json`
- **Why:** Both additions were clearly good:
  - `The answer is not to become superhuman. It is to build small pockets of closure.`
  - `The answer is not to wait for life to calm down. It is to install a new cue...`
  This follows the established rule: additive is acceptable after review; blind snapshot churn is not.
- **Alternatives considered:**
  - Require all baselines to remain unchanged — rejected because the point of the pass was to catch reviewed misses already present in the corpora.
  - Bulk-refresh all corpora after the code change — rejected because that would hide whether anything new was actually reviewed.

## Architectural Notes
- This pass continues the “construction + constrained parts” direction without pretending every `not` sentence belongs to one giant family.
- `negation-reframe` remains the home for adjacent corrective explanation pairs, not for all inline contrast rhetoric.
- `empty-emphasis` remains the home for short deictic dismissive/meta lines, even if they contain negation.
- The current clean line is:
  - catch reviewed adjacent corrective pairs
  - reject broad single-sentence inline contrast generalization
- The post-change bundle result was:
  - caught: 18/20
  - remaining intentionally uncaught: the two high-risk inline `..., not ...` forms above

## Information Sources
- Current matcher implementation:
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`
- New synthetic guardrails:
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs`
- Prior matcher-refactor worklogs:
  - `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md`
  - `.worklogs/2026-03-27-164252-contrastive-aphorism-construction-refactor.md`
  - `.worklogs/2026-03-27-165356-response-wrapper-and-authority-padding-construction-refactor.md`
- Regression tooling:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`
  - `scripts/social_fixture_failures.py`
- User-reviewed bundle from the current conversation, especially the approved misses listed above and the explicit “skip blue cup” instruction.

## Open Questions / Future Considerations
- The remaining inline `..., not ...` misses may justify a separate future family if more reviewed examples accumulate:
  - causal support/effectiveness contrasts (`works because X, not Y`)
  - fragmentary needs/tools contrasts (`need X, not Y`)
- If that happens, the matcher should be built from constrained predicate/noun families harvested from real reviewed examples, not from generic clause-level negation detection.
- `What helps is not brilliant.` is still an exact `empty-emphasis` phrase, not a generalized family. That is intentional until more approved variants appear.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — current corrective negation matcher, including the new `answer`, `make ... okay`, and `teach regulation` families
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — synthetic positives/negatives pinning the new negation branches
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs` — current short deictic/meta filler matcher with the new `what-helps-not-brilliant` line
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs` — the exact positive/negative guardrails for the new meta line
- `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.baseline.general-en.json` — reviewed generated-corpus additive negation baseline from this pass
- `fixtures/linkedin/gpt_5_4_mini/why-modern-work-makes-people-feel-constantly-behind.baseline.general-en.json` — reviewed social-corpus additive negation baseline from this pass
- `apps/prosesmasher/CHANGELOG.md` — `0.3.10` release note
- `.worklogs/2026-03-27-162652-negation-construction-refactor-and-pattern-plan.md` — earlier pass that established the current negation-construction direction

## Next Steps / Continuation Plan
1. If the user wants to keep pushing negation coverage, collect more real reviewed examples specifically for the two remaining inline shapes:
   - `works because X, not Y`
   - fragmentary `need X, not Y`
2. Before adding either family, search the corpora for legitimate uses of those shapes and write explicit synthetic negatives from technical/explanatory prose.
3. Do **not** broaden `negation-reframe` with a generic one-sentence `..., not ...` branch. Any future work there should be a new constrained family or a different rule.
4. Keep using the same release gate:
   - targeted synthetic tests
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - generated compares per model
   - explainer compare
   - social compare
5. If a future pass adds reviewed hits, refresh only those approved sidecars instead of bulk-snapshotting the whole corpus.
