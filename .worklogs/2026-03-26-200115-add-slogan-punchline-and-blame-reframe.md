# Add Slogan Punchline And Blame Reframe

**Date:** 2026-03-26 20:01
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/*`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `apps/prosesmasher/presets/full-config-en.json`, workspace `Cargo.toml` files, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added two new narrow `llm-slop` rules, `slogan-punchline` and `blame-reframe`, to catch the remaining user-supplied coaching-slop lines without broadening into generic abstract prose. Released the workspace as `0.2.7` after verifying the new rules against targeted synthetic negatives, the six user snippets, and all 60 generated article fixture baselines.

## Context & Problem
After `0.2.6`, the tool still missed a specific leftover cluster of sloganized coaching prose:
- `The rehearsal is the part that sticks.`
- `Daily life is enough for this. Daily life is the curriculum.`
- `It sounds small, and it changes everything.`
- `That is the part most families miss.`
- `... comes from development, not malice.`
- `Do it as skill-building instead of shame.`

The requirement was to catch the rest while not creating false positives on the generated article corpus. Earlier attempts to broaden `negation-reframe` into general `comes from x, not y` territory produced real false positives, so the solution had to preserve narrow ownership boundaries instead of stretching an existing rule until it became semantically muddy.

## Decisions Made

### Add `slogan-punchline` as a dedicated rule
- **Chose:** A new immediate `llm-slop` rule for short sloganized punchlines and one paired curriculum construction.
- **Why:** The remaining lines share a stable rhetorical function: they compress the advice into a slogan or moral punchline. They are not just another flavor of `boilerplate-conclusion` or `empty-emphasis`, and folding them into those rules would have made those boundaries worse.
- **Alternatives considered:**
  - Expanding `boilerplate-conclusion` — rejected because the new lines are not document-ending closers; they are local punchline sentences.
  - Expanding `empty-emphasis` — rejected because these lines carry more structure than deictic filler and would have made that rule too broad.

### Keep `slogan-punchline` deliberately stingy
- **Chose:** Only match:
  - `is the part that sticks`
  - `is the part most X miss`
  - short `it changes everything`
  - paired `X is enough for this/that` + `X is the curriculum`
- **Why:** The generated fixtures already contained near-miss legitimate lines like `Doing it is the part with rent.` and `That is the part people skip...`. The rule had to catch the sloganized family while explicitly leaving those alone.
- **Alternatives considered:**
  - Catching any `is the part ...` sentence — rejected because it immediately swallowed legitimate explanatory prose.
  - Catching any `changes everything` sentence — rejected because concrete technical uses are common and acceptable.

### Add `blame-reframe` instead of reviving broad corrective negation
- **Chose:** A second tiny rule for two narrow contrastive shapes:
  - causal source + `not <blame noun>`
  - `as <growth noun> instead of <blame noun>`
- **Why:** The remaining uncaught lines are not ordinary negation-reframes; they are coaching lines that replace blame with growth/diagnostic framing. A separate rule keeps that meaning explicit and avoids reopening the broader false-positive problem that already burned us.
- **Alternatives considered:**
  - Re-adding the old `comes from x, not y` branch to `negation-reframe` — rejected because it previously produced false positives in legitimate prose.
  - Catching any `instead of <noun>` contrast — rejected because the generated fixtures contain many legitimate `instead of` lines.

### Refuse broader `not malice` coverage
- **Chose:** Limit `blame-reframe` to growth/diagnostic terms like `development`, `learning`, `practice`, `skill-building`, not every abstract causal source.
- **Why:** A broader version would have started capturing ordinary abstract explanations instead of the narrow coaching slop we were targeting.
- **Alternatives considered:**
  - Treating all `... not malice` sentences as slop — rejected because the boundary became too opinionated and would have expanded the rule beyond the user’s specific examples.

### Use generated fixture compare as the main false-positive gate
- **Chose:** Keep the 60 generated article baselines unchanged and verify with `scripts/generated_fixture_failures.py compare` per model bucket.
- **Why:** The user clarified that additive hits are acceptable only if they are genuinely bad. The compare script provided the fastest deterministic check that the new rules were not silently expanding into the existing generated corpus.
- **Alternatives considered:**
  - Blindly snapshotting new baselines — rejected because it would hide regressions instead of proving safety.
  - Relying only on synthetic tests — rejected because the whole point was to keep the rules out of the real generated articles unless the additions were clearly justified.

## Architectural Notes
- Both new rules live inside `llm-slop` with full runtime/assertions/sidecar structure, preserving the owned-family rule architecture.
- `support.rs` gained adjacent-sentence traversal to support the paired curriculum matcher without leaking that traversal into unrelated rule files.
- Config and preset exposure follow the existing pattern: `HeuristicsConfig`, DTO mapping, preset assets, and catalog counts all move together.
- The release remained additive at the architecture level: no family boundaries changed and no existing rules were overloaded to absorb semantically different patterns.

## Information Sources
- User-provided uncaught lines in the chat, especially:
  - `The rehearsal is the part that sticks.`
  - `Do it as skill-building instead of shame.`
  - `Daily life is the curriculum.`
- Existing generated fixture regression corpus under `fixtures/{gpt_5_2_chat,gpt_5_4,gpt_5_4_mini,haiku,opus_4_6,sonnet_4_6}`
- `scripts/generated_fixture_failures.py` for pre/post behavior comparison
- Prior worklog:
  - `.worklogs/2026-03-26-193317-empty-emphasis-and-generated-baseline-fix.md`

## Open Questions / Future Considerations
- `blame-reframe` is intentionally narrower than the broader family of moralized coaching contrasts. If more real examples accumulate, build that family from additional real hits rather than by widening this rule ad hoc.
- `slogan-punchline` may eventually deserve one more pattern family for short “That is the trick / That is the whole game” style lines, but only if those show up in real fixtures and survive the same false-positive gate.
- The generated compare script is useful enough that it may deserve a packaged CLI or CI wrapper later instead of living as a standalone helper.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_slogan_punchline.rs` — new punchline matcher and paired-sentence curriculum logic
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_11_blame_reframe.rs` — new narrow blame-to-growth matcher
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/support.rs` — family-local evidence traversal helpers, including adjacent sentence collection
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_slogan_punchline_tests/synthetic.rs` — adversarial positives and near-miss negatives for the punchline rule
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_11_blame_reframe_tests/synthetic.rs` — adversarial positives and negatives for blame reframe
- `scripts/generated_fixture_failures.py` — generated article baseline compare mechanism used as the false-positive gate
- `apps/prosesmasher/CHANGELOG.md` — `0.2.7` release note
- `.worklogs/2026-03-26-193317-empty-emphasis-and-generated-baseline-fix.md` — prior release context and the compare-script background

## Next Steps / Continuation Plan
1. Review more uncaught lines from the generated corpora and group them by rhetorical function before adding any more rules. Do not broaden `slogan-punchline` or `blame-reframe` until real grouped examples demand it.
2. If a new leftover family emerges, prototype it as a dedicated `llm-slop` rule with hostile synthetic negatives first, then gate it with `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher --model <model>`.
3. If a future rule adds good real hits to the generated corpus, refresh only the affected baseline sidecars after manual review of the new failures rather than bulk-resnapshotting everything.
