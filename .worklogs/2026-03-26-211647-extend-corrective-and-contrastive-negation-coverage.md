# Extend Corrective And Contrastive Negation Coverage

**Date:** 2026-03-26 21:16
**Scope:** `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_slogan_punchline.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_slogan_punchline_tests/synthetic.rs`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Extended the current rhetoric detectors to catch the next user-supplied negation cluster without broadening into generic prose. Released the workspace as `0.2.9` after verifying the new branches on targeted snippets, targeted unit tests, the full workspace suite, and all six generated-article regression buckets.

## Context & Problem
After `0.2.8`, the tool still missed a new batch of lines with corrective or contrastive negation:

- `Kids get kind in reps, not revelations.`
- `Bring a pattern, not a vibe.`
- `You do not want to crush leadership. You want to turn orders into invitations.`
- `Mostly by treating social skills like scripts, not virtues.`
- `Most new moms do not need more products. They need less load.`

The user explicitly agreed to skip a looser sentence:
- `that as a mental health issue too, not just a discipline issue at school.`

The central constraint remained the same as earlier negation work: we could not just detect `not X` more broadly, because that immediately drifts into legitimate explanatory prose. The solution had to preserve narrow family ownership and survive the generated-corpus regression gate unchanged.

## Decisions Made

### Keep repeated corrective prose in `negation-reframe`
- **Chose:** Extend `negation-reframe` with two additional narrow branches:
  - quantified human `do not need -> they need`
  - transformation-specific `do not want -> want to turn ... into ...`
- **Why:** Both are still corrective rhetoric, not a new family. The miss was a more specific subject/coreference shape, not a fundamentally different device.
- **Alternatives considered:**
  - Add a broad `you do not want -> you want` matcher — rejected because it would catch ordinary instructional prose like `You do not want to click twice. You want to save the draft.`
  - Add a generic `X do not need -> they need` matcher for any plural subject — rejected because it would swallow technical prose like `Most servers do not need more memory. They need better tuning.`

### Gate quantified `need` on human plural subjects
- **Chose:** Only allow the pronoun-followup branch when the first sentence subject looks like a quantified human plural, e.g. `most new moms`.
- **Why:** The user example is rhetorical coaching prose. The strongest practical false-positive risk was nonhuman technical subjects. The human-subject gate keeps the branch narrow while still covering the intended family.
- **Alternatives considered:**
  - Allow any quantified plural subject — rejected because it would catch acceptable technical recommendation prose.
  - Require exact `most new moms` phrasing — rejected because it would be too brittle and too tied to one example.

### Put the short `X, not Y` coaching lines under `slogan-punchline`
- **Chose:** Extend `slogan-punchline` with three narrow contrastive aphorism shapes:
  - imperative contrast: `Bring a pattern, not a vibe.`
  - human-plural reps contrast: `Kids get kind in reps, not revelations.`
  - `treating ... like X, not Y` moralized coaching contrast
- **Why:** These lines behave like sloganized morals or aphoristic coaching punchlines, not like the current `negation-reframe` family. Folding them into generic negation would destroy the boundary that previous false-positive audits already protected.
- **Alternatives considered:**
  - Broaden `negation-reframe` for any short `X, not Y` line — rejected because that is exactly the path that makes normal prose fail.
  - Create a brand-new rule family immediately — rejected because these lines fit the existing sloganized-punchline ownership boundary cleanly enough for now.

### Keep the short contrastive aphorisms intentionally phrase-tight
- **Chose:** Use shape-constrained, narrow contrastive branches instead of a looser structural detector.
- **Why:** The user asked to catch this new batch, but the project still wants near-zero false positives on desirable prose. The right trade-off here was to capture the emerging coaching-contrast family without pretending we have a safe general negation classifier.
- **Alternatives considered:**
  - Build a more abstract structural detector for short `X, not Y` clauses — rejected because the current corpus does not yet justify a safe, broad abstraction.

## Architectural Notes
- No new crates or rule families were introduced. This was deliberately an ownership-tightening pass, not a taxonomy expansion pass.
- `negation-reframe` remains in `cadence-patterns` because the new repeated `need`/`want` branches are still corrective cadence patterns.
- The short contrastive aphorisms live in `slogan-punchline`, preserving the idea that polished coaching slogans are a distinct `llm-slop` surface from ordinary corrective rhetoric.
- The generated-fixture baseline workflow remains the release gate for false-positive risk. This pass did not update any generated baselines because all six compare runs stayed empty.

## Information Sources
- User-provided lines from this session:
  - `Kids get kind in reps, not revelations.`
  - `Bring a pattern, not a vibe.`
  - `You do not want to crush leadership. You want to turn orders into invitations.`
  - `Mostly by treating social skills like scripts, not virtues.`
  - `Most new moms do not need more products. They need less load.`
- Existing generated regression corpora:
  - `fixtures/gpt_5_2_chat`
  - `fixtures/gpt_5_4`
  - `fixtures/gpt_5_4_mini`
  - `fixtures/haiku`
  - `fixtures/opus_4_6`
  - `fixtures/sonnet_4_6`
- Prior worklogs:
  - `.worklogs/2026-03-26-204645-expand-meta-framing-and-corrective-coverage.md`
  - `.worklogs/2026-03-26-200115-add-slogan-punchline-and-blame-reframe.md`
  - `.worklogs/2026-03-26-193317-empty-emphasis-and-generated-baseline-fix.md`

## Open Questions / Future Considerations
- The skipped line `... not just a discipline issue ...` still needs a dedicated truth table if we ever want to catch that family safely.
- The new `slogan-punchline` contrastive branches are intentionally narrow. If more real examples accumulate, revisit whether they form a broader but still safe “contrastive coaching aphorism” subtype.
- The `do not want -> want to turn ... into ...` branch is narrow by design. Do not broaden it to generic `you do not want -> you want` without a new false-positive audit over technical/product prose.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — new quantified-human need and transformation-specific want branches.
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — positives and negatives for the new corrective branches.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_slogan_punchline.rs` — new contrastive aphorism subtypes under slogan punchline.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_slogan_punchline_tests/synthetic.rs` — adversarial positives and near-miss negatives for the new short contrastive shapes.
- `apps/prosesmasher/CHANGELOG.md` — `0.2.9` release note.
- `.worklogs/2026-03-26-204645-expand-meta-framing-and-corrective-coverage.md` — immediately preceding release and false-positive posture.
- `.worklogs/2026-03-26-200115-add-slogan-punchline-and-blame-reframe.md` — original `slogan-punchline` ownership rationale.

## Next Steps / Continuation Plan
1. Build the next truth table from user-supplied misses that still remain uncaught, especially the looser `not just X` and `not during/after` coaching lines. Pair each candidate with at least one legitimate near-miss from the generated corpus before coding.
2. Keep using `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher --model <model>` as the first gate before any baseline rewrite. If a future rule adds hits, inspect every added hit manually before deciding whether they are desirable.
3. If the next uncaught family keeps mixing moralized aphorism and corrective rhetoric, prefer a small dedicated `llm-slop` rule over broadening `negation-reframe` again.
