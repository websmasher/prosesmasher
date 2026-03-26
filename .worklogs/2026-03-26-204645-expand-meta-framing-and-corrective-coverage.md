# Expand Meta Framing And Corrective Coverage

**Date:** 2026-03-26 20:46
**Scope:** `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Expanded three existing rule families to catch the next batch of user-supplied coaching-slop lines without widening into generic prose. Released the workspace as `0.2.8` after verifying the new branches against targeted synthetic negatives, the user snippets, the full workspace test suite, and the six generated-article baseline buckets.

## Context & Problem
After `0.2.7`, several user-supplied lines were still uncaught even though they were clearly in-scope:

- `The child does not need ... The child needs ...`
- `The answer is simple.`
- `The useful question is ...`
- `A simple sequence works well:`
- `That one change helped a lot.`
- `This is telling you something.`

At the same time, the user explicitly did not want broad cliché detection and had already rejected wider experiments that pulled in legitimate prose. That forced a narrow question: which of these lines belong to existing families, and how can those families be extended without creating new hits in the 60 generated article fixtures unless those hits are genuinely bad?

## Decisions Made

### Extend `negation-reframe` only for repeated noun-subject corrective pairs
- **Chose:** Add a repeated noun-subject branch for `X does not need ... X needs ...`.
- **Why:** `The child does not need ... The child needs ...` is plainly the same corrective rhetoric already owned by `negation-reframe`. The missing piece was not a new family; it was support for a repeated noun phrase instead of only pronoun-led `you/we/they` patterns.
- **Alternatives considered:**
  - Add a broad inline `not X, but Y` matcher — rejected because earlier experiments already produced false positives in normal explanatory prose.
  - Create a separate “parenting corrective” rule — rejected because this is not parenting-specific; it is the same rhetorical corrective structure with a different subject form.

### Treat the new meta lines as `generic-signposting`, not a new family
- **Chose:** Expand `generic-signposting` with three narrow meta-framing families:
  - `question-frame`
  - `answer-frame`
  - `sequence-frame`
- **Why:** `The answer is simple.`, `The useful question is ...`, and `A simple sequence works well:` are all low-information staging lines. They are not conclusions, slogans, or empty emphasis; they are meta frames around the content.
- **Alternatives considered:**
  - Create a dedicated `meta-framing` rule — rejected because the lines fit the existing `generic-signposting` ownership boundary cleanly enough.
  - Fold them into `boilerplate-framing` — rejected because these are not topic setup frames like `when it comes to`; they are generic narrative/argument signposts.

### Keep `generic-signposting` accumulative
- **Chose:** Leave the default threshold behavior intact so one isolated `question-frame` does not fail by itself.
- **Why:** Several of these frames are only convincingly slop when repeated or paired with another signposting move. This preserved the low false-positive posture while still catching the user’s bundled examples.
- **Alternatives considered:**
  - Make the new frames immediate — rejected because a single `The useful question is ...` line can occur in legitimate prose.

### Expand `empty-emphasis` only for deictic filler variants
- **Chose:** Add:
  - `That one change helped a lot.`
  - `This is telling you something.`
  as new deictic-only `empty-emphasis` patterns.
- **Why:** These are the same kind of self-important, low-information emphasis as `That last part matters.` The safe boundary is the deictic subject (`this` / `that`), not the broader verb phrase.
- **Alternatives considered:**
  - Match any `X is telling you something` line — rejected because generated fixtures already contain concrete acceptable lines like `Your body is telling you something ...`.
  - Move them into `slogan-punchline` — rejected because they do not function as sloganized morals; they are empty emphasis.

### Use the generated article baselines as the release gate
- **Chose:** Keep the six generated model corpora as the main false-positive floor and refuse any extra broadening once all compares stayed clean.
- **Why:** The user’s bar was not “never change baseline counts.” It was “catch bad slop, do not drift into desirable prose.” The generated corpus is the fastest deterministic check we have for that today.
- **Alternatives considered:**
  - Broaden further into lines like `Problem-solving lands after the storm, not during it.` in the same pass — rejected because those still need a tighter family boundary and should not piggyback on these narrower expansions.

## Architectural Notes
- No new family crates were added. This pass deliberately strengthened existing family ownership instead of creating another tiny rule family.
- `negation-reframe` remains in `cadence-patterns`; the new branch is still a cadence/corrective rhetoric pattern, not `llm-slop`.
- `generic-signposting` and `empty-emphasis` remain in `llm-slop`; both gained more precise subtypes rather than broader bag-of-phrases matching.
- Assertions/test shape stayed intact: all changes are covered through sidecar synthetic tests and the shared runtime behavior remains exposed only through the existing rule surfaces.

## Information Sources
- User-provided uncaught lines from this session, especially:
  - `The child does not need ... The child needs ...`
  - `The answer is simple.`
  - `The useful question is ...`
  - `A simple sequence works well:`
  - `That one change helped a lot.`
  - `This is telling you something.`
- Explorer subagent findings from this session:
  - repeated noun-subject corrective is safe inside `negation-reframe`
  - meta framing belongs under existing signposting/emphasis families
- Generated fixture regression corpora under:
  - `fixtures/gpt_5_2_chat`
  - `fixtures/gpt_5_4`
  - `fixtures/gpt_5_4_mini`
  - `fixtures/haiku`
  - `fixtures/opus_4_6`
  - `fixtures/sonnet_4_6`
- Prior worklogs:
  - `.worklogs/2026-03-26-193317-empty-emphasis-and-generated-baseline-fix.md`
  - `.worklogs/2026-03-26-200115-add-slogan-punchline-and-blame-reframe.md`

## Open Questions / Future Considerations
- Several lines are still intentionally uncaught:
  - `It changes the scene. It does not build the skill.`
  - `Problem-solving lands after the storm, not during it.`
  - `Prevention lives in rehearsal, not in the lecture ...`
  - `So the first fixes are boring.`
  These need a separate family or a much tighter truth table before landing.
- `generic-signposting` is now closer to a generic meta-framing bucket. If future additions start mixing too many rhetorical functions, it may need a split, but the current boundary is still coherent.
- The generated compare script remains count-based. That is enough for this release gate, but controversial future changes may need evidence-level sidecars for selected generated articles.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — repeated noun-subject corrective matcher.
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — new positive/negative coverage for repeated noun-subject corrective lines.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — new question/answer/sequence meta-frame coverage.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs` — confirms the new frames only fail once repeated.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs` — deictic `change helped` and `telling you something` additions.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs` — concrete-subject negative coverage that keeps the new branch narrow.
- `apps/prosesmasher/CHANGELOG.md` — `0.2.8` release note.
- `.worklogs/2026-03-26-193317-empty-emphasis-and-generated-baseline-fix.md` — prior `empty-emphasis` and generated-baseline context.
- `.worklogs/2026-03-26-200115-add-slogan-punchline-and-blame-reframe.md` — prior narrow-slop release and false-positive posture.

## Next Steps / Continuation Plan
1. Build a truth table for the still-uncaught contrastive coaching lines before touching `negation-reframe` again. Start from the four lines listed in Open Questions and explicitly pair them with legitimate near-misses from the generated corpus.
2. If a next rule is proposed, run `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher --model <model>` across all six generated buckets before snapshotting any new baseline state.
3. Prefer extending existing families only when the rhetorical function is clearly the same. If the next leftover cluster is moralized coaching contrast rather than meta framing or empty emphasis, give it its own small `llm-slop` rule instead of deforming `generic-signposting` or `negation-reframe`.
