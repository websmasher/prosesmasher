# Expand Generic Signposting Practical Frames

**Date:** 2026-03-27 08:41
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs`, generated/explainer `gpt_5_4_mini` baseline sidecars, workspace `Cargo.toml` files, `apps/prosesmasher/CHANGELOG.md`, `apps/prosesmasher/Cargo.lock`

## Summary
Expanded `generic-signposting` with the next narrow polished-article meta-framing batch: `the practical move is`, `the practical answer is`, `the practical version is`, and `the useful conclusion is simple`. Reviewed the additive hits in both the generated and explainer `gpt_5_4_mini` corpora, refreshed only those approved baselines, and released the workspace as `0.3.3`.

## Context & Problem
The previous `0.3.2` worklog explicitly called out the next likely `generic-signposting` misses:
- `the practical move is ...`
- `the practical answer is ...`
- `the useful conclusion is simple`

Those patterns had already appeared in reviewed fixtures and read like the same empty meta/compression family as:
- `the short answer is ...`
- `the short version ...`
- `the useful frame`

The goal for this pass was not to widen `generic-signposting` indiscriminately. It was to add only the next clearly same-family frames, then re-run the exact corpus review loop:
1. patch the matcher
2. run targeted tests
3. compare against the generated and explainer corpora
4. manually approve or reject additive hits
5. refresh only the approved sidecars

## Decisions Made

### Add four narrow practical/meta frame starts
- **Chose:** Add:
  - `the practical move is`
  - `the practical answer is`
  - `the practical version is`
  - `the useful conclusion is simple`
- **Why:** These are empty guidance/compression frames, not substantive claims. They behave like the already-owned `short answer` / `short version` family.
- **Alternatives considered:**
  - Add only the first three and leave `the useful conclusion is simple` for later — rejected because the generated corpus already contained a clean reviewed example and it fit the same family.
  - Broaden to looser `the practical ...` or `the conclusion is ...` patterns — rejected because that would immediately weaken the false-positive boundary.

### Keep `practical move` and `practical answer` as strong single-hit frames
- **Chose:** Route them through the existing strong single-hit meta-frame path rather than accumulative-only signposts.
- **Why:** These are not ordinary helpful transitions. They are short empty framing lines in the same way `the useful move is` and `the short answer is` already are.
- **Alternatives considered:**
  - Make them accumulative only — rejected because a single instance is already enough to be the bad pattern the user cares about.

### Allow concrete ordinary “practical move” prose to pass
- **Chose:** Add an explicit negative test for `Reducing temptation is another practical move ...`.
- **Why:** The new pattern should only match the sentence-initial meta frame, not any ordinary use of the words `practical move`.
- **Alternatives considered:**
  - Rely on the start-of-sentence phrase shape without a dedicated negative test — rejected because this is exactly the sort of boundary that drifts later if it is not pinned down.

### Keep the batch limited; do not add adjacent families in the same pass
- **Chose:** Leave out broader candidates such as:
  - `the catch is ...`
  - `the reality is ...`
  - `the mechanism is straightforward ...`
- **Why:** Those were previously noted as higher-risk and less clearly the same owned family. They need their own review cycle.
- **Alternatives considered:**
  - Bundle them into this patch because the compare loop was already running — rejected because that would mix a clean same-family extension with a fuzzier family boundary.

### Regenerate the lockfile after the workspace version bump
- **Chose:** Use `cargo generate-lockfile --manifest-path apps/prosesmasher/Cargo.toml` after the workspace version bump.
- **Why:** The blanket `0.3.2 -> 0.3.3` replacement also touched a third-party `0.3.2` lock entry. Regenerating the lockfile cleanly was safer than trying to hand-revert external entries.
- **Alternatives considered:**
  - Hand-edit `Cargo.lock` — rejected because it is fragile and easy to miss transitive entries.

## Architectural Notes
- No new rule family was created. This was a deliberate same-family extension of `generic-signposting`.
- The newly added starts were placed in the existing subfamilies:
  - `the practical move is` -> `question-frame`
  - `the practical answer is` / `the useful conclusion is simple` -> `answer-frame`
  - `the practical version is` -> `frame-signpost`
- The release remains additive at the public surface: one existing rule got wider, but no config shape changed.
- Baseline discipline stayed the same:
  - generated corpus compare had to go back to `[]`
  - explainer compare had to go back to `[]`

## Information Sources
- Prior worklog:
  - `.worklogs/2026-03-26-224601-implement-authority-padding-and-expand-signposting-conclusion.md`
- Reviewed corpus examples:
  - `fixtures/explainers/gpt_5_4_mini/why-adults-feel-tired-all-the-time.md`
  - `fixtures/explainers/gpt_5_4_mini/why-people-procrastinate.md`
  - `fixtures/gpt_5_4_mini/adult_procrastination_causes_and_fixes/article.md`
  - `fixtures/gpt_5_4_mini/stress_and_physical_symptoms/article.md`
- Existing compare tooling:
  - `scripts/generated_fixture_failures.py`
  - `scripts/explainer_fixture_failures.py`

## Open Questions / Future Considerations
- The next likely `generic-signposting` candidate is still `the useful conclusion is ...` without the `simple` tail, but that should only land if more reviewed examples accumulate.
- `the catch is ...` remains tempting, but it still looks too mixed: some hits are empty framing, others are legitimate methodological caveats.
- `authority-padding` and `boilerplate-conclusion` still have adjacent reviewed misses, but this pass intentionally did not touch them.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — matcher definitions for the new practical/meta frame starts
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs` — synthetic positives and the concrete negative that pins the boundary
- `fixtures/explainers/gpt_5_4_mini/why-adults-feel-tired-all-the-time.baseline.general-en.json` — reviewed additive `practical move` hit
- `fixtures/explainers/gpt_5_4_mini/why-people-procrastinate.baseline.general-en.json` — reviewed additive `practical answer` hit
- `fixtures/gpt_5_4_mini/adult_procrastination_causes_and_fixes/article.baseline.general-en.json` — reviewed additive `practical version` hit
- `fixtures/gpt_5_4_mini/stress_and_physical_symptoms/article.baseline.general-en.json` — reviewed additive `useful conclusion is simple` hit
- `.worklogs/2026-03-26-224601-implement-authority-padding-and-expand-signposting-conclusion.md` — prior release and the explicit continuation plan this pass executed

## Next Steps / Continuation Plan
1. Start the next candidate batch from the remaining reviewed misses, not from fresh intuition. The leading option is still a carefully truth-tabled `the catch is ...` family.
2. Before adding any broader `generic-signposting` patterns, search both `fixtures/gpt_5_4_mini` and `fixtures/explainers/gpt_5_4_mini` for concrete acceptable near-misses and pin them as synthetic negatives first.
3. Keep using the same release gate:
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher --model <each>`
   - `python scripts/explainer_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher --model gpt_5_4_mini`
