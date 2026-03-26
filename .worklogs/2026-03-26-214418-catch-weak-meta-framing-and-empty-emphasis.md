# Catch Weak Meta Framing And Empty Emphasis

**Date:** 2026-03-26 21:44
**Scope:** `apps/prosesmasher/CHANGELOG.md`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, workspace `apps/prosesmasher/**/Cargo.toml`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs`, `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.baseline.general-en.json`

## Summary
Tightened two remaining slop families the user explicitly cared about: weak meta-framing lines such as `The useful move is ...` and short deictic significance lines such as `That is still real change.` / `That is how the pattern weakens.` The workspace was bumped to `0.3.1`, the generated-fixture compare was re-run, and the one reviewed additive `gpt_5_4_mini` baseline hit was snapshotted.

## Context & Problem
After `0.3.0`, the user re-ran the CLI on fresh snippet bundles and found that most of the intended slop was now caught, but two narrow families still slipped through:

- weak meta-framing like `The useful move is to respect the intensity mismatch.`
- empty deictic emphasis like `That is still real change.` and `That is how the pattern weakens.`

The important constraint was not merely to catch them synthetically, but to verify that the changes did not destabilize the generated 60-article corpus. The user explicitly accepted additive new catches as long as the new catches were genuinely bad and reviewed, rather than requiring the corpus to stay unchanged.

## Decisions Made

### Promote Some Generic Signposts To Immediate Failures
- **Chose:** Treat the question/answer/sequence meta frames inside `generic-signposting` as strong framing that fails even when observed only once.
- **Why:** `The useful move is ...`, `The useful question is ...`, `The answer is simple.`, and `A simple sequence works well:` are empty guidance scaffolds rather than ordinary transitions. Keeping them purely accumulative was exactly why the user's snippet still passed.
- **Alternatives considered:**
  - Add a separate new rule for weak framing — rejected because the existing `generic-signposting` rule already owns empty rhetorical scaffolding and only needed a sharper split between strong and weak subfamilies.
  - Keep everything accumulative — rejected because that preserved the false negative the user pointed out.

### Keep Note/Consultation Frames Accumulative
- **Chose:** Leave `important-to`, `please note`, and consultation-style signposts as accumulative only.
- **Why:** Those can still appear legitimately in otherwise solid prose. The user's complaint was specifically about empty meta guidance frames, not every stock transition.
- **Alternatives considered:**
  - Make all signpost families immediate — rejected because that would widen false-positive risk with no evidence that the user wanted that.

### Extend Empty Emphasis Only With Sentence-Exact Deictic Magnifiers
- **Chose:** Add two narrow `empty-emphasis` subpatterns:
  - `That/This is still real change.`
  - `That/This is how the pattern/cycle/loop weakens.`
- **Why:** These are short standalone significance lines with almost no informational payload. The exact sentence-boundary shape makes them high-signal and keeps longer explanatory prose out.
- **Alternatives considered:**
  - Build a broad `That is how ...` matcher — rejected because it would catch perfectly normal explanatory prose.
  - Ignore them as merely stylistic — rejected because the user explicitly identified them as target slop.

### Review The One New Generated-Corpus Hit And Snapshot It
- **Chose:** After the matcher change produced one new generated-fixture difference in `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.md`, review that exact hit and then refresh the `gpt_5_4_mini` snapshot set.
- **Why:** The new hit was `The useful question is not “What is wrong with me?”`, which fits the newly targeted empty guidance framing. Once reviewed, it should become part of the baseline rather than linger as an unexplained diff.
- **Alternatives considered:**
  - Refuse any generated-corpus additions — rejected because the user already said unchanged baselines are not required if the new catches are genuinely bad.
  - Snapshot all model buckets blindly — rejected because only `gpt_5_4_mini` changed and the rest of the corpus already compared cleanly.

## Architectural Notes
- `generic-signposting` is now a hybrid rule:
  - weak signpost families still use the `maxPerDocument` accumulative behavior
  - strong meta frames (`question-frame`, `answer-frame`, `sequence-frame`) fail immediately
- This is acceptable because the split is internal to the rule's owned rhetorical family; it does not create overlap with another rule.
- `empty-emphasis` remains an immediate rule with sentence-exact deictic filler patterns. The new subpatterns stay intentionally narrow to avoid leaking into explanatory writing.
- The generated baseline tool remains the practical regression backstop: matcher changes are reviewed against the six model buckets and only snapshotted when the additive hits are explicitly judged good.

## Information Sources
- User-provided missed snippet set after `0.3.0`
- Existing rule files:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs`
- Existing synthetic tests for those rules
- Generated-corpus verification and reviewed article:
  - `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.md`
  - `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.baseline.general-en.json`
- Prior worklog:
  - `.worklogs/2026-03-26-213044-rename-slogan-punchline-to-contrastive-aphorism.md`
- Commands run during this pass:
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml -p prosesmasher-app-checks-llm-slop-runtime generic_signposting -- --nocapture`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml -p prosesmasher-app-checks-llm-slop-runtime empty_emphasis -- --nocapture`
  - `cargo build --manifest-path apps/prosesmasher/Cargo.toml -p prosesmasher`
  - `apps/prosesmasher/target/debug/prosesmasher check /tmp/prosesmasher_meta_gap.md --preset general-en --format json`
  - `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
  - `python scripts/generated_fixture_failures.py snapshot --binary apps/prosesmasher/target/debug/prosesmasher --model gpt_5_4_mini`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - `cargo run --manifest-path apps/prosesmasher/Cargo.toml -q -p prosesmasher -- --version`

## Open Questions / Future Considerations
- `That is still real change.` and `That is how the pattern weakens.` are now covered, but there are adjacent deictic magnifier families (`That is the shift.`, `That is what sticks.` etc.) that may deserve future review rather than immediate generalization.
- `generic-signposting` now fails some single-use meta frames by design. If later corpora show legitimate uses in strong prose, the first refinement point is likely narrowing the `question-frame` family, not backing out the whole strong/weak split.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — strong-vs-accumulative signposting logic
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs` — synthetic boundary tests for single-use question/move frames
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs` — deictic significance matcher
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs` — positive/negative cases for the new deictic lines
- `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.baseline.general-en.json` — reviewed additive generated-corpus baseline update
- `apps/prosesmasher/CHANGELOG.md` — `0.3.1` release notes
- `.worklogs/2026-03-26-213044-rename-slogan-punchline-to-contrastive-aphorism.md` — immediate predecessor that introduced `0.3.0`

## Next Steps / Continuation Plan
1. Keep using new snippet bundles to identify misses, but review each proposed family against the generated-model corpus before expanding.
2. The next likely gap is still the deictic/magnifier family around `That is the part/shift/change ...` lines; if more examples appear, collect at least 3-5 variants before widening `empty-emphasis`.
3. If new single-use meta frames keep showing up, consider whether `generic-signposting` should explicitly separate strong meta framing into named subfamilies rather than using one rule with internal strong/weak branches.
