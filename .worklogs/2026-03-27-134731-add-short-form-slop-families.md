# Add Short-Form Slop Families

**Date:** 2026-03-27 13:47
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/*`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/assertions/src/config_loader.rs`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `scripts/social_fixture_failures.py`, `fixtures/linkedin/**`, `fixtures/twitter/**`, `fixtures/instagram/**`, workspace `Cargo.toml` files, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added two new immediate `llm-slop` families for short-form AI writing: `lesson-framing` and `observer-guidance`. Also added a checked-in baseline workflow for the new LinkedIn/Twitter/Instagram corpora, reviewed the additive hits against those corpora, verified no diffs in the generated-article and explainer gates, and released the workspace as `0.3.4`.

## Context & Problem
The long-form article and explainer corpora were already catching a lot of polished prose slop, but the new short-form corpora exposed a different layer of synthetic writing:

- empty coaching wrappers like `The biggest lesson was simple.` and `The fix is boring, which is why it works.`
- empty reader-observation scaffolds like `You see it everywhere:`, `You can watch it happen in real time.`, `If this hits home, ...`, and `That is where the confusion slips in.`

The important constraint from the user was not just to match those phrases. The rules needed to stay narrow enough that acceptable technical or factual prose would not start failing just because it used words like `fix`, `watch`, or `where`.

That forced the work into two parallel tracks:
1. build a proper baseline/compare loop for the new social corpora before changing any matcher logic
2. implement the new rule families with explicit synthetic negatives that pin the false-positive boundary

## Decisions Made

### Add a dedicated short-form baseline workflow
- **Chose:** Add `scripts/social_fixture_failures.py` and snapshot all current LinkedIn/Twitter/Instagram fixture outputs before matcher changes.
- **Why:** The repo already had the same pattern for generated long-form corpora and the explainer corpus. The short-form fixtures were still untracked and had no regression floor, so there was no honest before/after check for the new families.
- **Alternatives considered:**
  - Reuse `generated_fixture_failures.py` directly — rejected because the directory layout is different (`fixtures/<corpus>/<model>/*.md` instead of `fixtures/<model>/*/article.md`).
  - Keep the review ad hoc and only inspect a few files manually — rejected because the user explicitly wanted the current error state recorded before broadening detection.

### Split the new misses into two rule families instead of stretching `generic-signposting`
- **Chose:** Add new `llm-slop` rules:
  - `lesson-framing`
  - `observer-guidance`
- **Why:** The short-form misses are not just more generic signposts. They are a distinct rhetorical function:
  - `lesson-framing` compresses a post into an empty “here is the lesson/fix” wrapper
  - `observer-guidance` tells the reader how to watch, notice, or interpret the scene before providing any real substance
- **Alternatives considered:**
  - Expand `generic-signposting` to own all of this — rejected because that would keep turning one rule into a grab bag for every empty meta sentence.
  - Fold the `That is where ...` lines into `empty-emphasis` — rejected because those lines are not deictic filler; they are faux causal/observer scaffolding.

### Keep `lesson-framing` narrow and evaluative, not any sentence starting with `The fix is`
- **Chose:** Match only:
  - `the biggest lesson was simple`
  - `the practical lesson for me was simple`
  - `the practical lesson was simple`
  - `the fix is ...` only when the sentence contains weak evaluative cues like `plain`, `boring`, `not heroic`, or `usually smaller than people want`
- **Why:** The checked-in corpora already contained acceptable concrete lines like:
  - `The fix is to initialize the parser before reading the file.`
  - `The fix is to engineer proximity artificially.`
  - `The fix is rarely just a new planner.`
  A broad `the fix is` matcher would immediately create false positives in both technical and non-sloppy explanatory prose.
- **Alternatives considered:**
  - Match any `the fix is ...` opener — rejected because the false-positive boundary is too weak.
  - Also catch `the way out is ...` in this pass — rejected because the reviewed examples were not enough to define a safe boundary. Technical or legitimate advisory prose can use that frame too.

### Keep `observer-guidance` tied to standalone scaffolds, not any longer sentence with the same prefix
- **Chose:** Treat these as immediate failures only when they are short standalone scaffolds:
  - `you see it everywhere`
  - `you see it after almost every`
  - `you can watch it happen in real time`
  - `you can tell the difference quickly`
  - `if this hits home`
  - `this is where people get stuck`
  - reviewed `that is where ...` abstract bridge lines such as `that is where the confusion slips in`
- **Why:** The first compile pass exposed exactly the failure mode we were worried about: the initial matcher also caught `You can watch it happen in real time on the dashboard.`, which is concrete and acceptable. Tightening the observer prompts to standalone scaffolds fixed that.
- **Alternatives considered:**
  - Match any sentence starting with `you can watch` / `you can tell` / `that is where` — rejected after the synthetic dashboard false positive.
  - Skip the `that is where ...` family entirely — rejected because several reviewed LinkedIn misses clearly belonged to that exact empty bridge pattern and stayed safe once the pattern list was kept abstract and narrow.

### Review social additive hits manually before refreshing any sidecars
- **Chose:** Keep the generated and explainer baselines unchanged, review the social additive hits sentence by sentence, and only then refresh the social sidecars.
- **Why:** The new families were built specifically from LinkedIn/Instagram misses, so social additivity was expected. The real risk was silent spread into the long-form corpora.
- **Alternatives considered:**
  - Bulk-refresh all baselines immediately after the code change — rejected because that would hide whether the new rules leaked into the established generated/explainer gates.
  - Require the social corpora to stay unchanged too — rejected because the purpose of the pass was to catch exactly the reviewed social misses.

### Update catalog-count tests instead of weakening them
- **Chose:** Raise the expected counts in `catalog/runtime/src/lib_tests/synthetic.rs` to reflect the two new checks.
- **Why:** The release gate correctly failed when the catalog tests still expected the old totals. The test intent is still valid; only the numbers changed.
- **Alternatives considered:**
  - Remove or loosen the count assertions — rejected because those tests are still useful integration checks for the published catalog surface.

## Architectural Notes
- The new rules stayed inside the `llm-slop` family and followed the established ownership pattern:
  - one runtime rule file
  - one assertions module
  - one sidecar `*_tests/mod.rs` with `synthetic.rs`
- `lesson-framing` and `observer-guidance` are immediate rules, not accumulative ones. These short-form scaffolds are noisy enough individually that a document-level count threshold would only hide the exact behavior we want to catch in LinkedIn and Instagram posts.
- The new social fixture workflow mirrors the earlier generated/explainer workflows. That keeps all real-corpus validation in the same shape:
  - snapshot current failures
  - compare after matcher changes
  - review additive hits
  - refresh only approved baselines

## Information Sources
- Reviewed short-form corpora:
  - `fixtures/linkedin/gpt_5_4/*.md`
  - `fixtures/linkedin/gpt_5_4_mini/*.md`
  - `fixtures/instagram/gpt_5_4/*.md`
  - `fixtures/instagram/gpt_5_4_mini/*.md`
- Existing regression workflows:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`
- Prior worklogs that set the release/review posture:
  - `.worklogs/2026-03-26-224601-implement-authority-padding-and-expand-signposting-conclusion.md`
  - `.worklogs/2026-03-27-084121-expand-generic-signposting-practical-frames.md`
- Subagent exploration:
  - Dalton’s short-form counterexample sweep confirmed why `the fix is ...` and `that is where ...` needed to stay narrow.

## Open Questions / Future Considerations
- The next likely short-form families are still the ones found in the LinkedIn/Instagram review but intentionally not implemented here:
  - validation / absolution framing
  - anti-glamour / boring-truth coaching
  - diagnostic-question scaffolding
  - possibly a tighter spatialized-causal-bridge expansion if more reviewed examples accumulate
- `lesson-framing` still deliberately does not own `the way out is ...` or every `the fix is ...` sentence. That is a likely future review area, but only after more concrete acceptable/sloppy pairs are collected.
- `observer-guidance` currently uses a narrow reviewed pattern list for the `that is where ...` bridge subtype. If more variants show up, that subtype may deserve its own rule rather than continued stretching.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_13_lesson_framing.rs` — runtime matcher for the new lesson/fix wrapper family
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_14_observer_guidance.rs` — runtime matcher for the new observer/address/where-bridge family
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_13_lesson_framing_tests/synthetic.rs` — synthetic positives and negatives that pin the lesson-framing boundary
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_14_observer_guidance_tests/synthetic.rs` — synthetic positives and the concrete dashboard/location negatives that pin observer-guidance
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical config surface with the two new heuristics toggles
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — DTO mapping for `lessonFraming` and `observerGuidance`
- `scripts/social_fixture_failures.py` — snapshot/compare workflow for LinkedIn/Twitter/Instagram fixtures
- `fixtures/linkedin/gpt_5_4/*.baseline.general-en.json` — reviewed additive short-form baseline examples for the new rules
- `fixtures/instagram/gpt_5_4_mini/*.baseline.general-en.json` — reviewed additive short-form baseline examples for the new rules
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs` — catalog surface counts that had to be updated for the new checks
- `.worklogs/2026-03-26-224601-implement-authority-padding-and-expand-signposting-conclusion.md` — prior real-corpus review workflow
- `.worklogs/2026-03-27-084121-expand-generic-signposting-practical-frames.md` — immediate predecessor release posture

## Next Steps / Continuation Plan
1. Mine the remaining reviewed LinkedIn/Instagram misses into the next one or two clearly bounded families before touching Twitter again.
   Read:
   - `fixtures/linkedin/gpt_5_4_mini/*.md`
   - `fixtures/instagram/gpt_5_4_mini/*.md`
   Prioritize the still-unhandled families listed above.
2. Before broadening `lesson-framing`, collect at least 3-5 acceptable counterexamples for `the way out is ...`, `the fix is ...`, and any new lesson wrapper candidate from the existing checked-in corpora, then pin them as synthetic negatives first.
3. Reuse the exact same release gate for future short-form passes:
   - `cargo fmt --manifest-path apps/prosesmasher/Cargo.toml --all`
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
   - `python scripts/explainer_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
   - `python scripts/social_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
4. If future short-form work starts adding too many single-line immediate rules, pause and re-evaluate whether some of them should become one higher-level family with clearer internal subtypes rather than a growing stack of micro-rules.
