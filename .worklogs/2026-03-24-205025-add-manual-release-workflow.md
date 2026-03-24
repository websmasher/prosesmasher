# Add Manual Release Workflow

**Date:** 2026-03-24 20:50
**Scope:** `.github/workflows/manual-release.yml`

## Summary
This adds a break-glass GitHub Actions workflow for releasing the current workspace without going through `release-plz release-pr`. It exists specifically so `0.1.7` can be published using the repository's `CARGO_REGISTRY_TOKEN` secret even though local publishing is not authenticated and the automated PR-generation path is blocked by historical package comparison.

## Context & Problem
After pushing the manual `0.1.7` version-bump commit, local `cargo publish` immediately failed with:

- `no token found, please run cargo login`

That was expected because the crates.io token is configured in GitHub Actions secrets, not in the local shell environment. At the same time, the existing `Release` workflow still could not be used because it insists on running `release-plz release-pr`, and that step is currently blocked on comparing packages against the old `0.1.4` release commit with the broken sibling path dependency.

So the repo needed a release path that:

- uses GitHub-hosted secrets
- skips `release-pr`
- still publishes with `release-plz release`
- still triggers dist packaging for the public `prosesmasher` tag

## Decisions Made

### Add a separate manual-release workflow instead of overloading `release.yml`
- **Chose:** create `.github/workflows/manual-release.yml` with `workflow_dispatch`.
- **Why:** it cleanly separates the emergency/manual path from the normal push-triggered release automation and avoids making the default workflow harder to reason about.
- **Alternatives considered:**
  - Add manual inputs and branching logic to `release.yml` — rejected because the normal push path is already under active repair and did not need more conditional behavior in the middle of a live release.
  - Publish locally after configuring a token on this machine — rejected because the existing trusted publishing secret already lives in GitHub Actions and the user asked to release now, not to reconfigure local credentials.

### Reuse `release-plz release` and the existing dist workflow
- **Chose:** keep the manual workflow minimal: one `release-plz release` job plus the same tag-extraction and `dist.yml` handoff used elsewhere.
- **Why:** the publish/tag logic already exists; the broken part is specifically the PR-generation phase, not the release or dist behavior.
- **Alternatives considered:**
  - Write a custom shell loop that calls `cargo publish` for each crate — rejected because `release-plz release` already knows the workspace graph and release config.
  - Upload artifacts manually without `dist.yml` — rejected because the repo already has a multi-platform packaging workflow.

## Architectural Notes
This workflow is operational scaffolding, not a new product path. It preserves the existing release architecture:

- `release-plz` still drives crates.io publication
- the public package still controls the tag/release surface
- `dist.yml` still produces GitHub release assets

The difference is only that manual release dispatch starts at the publish step instead of the release-PR generation step.

## Information Sources
- Local `cargo publish` failure after version bump
- `.github/workflows/release.yml`
- `.github/workflows/dist.yml`
- `.worklogs/2026-03-24-204822-prepare-manual-0-1-7-release.md`
- Earlier release repair worklogs from 2026-03-24

## Open Questions / Future Considerations
- Once the historical `release-pr` comparison issue is solved, decide whether this manual workflow should remain as a permanent break-glass path or be removed.
- The current release still needs the live workflow run to prove `release-plz release` plus `dist` works end-to-end in GitHub Actions.

## Key Files for Context
- `.github/workflows/manual-release.yml` — new manual publish/tag/dist entrypoint
- `.github/workflows/release.yml` — default push-triggered release flow that remains blocked on `release-pr`
- `.github/workflows/dist.yml` — reusable artifact/release workflow
- `.worklogs/2026-03-24-204822-prepare-manual-0-1-7-release.md` — manual `0.1.7` release-prep rationale

## Next Steps / Continuation Plan
1. Commit this workflow and push `main`.
2. Dispatch `Manual Release` from GitHub Actions on `main`.
3. Watch the publish job create the `v0.1.7` tag and then confirm the dist job uploads the release assets.
4. If the workflow surfaces any remaining publish-time issue, patch that workflow or the release config directly and rerun it.
