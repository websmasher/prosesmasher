# Release 0.1.3 Quality Taxonomy

**Date:** 2026-03-23 16:37
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/crates/ports/outbound/traits/Cargo.toml`, `apps/prosesmasher/crates/app/core/Cargo.toml`, `apps/prosesmasher/crates/adapters/outbound/fs/Cargo.toml`, `apps/prosesmasher/crates/adapters/outbound/parser/Cargo.toml`, `apps/prosesmasher/crates/adapters/inbound/cli/Cargo.toml`, `apps/prosesmasher/packages/prosesmasher/Cargo.toml`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Bumped the full `prosesmasher` crate graph from `0.1.2` to `0.1.3` and documented the taxonomy realignment in the changelog so the corrected canonical schema can be published to crates.io. Refreshed the workspace lockfile and re-ran the full verification suite before release.

## Context & Problem
The taxonomy fix was committed locally, but users installing from crates.io would still receive `0.1.2`, whose canonical config surface did not yet reflect the corrected `quality.flow` / `quality.readability` split. Because this is a public schema change, the fix needs a real follow-up release rather than staying only in the repository.

## Decisions Made

### Cut a new patch release for the schema correction
- **Chose:** Bump the full crate graph to `0.1.3`.
- **Why:** The canonical config shape changed in a user-visible way, so crates.io needs a fresh version that carries the corrected model.
- **Alternatives considered:**
  - Leave the fix unreleased — rejected because installed users would still see the old schema.
  - Fold it into a larger future release — rejected because this is a direct correctness fix for the published API.

### Keep the version bump graph-wide
- **Chose:** Update every publishable `prosesmasher*` crate plus the lockfile.
- **Why:** The wrapper crate depends on the internal crates and the published graph needs to stay version-coherent.
- **Alternatives considered:**
  - Only bump the public wrapper crate — rejected because the internal published packages also changed and would fall out of sync.

## Architectural Notes
This release does not add new runtime behavior beyond the schema alignment already committed in the previous changeset. It is the publication vehicle for that contract fix:

- workspace version is now `0.1.3`
- internal dependency constraints point to `0.1.3`
- the changelog explicitly records the taxonomy correction

## Information Sources
- `.worklogs/2026-03-23-163539-realign-quality-taxonomy.md`
- `cargo test -q`
- `cargo clippy -q --all-targets --all-features`
- `apps/prosesmasher/CHANGELOG.md`

## Open Questions / Future Considerations
- After publishing, the installed binary should be rechecked with `dump-config --full-config` to confirm crates.io users receive the corrected schema.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — workspace version source of truth
- `apps/prosesmasher/Cargo.lock` — resolved version graph
- `apps/prosesmasher/CHANGELOG.md` — public release note for `0.1.3`
- `.worklogs/2026-03-23-163539-realign-quality-taxonomy.md` — the actual schema redesign behind this release

## Next Steps / Continuation Plan
1. Commit the `0.1.3` bump and changelog.
2. Publish the crates in dependency order.
3. Reinstall `prosesmasher 0.1.3` from crates.io and verify `dump-config --full-config` shows `quality.flow` and `quality.readability`.
