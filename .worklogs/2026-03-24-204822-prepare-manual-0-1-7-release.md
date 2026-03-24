# Prepare Manual 0.1.7 Release

**Date:** 2026-03-24 20:48
**Scope:** `apps/prosesmasher/Cargo.toml`, crate `Cargo.toml` dependency versions, `apps/prosesmasher/CHANGELOG.md`, `.github/workflows/dist.yml`, `apps/prosesmasher/Cargo.lock`

## Summary
This prepares a manual `0.1.7` release after the automated `release-plz release-pr` path proved blocked by historical package comparison against an old release commit with a broken sibling path dependency. The workspace version graph is bumped to `0.1.7`, the changelog records the shipped parser/heuristic/release-surface fixes, and the dist workflow is made manually dispatchable so GitHub release artifacts can still be built for the tagged release.

## Context & Problem
After fixing the immediate CI issues in the release workflow itself, `release-plz release-pr` still failed. The failure was no longer about the current workspace. Instead, `release-plz` tried to compare the current packages against the old `Release prosesmasher 0.1.4` commit (`f9e073e`) and ran `cargo package` on a historical checkout whose workspace still referenced:

- `../../../low-expectations/crates/low-expectations`

That historical commit cannot be packaged on CI because the sibling repository is absent. In other words, the current repo state is releasable, but the automation is blocked by an old release anchor that is not reproducible from a clean checkout.

The user asked to sort out the worktree and release the changes now. Given that constraint, waiting on a deeper `release-plz` migration or history rewrite was the wrong move.

## Decisions Made

### Cut a manual patch release to `0.1.7`
- **Chose:** bump the entire workspace dependency graph from `0.1.6` to `0.1.7`.
- **Why:** the repo contains real shipped behavior changes, and a clean new release is the fastest path that does not depend on the broken historical comparison.
- **Alternatives considered:**
  - Keep chasing `release-plz release-pr` until it can compare against the old `0.1.4` commit — rejected because that is now a historical reproducibility problem, not a current uncommitted-worktree problem.
  - Publish without a version bump — rejected because crates.io and GitHub releases need a new version anchor for the new code.

### Keep the release surface honest with a changelog entry
- **Chose:** add a `0.1.7` changelog section covering the user-visible parser and heuristic fixes plus packaging/release-surface cleanup.
- **Why:** this release bundles several meaningful behavior changes, and future release archaeology should not require reading commit diffs to understand them.
- **Alternatives considered:**
  - Skip changelog updates and rely on git history — rejected because the crate is public and the version bump should be documented.

### Make the dist workflow manually dispatchable
- **Chose:** add `workflow_dispatch` with the existing `tag` input to `.github/workflows/dist.yml`.
- **Why:** the repo still needs GitHub release artifacts for `cargo binstall`, and manual release execution now needs a direct way to invoke the artifact pipeline after pushing the tag.
- **Alternatives considered:**
  - Build release artifacts locally and upload them by hand — rejected because the existing dist workflow already encodes the multi-platform build matrix.
  - Leave dist callable only from `release.yml` — rejected because the blocked `release-pr` step would prevent reaching the workflow-call path.

## Architectural Notes
This does not abandon the multi-crate release model. It is a temporary operational bypass around one automation choke point:

- crates.io publication still uses the full workspace crate graph
- GitHub binary artifacts still come from the shared `dist` workflow
- only the `release-plz release-pr` step is being bypassed for this release because it depends on comparing against a historically non-reproducible commit

The important distinction is current-state reproducibility versus historical-state reproducibility. The current workspace is now cleanly reproducible from crates.io dependencies, but one older release anchor was not.

## Information Sources
- Failed workflow runs:
  - `23511251360`
  - `23511320327`
- `apps/prosesmasher/Cargo.toml`
- crate manifests under `apps/prosesmasher/crates/**/Cargo.toml`
- `apps/prosesmasher/CHANGELOG.md`
- `.github/workflows/dist.yml`
- Prior related worklogs:
  - `.worklogs/2026-03-24-202631-html-parser-quality-release-plumbing.md`
  - `.worklogs/2026-03-24-203252-fix-release-plz-manifest-path.md`
  - `.worklogs/2026-03-24-204046-fix-release-workspace-dependency-and-semver-step.md`
  - `.worklogs/2026-03-24-204300-fix-release-plz-cliff-path.md`

## Open Questions / Future Considerations
- `release-plz release-pr` still needs a permanent fix for historical package comparison, likely by changing how old releases are anchored or by resetting the release baseline once the current manual release is out.
- After `0.1.7` ships, the workflow should be revisited so future patch releases do not need the manual bypass.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — workspace version source of truth
- `apps/prosesmasher/Cargo.lock` — lockfile aligned to `0.1.7`
- `apps/prosesmasher/CHANGELOG.md` — release notes for `0.1.7`
- `.github/workflows/dist.yml` — manual entrypoint for GitHub release artifact generation
- `apps/prosesmasher/release-plz.toml` — current automated release config that remains blocked on historical comparison
- `.worklogs/2026-03-24-204046-fix-release-workspace-dependency-and-semver-step.md` — dependency reproducibility fix that solved the current workspace but exposed the historical comparison issue
- `.worklogs/2026-03-24-204300-fix-release-plz-cliff-path.md` — latest release-plz path fix before switching to manual release

## Next Steps / Continuation Plan
1. Stage this worklog with the `0.1.7` version/changelog/workflow changes and commit them together.
2. Push `main`.
3. Publish the workspace crates to crates.io in dependency order for `0.1.7`.
4. Create and push the `v0.1.7` tag.
5. Dispatch `.github/workflows/dist.yml` with `tag=v0.1.7` and verify the GitHub release assets upload successfully.
