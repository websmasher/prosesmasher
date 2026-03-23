# Commit Lockfile Release Cleanup

**Date:** 2026-03-23 16:17
**Scope:** `apps/prosesmasher/Cargo.lock`

## Summary
Committed the remaining lockfile change after the `0.1.2` release work so the repository returns to a clean state. The diff is only the workspace package version updates from `0.1.1` to `0.1.2`.

## Context & Problem
The `0.1.2` release work and publish flow were completed, but `apps/prosesmasher/Cargo.lock` was left modified in the working tree. That meant the repository was not actually clean after release, even though the change was expected and should have been part of the version bump history.

## Decisions Made

### Keep the lockfile aligned with the published version bump
- **Chose:** Commit the remaining `Cargo.lock` changes as a final cleanup pass.
- **Why:** The lockfile should reflect the current published workspace version and the repository should not retain unexplained release fallout.
- **Alternatives considered:**
  - Revert the lockfile diff — rejected because the diff is the correct state after the `0.1.2` version bump.
  - Leave it uncommitted — rejected because it leaves the repo dirty and obscures the actual final release state.

## Architectural Notes
This change does not affect runtime behavior. It only restores consistency between the published crate graph and the checked-in workspace lockfile.

## Information Sources
- `git diff -- apps/prosesmasher/Cargo.lock`
- `.worklogs/2026-03-23-161101-tighten-cli-contract-and-docs.md`
- `cargo search prosesmasher --limit 5`

## Open Questions / Future Considerations
- None for the lockfile itself. The meaningful follow-up remains product work from the latest CLI feedback, not release cleanup.

## Key Files for Context
- `apps/prosesmasher/Cargo.lock` — final lockfile state for the `0.1.2` crate graph
- `.worklogs/2026-03-23-161101-tighten-cli-contract-and-docs.md` — main `0.1.2` release work and rationale
- `apps/prosesmasher/Cargo.toml` — workspace version source of truth

## Next Steps / Continuation Plan
1. Stage `apps/prosesmasher/Cargo.lock` with this worklog and commit the cleanup.
2. Re-run smoke checks against the installed `prosesmasher 0.1.2` binary to verify the shipped CLI contract.
3. Continue with the remaining product-correct feedback items as separate feature work, not as release cleanup.
