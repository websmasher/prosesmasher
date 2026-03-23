# Release 0.1.6 Final Cleanout

**Date:** 2026-03-23 18:41
**Scope:** repo root docs, package README, version graph, changelog, tracked file cleanup

## Summary
This is the final repo cleanout before another crates.io publish. It commits the remaining documentation edits, removes the stale `CLAUDE.md` file that was already deleted in the worktree, bumps the workspace to `0.1.6`, and republishes the full crate graph so the public repo and crates.io package stay aligned.

## Context & Problem
After the public repo pass and subsequent README cleanup, there were still uncommitted tracked changes in the repo:

- `README.md`
- `apps/prosesmasher/packages/prosesmasher/README.md`
- the tracked deletion of `CLAUDE.md`

The user explicitly asked to clean out the repo and commit absolutely everything before pushing and releasing. That meant the final state needed:

- a clean git worktree
- one commit that included the remaining tracked changes
- an actual version bump and crates.io release, rather than leaving docs ahead of the published package

## Decisions Made

### Cut a patch release for the cleanup
- **Chose:** bump the whole crate graph to `0.1.6`.
- **Why:** the published package should reflect the current public-facing README state instead of leaving crates.io behind the repo again.
- **Alternatives considered:**
  - Commit docs only without a release — rejected because the user explicitly asked for a cargo release too.
  - Bundle the docs into a larger future release — rejected because the repo needed to be cleaned out now.

### Included the tracked `CLAUDE.md` deletion instead of restoring it
- **Chose:** keep the deletion and commit it as part of the cleanout.
- **Why:** the file was already gone from the worktree, and the instruction was to commit everything, not selectively preserve stale tracked files.
- **Alternatives considered:**
  - Restore `CLAUDE.md` before commit — rejected because it would add churn without a clear product reason.

### Revalidated the shipped READMEs with the tool itself
- **Chose:** rerun both README files through `prosesmasher` before releasing.
- **Why:** the docs are part of the product surface and had already been revised for tone.
- **Alternatives considered:**
  - Skip doc validation because only wording changed — rejected because the user explicitly wanted the docs run through the library.

## Architectural Notes
No runtime code changed in this cleanout slice. This is a packaging and repo-state alignment pass. The important effect is that:

- GitHub README
- crates.io README
- workspace version graph
- published package version

all move together again.

## Information Sources
- Current repo worktree state from `git status`
- `README.md`
- `apps/prosesmasher/packages/prosesmasher/README.md`
- `apps/prosesmasher/CHANGELOG.md`
- `apps/prosesmasher/Cargo.toml` and dependent crate `Cargo.toml` files
- validator runs on both README files before release

## Open Questions / Future Considerations
- This commit intentionally does not include untracked local fixtures because they were not present in the worktree at the time of the cleanout.
- If the newer local heuristic experiments should also ship, they need their own version bump and release pass from a clean starting point.

## Key Files for Context
- `README.md` — public GitHub-facing product description
- `apps/prosesmasher/packages/prosesmasher/README.md` — crates.io-facing product description
- `apps/prosesmasher/Cargo.toml` — workspace version source of truth
- `apps/prosesmasher/CHANGELOG.md` — release notes
- `.worklogs/2026-03-23-183336-public-readme-and-corrective-hardening.md` — public-facing README pass
- `.worklogs/2026-03-23-183711-refresh-agents-handoff.md` — follow-up handoff sync

## Next Steps / Continuation Plan
1. Commit all remaining tracked changes with `git add -A`.
2. Push `main` to `origin`.
3. Publish the full `0.1.6` crate graph in dependency order.
4. Reinstall the released crate and verify the installed binary version is `0.1.6`.
