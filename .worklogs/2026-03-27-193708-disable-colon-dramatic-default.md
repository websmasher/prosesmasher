# Disable Colon-Dramatic By Default

**Date:** 2026-03-27 19:37
**Scope:** `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `apps/prosesmasher/CHANGELOG.md`, workspace version metadata

## Summary
Disabled `colon-dramatic` in the canonical heuristics defaults and the shipped full English preset, then bumped the release surface to `0.3.12`. The rule stays implemented and available for explicit opt-in use, but it is no longer part of the default preset path because the current matcher is still too noisy on ordinary prose.

## Context & Problem
The user reported a large number of false positives from `colon-dramatic`, especially on normal article/link-intro prose such as “here is what I wrote” style constructions. This matches an already-known limitation in the codebase: the synthetic suite contains an ignored false-positive test for factual `label: value` shapes, and the project handoff notes already called out that the heuristic lacks intent detection.

At this point the clean fix is not to broaden exemptions piecemeal. The safer product move is to disable the rule by default until the matcher has a better boundary.

## Decisions Made

### Disable the rule at the config surface instead of deleting it
- **Chose:** Set `colon_dramatic.enabled = false` in `HeuristicsConfig::default()` and in both copies of the shipped `full-config-en.json`.
- **Why:** This removes the false-positive pressure from all default and preset-driven runs without losing the implementation for future opt-in or later repair.
- **Alternatives considered:**
  - Delete the rule from the catalog — rejected because the heuristic may still be useful later after a boundary redesign.
  - Patch the matcher immediately — rejected because the current issue is broad enough that a quick exemption pass would likely just produce a different class of false positives.

### Keep the public default contract explicit
- **Chose:** Updated the release note so `dump-config --full-config` now intentionally shows `"colonDramatic": { "enabled": false }`.
- **Why:** This is a public-surface change, not just an internal tweak. The emitted config should reflect the decision clearly.
- **Alternatives considered:**
  - Rely on code defaults only and leave the preset JSON untouched — rejected because the preset assets are part of the public behavior and would continue advertising the rule as enabled.

### Treat this as a versioned release-surface change
- **Chose:** Bumped the workspace to `0.3.12`.
- **Why:** The shipped default preset behavior changed, and the CLI version output should track that.
- **Alternatives considered:**
  - Leave the version unchanged — rejected because the user explicitly wants semantic version bumps on each shipped build pass.

## Architectural Notes
- The rule remains in the catalog and runtime. This is a preset/default-surface rollback, not a code-path removal.
- The disabling point is intentionally duplicated in:
  - domain default config
  - repo preset asset
  - embedded preset asset in the FS adapter
- That keeps all config entry paths aligned: direct defaults, dumped configs, and preset loading.

## Information Sources
- User report in the live session about `colon-dramatic` producing many false positives on normal prose.
- Existing known-limitation note in `AGENTS.md` / project handoff about `colon-dramatic` false positives on factual `label:value` patterns.
- Existing ignored test in `apps/prosesmasher/crates/app/checks/style-signals/runtime/src/heur_09_colon_dramatic_tests/synthetic.rs`.
- Existing config surface in:
  - `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`
  - `apps/prosesmasher/presets/full-config-en.json`
  - `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`

## Open Questions / Future Considerations
- If `colon-dramatic` comes back, it should return only after a proper boundary redesign, probably with stronger structural cues than “short clause after colon”.
- A future fix should explicitly protect:
  - factual label/value prose
  - intro/link framing
  - technical explanatory sentences with legitimate colon usage

## Key Files for Context
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical default heuristic surface; this is where `colon-dramatic` was turned off by default.
- `apps/prosesmasher/presets/full-config-en.json` — public preset asset users inspect and dump.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json` — embedded preset copy used by the loader.
- `apps/prosesmasher/crates/app/checks/style-signals/runtime/src/heur_09_colon_dramatic_tests/synthetic.rs` — existing ignored false-positive test that explains why this rollback was warranted.
- `apps/prosesmasher/CHANGELOG.md` — release note for `0.3.12`.
- `.worklogs/2026-03-27-174128-abstract-frames-group-generalization-empty-labels.md` — previous release pass immediately before this default-surface rollback.

## Next Steps / Continuation Plan
1. If the rule is revisited, start by collecting a dedicated corpus of bad dramatic-colon examples versus normal article/link/factual colon usage.
2. Replace the current “short clause after colon” heuristic with a more explicit rhetorical-shape matcher before re-enabling it by default.
3. Until that redesign exists, keep `colon-dramatic` opt-in only and treat any attempt to re-enable it as a regression-sensitive change requiring corpus review.
