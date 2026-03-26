# Add Boilerplate Conclusion And Generated Fixture Baselines

**Date:** 2026-03-26 18:38
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/*`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `scripts/generated_fixture_failures.py`, generated fixture baseline sidecars under `fixtures/{gpt_5_2_chat,gpt_5_4,gpt_5_4_mini,haiku,opus_4_6,sonnet_4_6}/*/article.baseline.general-en.json`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added `boilerplate-conclusion` as the next `llm-slop` rule and paired it with a formal generated-fixture regression loop. The new loop snapshots per-article failure counts for the 60 model-written article fixtures, compares future changes against that baseline, and was used in this change to confirm that the new rule only added five meaningful slop hits and removed nothing.

## Context & Problem
The current `llm-slop` family had moved beyond explicit assistant disclaimers, but the generated article corpus still contained polished closing boilerplate that read like model-written article wrap-up language. The user explicitly asked for a formal verification approach before accepting new rule work: write down the current error list for each generated article, compare after the change, and inspect whether newly caught text is actually slop.

That meant the task was not just “add another rule.” It needed:
- a new rule with a low false-positive boundary
- a persisted pre/post comparison mechanism over the new generated article corpora
- an additive review pass so no previously known findings silently disappeared

## Decisions Made

### Make `boilerplate-conclusion` immediate and end-position scoped
- **Chose:** Implement `boilerplate-conclusion` as an immediate English-only rule that inspects only the final three paragraph sentences of the document.
- **Why:** The repeated miss in the corpus was not “documents contain several conclusion clichés.” It was single, canned, moralizing endings. An accumulative threshold would miss the actual failure shape.
- **Alternatives considered:**
  - Make it accumulative like `boilerplate-framing` — rejected because most real article-template closers appear once.
  - Let it scan the whole document — rejected because phrases like `the research is clear` can be legitimate mid-article transitions or evidence setup.

### Keep the matcher combo-based but intentionally narrow
- **Chose:** Start with three bounded pattern families:
  - `insight-close`
  - `authority-close`
  - `acceptance-close`
- **Why:** The user wanted less exact-phrase dependence, but also near-zero false positives. The safest way to do that was to keep the rule narrow and evidence-first, then validate it against the generated corpus before broadening.
- **Alternatives considered:**
  - Ship a broad conclusion-phrase bag (`the bottom line`, `ultimately`, `in conclusion`) — rejected because those are common in legitimate writing.
  - Fold these patterns into `boilerplate-framing` — rejected because conclusion ownership is distinct from topic setup and needs its own boundary.

### Add a dedicated generated-fixture snapshot/compare tool
- **Chose:** Add `scripts/generated_fixture_failures.py` with two commands:
  - `snapshot` to write `article.baseline.general-en.json`
  - `compare` to diff current failures against the saved baseline
- **Why:** The user asked for a formal verification loop, not just ad hoc reruns. The sidecars make the generated corpus usable as a regression floor.
- **Alternatives considered:**
  - One giant corpus manifest — rejected because per-article sidecars are easier to inspect and evolve.
  - Manual notes outside the repo — rejected because they would drift immediately.

### Refresh the generated baselines only after reviewing the delta
- **Chose:** First compare the new rule against the previously snapshotted corpus, inspect every new `boilerplate-conclusion` hit manually, and only then rewrite the baseline sidecars to the new accepted state.
- **Why:** This preserves the user’s additive rule: do not blindly rewrite fixture expectations until the new detections have been judged.
- **Alternatives considered:**
  - Snapshot immediately after implementing the rule — rejected because that would erase the ability to distinguish intended additions from accidental regressions.

### Accept the five additive corpus hits
- **Chose:** Keep the matcher after the compare showed five additive hits and no removals:
  - `fixtures/gpt_5_2_chat/why_people_wake_up_tired/article.md`
  - `fixtures/haiku/stress_and_physical_symptoms/article.md`
  - `fixtures/opus_4_6/social_anxiety_in_daily_life/article.md`
  - `fixtures/opus_4_6/why_couples_stop_communicating/article.md`
  - `fixtures/opus_4_6/why_people_wake_up_tired/article.md`
- **Why:** Manual review showed these were all genuine article-template closers: reassurance morality, “research is clear” wrap-up certainty, or “most important step/idea” summary rhetoric near the document ending.
- **Alternatives considered:**
  - Tighten away `most important step` by removing `step` from the summary noun set — rejected for now because the observed `social_anxiety_in_daily_life` closer still looked like the exact abstract wrap-up we want to catch.
  - Broaden immediately to other conclusion patterns like `the good news is` — rejected because this change was already sufficiently validated and should not grow before another corpus pass.

### Release as `0.2.5`
- **Chose:** Bump the workspace and all internal `prosesmasher*` package versions from `0.2.4` to `0.2.5`, update the changelog, and verify `prosesmasher --version`.
- **Why:** The repo’s current release discipline requires a semantic version bump on each shipped build.
- **Alternatives considered:**
  - Leave versions unchanged until a larger batch accumulates — rejected because it breaks the established release contract for this repo.

## Architectural Notes
`boilerplate-conclusion` follows the current rule architecture exactly:
- runtime rule in `llm-slop/runtime`
- reusable public-behavior assertions in `llm-slop/assertions`
- sidecar synthetic tests under `slop_08_boilerplate_conclusion_tests/`

The generated-fixture comparison loop is intentionally outside the rule families in `scripts/` because it verifies the whole CLI/catalog/parser stack, not one rule family in isolation. It also complements the existing per-fixture expected-failure sidecars by giving the newer model-generated corpora a fast count-based regression floor before we decide which articles deserve richer checked-in evidence assertions.

## Information Sources
- `.plans/2026-03-26-boilerplate-conclusion-expansion-plan.md` — scope and false-positive boundaries for the new family.
- `.worklogs/2026-03-26-181024-plan-next-llm-slop-families.md` — recent planning context for the polished-article slop gap.
- `fixtures/opus_4_6/*/article.md` and the other generated model corpora — real source material used to shape and verify the matcher.
- Existing `llm-slop` rule family files for structure and test style:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_04_boilerplate_framing.rs`
- `.worklogs/2026-03-26-174550-fix-sentence-case-cli-release.md` — current release/versioning baseline and the reminder to keep `cargo run -p prosesmasher` and `--version` green.

## Open Questions / Future Considerations
- `boilerplate-conclusion` is still intentionally narrow. The next likely extension points are “good news” reassurance closers and stronger motivational-uplift endings, but those should go through the same compare-first corpus audit before landing.
- The generated model fixture dirs were previously untracked local corpora. They are now useful enough to be treated as a live regression asset, but they will keep growing; future changes may need per-model filtering or a slower/parallel compare runner.
- `scripts/generated_fixture_failures.py` is currently count-based, not evidence-based. That is enough for “did we lose or gain rule hits?”, but not enough to guarantee the exact matched sentence stayed the same.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_08_boilerplate_conclusion.rs` — new rule implementation and matcher boundary.
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_08_boilerplate_conclusion.rs` — reusable assertion helpers for the new rule.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_08_boilerplate_conclusion_tests/synthetic.rs` — adversarial positives/negatives that define the intended rule edge.
- `scripts/generated_fixture_failures.py` — snapshot/compare workflow for the generated article corpus.
- `fixtures/opus_4_6/why_people_wake_up_tired/article.baseline.general-en.json` — concrete example of the new baseline sidecar format.
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical config surface; new `boilerplate_conclusion` field lives here.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON DTO mapping for `boilerplateConclusion`.
- `apps/prosesmasher/CHANGELOG.md` — `0.2.5` release note.
- `.worklogs/2026-03-26-181024-plan-next-llm-slop-families.md` — planning context for the polished article-slop expansion.
- `.worklogs/2026-03-26-174550-fix-sentence-case-cli-release.md` — latest pre-change release baseline.

## Next Steps / Continuation Plan
1. Use the generated corpus compare loop before every new `llm-slop` rule: snapshot current accepted state, implement the rule, compare, manually inspect every added hit, then refresh the baselines only if the delta is good.
2. Implement the next planned family from the existing notes, most likely `response-wrapper` expansion or `authority-padding`, and drive it from the same generated-article corpus instead of synthetic-only design.
3. If the count-only baselines start proving too weak, add a second sidecar layer for selected “gold” generated fixtures that records specific expected rule/evidence pairs the way the handwritten fixture corpus already does.
