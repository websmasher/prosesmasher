# Release 0.1.4 Heuristic Tightening

**Date:** 2026-03-23 17:40
**Scope:** `apps/prosesmasher/crates/app/core/src/quality/heuristics/*`, `apps/prosesmasher/crates/domain/types/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`, preset JSON, CLI output/tests, workspace versioning and changelog

## Summary
This release tightens several high-signal rhetorical heuristics and ships them as `0.1.4`. It adds a new `fragment-stacking` detector, broadens affirmation-closer coverage to short `That's the ...` formulas, narrows corrective-negation detection around a curated same-root framing-verb family, and updates the `substack-en` preset to a shorter structural envelope.

## Context & Problem
Recent real-text review surfaced several misses and one modeling problem in the existing heuristic layer:

- clipped cadence runs like `Skipped snack. Too much noise. Weird nap.` were not represented directly
- affirmation-closer detection missed short standalone formulas like `That's the whole repair.`
- the corrective-negation family needed to catch `Not to X. To Y.` and `does not mean / it means`
- the first pass at fragment detection overfit overlapping triples instead of detecting a single cadence run
- the first pass at `does not mean / it means` was too literal and needed a product-level abstraction around shared framing verbs, with false-positive pressure from ordinary engineering or technical prose

The goal for this slice was to ship the new heuristics, keep the JSON surface clean, and avoid widening the detector family into generic explanatory prose.

## Decisions Made

### Added `fragment-stacking` as a run detector, not a sliding-window counter
- **Chose:** detect contiguous runs of clipped fragment sentences and emit one evidence item per run.
- **Why:** the actual style problem is cadence, not a specific `3`-sentence window. Overlapping triples produced noisy duplicate findings and still missed the `short, short, long` payoff shape.
- **Alternatives considered:**
  - Sliding triples only — rejected because it double-counted the same passage and overfit the surface shape.
  - Paragraph-level aggregate score — rejected because it would lose the exact run boundary needed for rewrite guidance.

### Broadened affirmation closers to short `That's the ...` formulas
- **Chose:** keep the explicit shipped closer phrases and add short sentence-level formulas starting with `that's the` / `that is the`.
- **Why:** the product should catch obvious stock cadence like `That's the whole repair.` even when it is not a literal section-ending canned string.
- **Alternatives considered:**
  - Add one more exact phrase — rejected because the family is clearly broader than a single string.
  - Catch every `that's the` sentence — rejected because shortness is an important guardrail.

### Generalized `does not mean / it means` into curated same-root framing verbs
- **Chose:** detect shared-root corrective framing only for a curated set of verbs: `mean`, `reflect`, `indicate`, `signal`, `suggest`.
- **Why:** the real pattern is negated framing followed by positive reframing with the same verb root. Limiting the verb family prevents the check from swallowing generic same-root prose like `does not make / it does make`.
- **Alternatives considered:**
  - Keep a hardcoded `mean` branch — rejected because the abstraction was obviously broader.
  - Match any `does not X / it Xs` pair — rejected because it would overfire on legitimate explanatory prose and technical writing.
  - Remove same-root handling entirely and keep only copular `not X / it's Y` — rejected because that misses a prominent slop family already seen in the wild.

### Kept `substack-en` quality shared and changed only structure
- **Chose:** update `substack-en` word count to `500..1000` without preset-specific quality divergence.
- **Why:** preset differences are structural, not stylistic; `substack` needed the shorter envelope while still sharing the same quality defaults as the other shipped presets.
- **Alternatives considered:**
  - Introduce looser/tighter quality knobs for `substack` — rejected because it would break the current preset philosophy.
  - Leave the old longer range — rejected because it no longer matched intended use.

## Architectural Notes
The release keeps the current product boundary intact:

- heuristics remain library-owned and enabled by default
- presets express structural policy only
- JSON output stays deterministic and minimal

The new `fragment-stacking` check lives in the heuristics family and is wired through the canonical config and loader so the shipped full config remains authoritative. The corrective-negation work stays inside the existing `negation-reframe` check rather than splitting the family into separate user-facing checks; this preserves CLI stability while improving the internal matcher.

## Information Sources
- Local fixture review:
  - `fixtures/tantrums-science.md`
  - `fixtures/stay-calm.md`
- Existing heuristic implementation and tests:
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs`
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/affirmation_closers.rs`
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/mod.rs`
- Current preset and canonical config model:
  - `apps/prosesmasher/presets/substack-en.json`
  - `apps/prosesmasher/presets/full-config-en.json`
  - `apps/prosesmasher/crates/domain/types/src/config.rs`
  - `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`
- Prior release/context worklogs:
  - `.worklogs/2026-03-23-161101-tighten-cli-contract-and-docs.md`
  - `.worklogs/2026-03-23-154505-prepare-and-release-0-1-1.md`

## Open Questions / Future Considerations
- `negation-reframe` still catches plain inline `X, not Y` engineering prose by design; that may deserve a future narrowing toward interpretive/category contrasts only if false positives accumulate.
- `fragment-stacking` has only been sanity-checked on the current prose fixtures; broader corpus review may reveal other terse-but-legitimate registers worth exempting.
- Root-level untracked `fixtures/` were used for local validation but are not part of the shipped release.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/fragment_stacking.rs` — new cadence-run detector and evidence model
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs` — corrected same-root framing-verb logic and adjacent/inline branches
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/affirmation_closers.rs` — broader short `That's the ...` closer handling
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/mod.rs` — active heuristic registry
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical heuristic config shape
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — JSON loader mapping for the new heuristic config
- `apps/prosesmasher/presets/substack-en.json` — updated structural preset range
- `apps/prosesmasher/CHANGELOG.md` — released product surface summary
- `.worklogs/2026-03-23-161101-tighten-cli-contract-and-docs.md` — previous CLI/release contract work

## Next Steps / Continuation Plan
1. Publish `0.1.4` across the full crate graph after this commit and verify the installed binary reflects the new version.
2. Re-run installed-binary smoke checks on the shipped preset path, especially `--preset substack-en` and `dump-config --preset substack-en`.
3. If inline `X, not Y` starts overfiring on engineering prose, split the current branch into “interpretive corrective” vs “ordinary contrast” using noun/category cues instead of broad copular shape alone.
