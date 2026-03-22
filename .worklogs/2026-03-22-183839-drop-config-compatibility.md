# Drop config compatibility layer

**Date:** 2026-03-22 18:38
**Scope:** `AGENTS.md`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`, `apps/prosesmasher/crates/app/core/src/patterns/mod.rs`, `apps/prosesmasher/crates/app/core/src/terms/mod.rs`, `apps/prosesmasher/crates/app/core/src/terms/hedge_words_tests.rs`, `apps/prosesmasher/crates/domain/types/src/config.rs`, `apps/prosesmasher/crates/domain/types/src/lib.rs`, `apps/prosesmasher/crates/domain/types/src/lib_tests.rs`

## Summary
Removed the remaining `terms` / `thresholds` compatibility layer and made `prosesmasher` canonical-only. The loader now accepts only `quality` + `documentPolicy`, active checks no longer read synthesized legacy views, and docs/tests were updated to reflect the hard break.

## Context & Problem
The earlier migration left a transitional boundary in place:
- `CheckConfig` still carried synthesized `terms` and `thresholds`
- the FS loader still normalized old schema into canonical config
- a few helpers still looked at compatibility fields

That made the codebase internally inconsistent. The user clarified that there are no active consumers of the old schema and explicitly asked to remove all backward compatibility rather than carry legacy surface area forward. With that constraint, keeping adapter compatibility or runtime bridges would only preserve dead semantics and increase the chance of silent misconfiguration.

## Decisions Made

### Remove compatibility from the domain model
- **Chose:** Delete `CheckConfig.terms`, `CheckConfig.thresholds`, and the old `TermLists` / `Thresholds` structs.
- **Why:** Canonical config had already become the real product model. Leaving synthetic compatibility fields in the domain would keep the wrong abstractions alive and invite more conditional logic.
- **Alternatives considered:**
  - Keep compatibility fields as private/internal views — rejected because they would still distort the domain model.
  - Keep them until another cleanup pass — rejected because the user explicitly requested a clean break now.

### Make the FS loader canonical-only and strict
- **Chose:** Replace the dual-schema DTOs with canonical-only DTOs and add `serde(deny_unknown_fields)` across the config DTO tree.
- **Why:** Once backward compatibility is dropped, the loader must reject old config instead of silently accepting unknown `terms` / `thresholds` keys and running defaults. Strict parsing makes failures immediate and obvious.
- **Alternatives considered:**
  - Accept old keys but ignore them — rejected because that would create dangerous false-success behavior.
  - Keep the loader permissive for convenience — rejected because it would undermine the hard break and make debugging configuration errors harder.

### Remove the last runtime compatibility hooks
- **Chose:** Delete fallback logic in `patterns/mod.rs` and `terms/mod.rs` that used legacy config-authored pattern/hedge lists.
- **Why:** Those heuristics are library-owned defaults now. The runtime should not carry any hidden dependence on removed config shapes.
- **Alternatives considered:**
  - Keep the hooks but mark them deprecated — rejected because there are no users left to protect.
  - Move the hooks into the loader — rejected because the user asked to remove compatibility, not relocate it.

### Update tests and handoff docs to match the hard break
- **Chose:** Rewrite config loader tests around canonical behavior and add assertions that legacy keys are rejected. Update `AGENTS.md` to describe the canonical-only model and current check taxonomy.
- **Why:** Without this, future sessions would keep reasoning from the wrong product contract.
- **Alternatives considered:**
  - Leave docs/tests partially historical — rejected because stale docs are how compatibility myths survive.

## Architectural Notes
The config boundary is now clean:
- domain model: canonical only
- FS adapter: canonical only, strict unknown-field rejection
- runtime helpers: canonical only

This restores the intended layering:
- `low-expectations` remains the primitive validation substrate
- `prosesmasher` owns the semantic prose-quality layer
- config semantics are `quality.lexical`, `quality.heuristics`, and opt-in `documentPolicy`

The strict DTO parsing is an important part of the architecture now. It prevents old config from being misread as a valid empty canonical config, which would have produced misleading passes.

## Information Sources
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — initial canonical config move
- `.worklogs/2026-03-22-181937-heuristic-defaults-migration.md` — heuristic ownership move
- `.worklogs/2026-03-22-182357-canonical-docs-and-presets.md` — canonical public docs/presets
- `.worklogs/2026-03-22-182620-remove-legacy-lexical-checks.md` — prior dead-code cleanup
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical domain shape
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — strict canonical loader
- `apps/prosesmasher/crates/app/core/src/patterns/mod.rs` — library-owned heuristic defaults
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — canonical lexical/hedge helpers
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- The checks still live in historical module folders (`terms`, `patterns`, `structure`, `readability`) even though the semantic model is now clearer. That is a code-organization issue, not an API issue.
- `AGENTS.md` still summarizes modules by old folder names because the physical layout has not been reorganized yet.
- If we later want even stricter config validation, the next place to look is whether some optional canonical blocks should become explicitly required rather than defaulting empty.

## Key Files for Context
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical config structs and defaults
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — canonical-only serde DTOs and strict parsing
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs` — canonical loader behavior and rejection tests
- `apps/prosesmasher/crates/app/core/src/patterns/mod.rs` — built-in heuristic phrase defaults with no legacy override path
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — built-in hedge defaults and lexical helpers
- `AGENTS.md` — current handoff reflecting the canonical-only product contract
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — initial migration context
- `.worklogs/2026-03-22-181937-heuristic-defaults-migration.md` — default heuristic context

## Next Steps / Continuation Plan
1. Revisit module organization in `apps/prosesmasher/crates/app/core/src/` so folder names reflect the semantic split better: core prose quality vs document policy.
2. Decide whether to keep readability as its own top-level module or fold it under a broader quality namespace without changing check behavior.
3. Review curated presets against the now-final canonical schema and trim any leftover over-specific editorial choices that were added before the config model stabilized.
