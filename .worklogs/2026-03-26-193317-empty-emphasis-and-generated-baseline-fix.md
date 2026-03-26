# Add Empty Emphasis And Repair Generated Fixture Baselines

**Date:** 2026-03-26 19:33
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/*`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `scripts/generated_fixture_failures.py`, generated fixture baseline sidecars under `fixtures/{gpt_5_2_chat,gpt_5_4,gpt_5_4_mini,haiku,opus_4_6,sonnet_4_6}/*/article.baseline.general-en.json`, `apps/prosesmasher/Cargo.toml`, workspace crate `Cargo.toml` files, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added `empty-emphasis` as a new `llm-slop` rule for short deictic filler lines, expanded `negation-reframe` with safe repeated corrective families, and fixed the generated-fixture regression tool so it records real observed hit counts instead of collapsing every failing rule to `1`. During the audit I also tried a broader `comes from x, not y` negation branch, proved it produced false positives on legitimate prose, and removed it before shipping.

## Context & Problem
The repo was left in a dirty experimental state after trying to catch a new batch of uncaught slop snippets. The user supplied new misses like:

- `The goal is not zero tears. The goal is learning...`
- `You do not need to sell school as magical. You need to make it familiar and survivable.`
- `That last part matters.`

At the same time, the generated-fixture baseline tool turned out to have a real defect: it only counted failure *presence* per rule, not actual observed hits. That made the generated article baselines too weak for matcher iteration, especially for high-count rules like `smart-quotes`, `colon-dramatic`, and `paragraph-length`.

The immediate task was not just “add more detection.” It was to:

- keep the safe new rule work
- reject the unsafe matcher branch that overfit rhetorical source-contrast prose
- repair the generated regression baseline so it can be trusted
- release the repo in a clean, verifiable state instead of leaving another dirty tree behind

## Decisions Made

### Add `empty-emphasis` as a narrow immediate `llm-slop` rule
- **Chose:** Add `empty-emphasis` as a simple English-only immediate rule for short deictic filler lines like `That last part matters.` and `This part matters.`
- **Why:** These lines are structurally low-information magnifiers. They are short, self-important, and common in slop-y article rhetoric, but they do not belong inside the existing `negation-reframe` family.
- **Alternatives considered:**
  - Fold them into `generic-signposting` — rejected because the shape is not signposting or transition language; it is short empty emphasis.
  - Build a broader slogan/magnification family immediately — rejected because only the deictic `part/bit matters` subtype had a clean boundary today.

### Expand `negation-reframe` only with repeated corrective families that survived review
- **Chose:** Keep the new repeated corrective branches for:
  - `The goal is not ... The goal is ...`
  - `The point is not ... The point is ...`
  - `You do not need ... You need ...`
  - the same patterns with a tiny interrupt sentence in the middle
- **Why:** These are strong rhetorical corrective structures, and the user explicitly identified them as misses. Synthetic tests and direct snippet runs showed they behave like the existing family instead of broadening it into ordinary prose.
- **Alternatives considered:**
  - Leave `negation-reframe` unchanged and create a second family for repeated corrective framing — rejected because these are plainly the same rhetorical device the rule already owns.
  - Broaden inline `x, not y` much more aggressively — rejected because it quickly swallows legitimate contrasts.

### Reject the experimental `comes from x, not y` branch
- **Chose:** Remove the source-contrast inline matcher before shipping.
- **Why:** It looked attractive because it caught the user’s `... comes from development, not malice.` example, but the false-positive audit showed it also flagged legitimate explanatory prose such as:
  - `Part of the pleasure comes from imagining the result, not from doing the work.`
  - `Confidence grows from successful repetition, not from chanting at a mirror.`
- **Alternatives considered:**
  - Keep it because the motivating example felt slop-y — rejected because the false positives were real and high-quality prose could easily use the same structure.
  - Try to salvage it with a tiny blacklist of bad RHS nouns like `malice` — rejected because that would turn the rule into ad hoc phrase curation without a convincing general boundary.

### Fix the generated-fixture baseline tool to count real observed hits
- **Chose:** Change `scripts/generated_fixture_failures.py` so a rule’s count is based on `observed` when available, or on evidence length as fallback, instead of incrementing each failing rule once.
- **Why:** The old tool undercounted every multi-hit rule and made the generated baselines misleading. A file with 15 `colon-dramatic` hits and a file with 1 `colon-dramatic` hit were both recorded as `1`.
- **Alternatives considered:**
  - Leave the script alone and treat it as a coarse smoke check — rejected because the entire point of the generated corpus is incremental matcher verification, and presence-only counts are too weak for that.
  - Replace the count baseline with full evidence snapshots immediately — rejected because that is a larger format migration than needed to repair the current broken contract.

### Refresh all generated article baselines to the stricter count contract
- **Chose:** Regenerate all 60 `article.baseline.general-en.json` sidecars after fixing the script, then verify `compare` returns `[]`.
- **Why:** Once the tool was fixed, the old sidecars were wrong by construction. Keeping them would leave the repo permanently dirty and the regression workflow unreliable.
- **Alternatives considered:**
  - Only refresh the few sidecars touched by the new rules — rejected because the script bug affected every generated article baseline, not just the files with new rule hits.

### Release as `0.2.6`
- **Chose:** Bump the full workspace and internal crate graph to `0.2.6`, update the changelog, regenerate `Cargo.lock` properly, and verify `prosesmasher --version`.
- **Why:** The repo convention is to ship each completed build as a semantic version, and this change includes new CLI-visible behavior plus a repaired regression asset set.
- **Alternatives considered:**
  - Leave the workspace at `0.2.5` and only commit the local fixups — rejected because the user explicitly wants versioned CLI releases for each build.
  - Keep the naively text-rewritten lockfile — rejected because it corrupted an external dependency version (`displaydoc`) and broke `cargo run`.

## Architectural Notes
This change keeps the current family boundaries intact:

- `empty-emphasis` lives in `llm-slop`
- repeated corrective rhetoric stays in `cadence-patterns/negation-reframe`
- generated corpus verification stays in `scripts/` because it validates the whole CLI/checking pipeline, not one family

The most important architectural choice was *not* expanding `negation-reframe` just because the motivating example felt right. The false-positive audit forced a harder rule: rhetorical source-contrast is still too semantically broad for a safe deterministic branch. That preserves the project’s evidence-first, near-zero-false-positive bar instead of letting one compelling snippet deform the family boundary.

The baseline script repair is also architectural, not cosmetic. The generated corpora are now trustworthy as a regression floor because they record actual observed hit counts. That gives future matcher work a meaningful pre/post diff instead of a yes/no flag per rule.

## Information Sources
- User-provided missed snippets from this session — especially the `goal is not`, `do not need`, and `That last part matters` examples.
- `.worklogs/2026-03-26-183813-add-boilerplate-conclusion-and-generated-fixture-baselines.md` — previous generated baseline workflow and release context.
- `.worklogs/2026-03-26-181024-plan-next-llm-slop-families.md` — recent `llm-slop` planning context.
- Generated article corpora under `fixtures/{gpt_5_2_chat,gpt_5_4,gpt_5_4_mini,haiku,opus_4_6,sonnet_4_6}` — used both for true-positive discovery and false-positive review.
- Direct spot checks against concrete corpus lines, including legitimate near-misses such as:
  - `Part of the pleasure comes from imagining the result, not from doing the work.`
  - `Confidence grows from successful repetition, not from chanting at a mirror.`
- Existing rule structure and assertions patterns in:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_08_boilerplate_conclusion.rs`
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`

## Open Questions / Future Considerations
- The uncaught slogan/magnification snippets still need a real family. The current safe `empty-emphasis` rule only owns the narrow `this/that [last] part/bit matters` subtype, not broader lines like:
  - `The rehearsal is the part that sticks.`
  - `Daily life is the curriculum.`
  - `It sounds small, and it changes everything.`
- If we later want to catch source-contrast rhetoric like `... not malice`, it should likely be a different family with a much stronger semantic boundary, not a further broadening of `negation-reframe`.
- The generated baseline script is still count-based, not evidence-based. It is now correct at the count level, but selected “gold” fixtures may still deserve richer checked-in evidence contracts.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis.rs` — new rule implementation for short empty emphasis lines.
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_09_empty_emphasis.rs` — reusable assertions for `empty-emphasis`.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_09_empty_emphasis_tests/synthetic.rs` — positive and negative boundary tests for the new rule.
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — expanded but constrained corrective rhetoric matcher.
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — added repeated-frame and interrupted corrective coverage.
- `scripts/generated_fixture_failures.py` — fixed generated-corpus snapshot/compare logic; now counts real observed hits.
- `fixtures/gpt_5_2_chat/why_couples_stop_communicating/article.baseline.general-en.json` — concrete example of the refreshed count-based generated baseline format.
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical config surface; includes the new `empty_emphasis` toggle.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON mapping for `emptyEmphasis`.
- `apps/prosesmasher/CHANGELOG.md` — `0.2.6` release note.
- `.worklogs/2026-03-26-183813-add-boilerplate-conclusion-and-generated-fixture-baselines.md` — prior generated-baseline and release workflow.

## Next Steps / Continuation Plan
1. Design the next narrow rule for the remaining slogan/magnification cluster. Start from the uncaught snippets above and build a truth table before coding.
2. When adding the next `llm-slop` rule, use the repaired `generated_fixture_failures.py` loop from the start:
   - compare current baselines
   - inspect every added hit
   - only then refresh the sidecars
3. If count-only generated baselines stop being enough for controversial rules, add evidence-based “gold” sidecars for a smaller curated subset of generated fixtures instead of making all 60 fixtures fully evidence-driven at once.
