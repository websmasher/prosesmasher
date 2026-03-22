# Remove legacy lexical check family

**Date:** 2026-03-22 18:26
**Scope:** `apps/prosesmasher/crates/app/core/src/check.rs`, `apps/prosesmasher/crates/app/core/src/terms/mod.rs`, `apps/prosesmasher/crates/app/core/src/terms/{banned_words,banned_phrases,forbidden_terms,gendered_terms,race_terms}*.rs`

## Summary
Removed the inactive legacy lexical check family and their tests, leaving `prohibited-terms` as the only active lexical prohibition surface. Also updated the `Check` trait docs to reflect the canonical public IDs and labels.

## Context & Problem
After the canonical config migration and the heuristic-defaults migration, the old lexical family was no longer part of the active registry:
- `banned-words`
- `banned-phrases`
- `forbidden-terms`
- `gendered-terms`
- `race-terms`

They remained in the tree as dead code and stale tests. That created two problems:
- the codebase still advertised an obsolete taxonomy internally
- searches and agent sessions kept surfacing historical IDs that no longer represented the product

At this point the compatibility layer exists at config loading, not as parallel active checks. Keeping the dormant modules offered little value and continued to blur the public model.

## Decisions Made

### Delete inactive lexical modules instead of keeping them as dead compatibility code
- **Chose:** Remove the old lexical check implementations and their tests from `app/core`.
- **Why:** The compatibility story is now handled by the loader normalizing old config into canonical prohibited terms. There is no need to preserve a second obsolete runtime surface inside the core check registry.
- **Alternatives considered:**
  - Keep the files for another migration cycle — rejected because they were already inactive and only contributed confusion.
  - Keep them but mark them deprecated — rejected because no active code path referenced them anymore, so deprecation would be ceremony without value.

### Tighten comments to the actual public surface
- **Chose:** Update `Check` trait examples from `banned-words` to `prohibited-terms`.
- **Why:** Low-level comments are part of the developer-facing API. They should stop teaching removed IDs.
- **Alternatives considered:**
  - Leave comments alone since they are non-functional — rejected because stale comments keep reviving the wrong mental model.

## Architectural Notes
This cleanup completes the lexical surface simplification:
- active runtime: `prohibited-terms`
- loader compatibility: still accepts legacy lexical config fields and normalizes them
- no second legacy lexical check family remains in the core crate

That is the right layering. Compatibility belongs at the boundary, not as a shadow set of core check implementations.

## Information Sources
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — canonical config and `prohibited-terms` migration
- `.worklogs/2026-03-22-181937-heuristic-defaults-migration.md` — quality heuristics migration
- `.worklogs/2026-03-22-182357-canonical-docs-and-presets.md` — public docs/preset update to canonical surface
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — active term registry
- `apps/prosesmasher/crates/app/core/src/check.rs` — trait docs referencing public IDs
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- The compatibility bridge in `patterns/mod.rs` and `terms/mod.rs` still uses legacy config fields as override inputs. That is intentional for now, but the long-term decision is whether to keep that adapter-like behavior in core helpers or push it fully back into the FS normalization layer.
- Historical loader fixtures still cover legacy config. That is correct, but they should remain clearly separated from canonical examples.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — active lexical registry and helper surface
- `apps/prosesmasher/crates/app/core/src/terms/prohibited_terms.rs` — canonical lexical prohibition check
- `apps/prosesmasher/crates/app/core/src/check.rs` — trait docs aligned to current IDs
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — canonical lexical/config migration
- `.worklogs/2026-03-22-182357-canonical-docs-and-presets.md` — public docs aligned to canonical surface

## Next Steps / Continuation Plan
1. Decide whether to leave the remaining legacy override hooks in `patterns/mod.rs` and `terms/mod.rs` or move them fully into config normalization.
2. If the answer is “move them,” start by reading:
   - `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/mod.rs`
   - `apps/prosesmasher/crates/app/core/src/terms/mod.rs`
3. Keep the next cleanup focused on compatibility-boundary consolidation rather than mixing it with more product-facing changes.
