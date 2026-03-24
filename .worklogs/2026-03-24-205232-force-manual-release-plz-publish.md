# Force Manual Release-Plz Publish

**Date:** 2026-03-24 20:52
**Scope:** `.github/workflows/manual-release.yml`

## Summary
This updates the new manual release workflow so it actually publishes the staged `0.1.7` release. The first manual run proved that `release-plz release` was still respecting `release_always = false` from the shared config and therefore skipped publication because the current commit did not come from a release PR.

## Context & Problem
The first `Manual Release` GitHub Actions run (`23511627791`) completed successfully but did not publish anything. The key log line was:

- `skipping release: current commit is not from a release PR`

That behavior came from the shared `apps/prosesmasher/release-plz.toml` setting:

- `release_always = false`

That default is correct for the normal push-triggered workflow, because it prevents arbitrary `main` commits from publishing. But the whole point of the new manual workflow is to serve as a break-glass path when the release-PR generation phase is unavailable. Reusing the shared config verbatim therefore preserved the same policy gate that the manual workflow was supposed to bypass.

## Decisions Made

### Generate a temporary manual release-plz config inside the workflow
- **Chose:** create `apps/prosesmasher/release-plz.manual.toml` on the runner and flip only `release_always = true` before invoking `release-plz release`.
- **Why:** this keeps the committed default config unchanged for the normal workflow while giving the manual workflow the one policy override it needs.
- **Alternatives considered:**
  - Change `apps/prosesmasher/release-plz.toml` to `release_always = true` permanently — rejected because that weakens the default safety guard for ordinary pushes.
  - Add a separate committed second config file to the repo — rejected because the workflow can derive the manual variant trivially and there is no reason to maintain two checked-in copies.

## Architectural Notes
The manual workflow now differs from the normal workflow in exactly one policy bit:

- normal path: `release_always = false`
- break-glass manual path: `release_always = true`

Everything else still comes from the shared `release-plz.toml`, so publication order, tagging behavior, and crate-selection policy stay aligned.

## Information Sources
- Successful-but-skipped GitHub Actions run `23511627791`
- `.github/workflows/manual-release.yml`
- `apps/prosesmasher/release-plz.toml`
- `.worklogs/2026-03-24-205025-add-manual-release-workflow.md`

## Open Questions / Future Considerations
- Once the historical `release-pr` issue is fixed, this manual override path may no longer be needed except as emergency tooling.
- The next manual workflow run still needs to prove end-to-end publication and dist release creation.

## Key Files for Context
- `.github/workflows/manual-release.yml` — break-glass publish workflow with the temporary config override
- `apps/prosesmasher/release-plz.toml` — default release-plz config kept unchanged
- `.worklogs/2026-03-24-205025-add-manual-release-workflow.md` — original manual workflow rationale
- `.worklogs/2026-03-24-204822-prepare-manual-0-1-7-release.md` — manual release-prep rationale and version bump

## Next Steps / Continuation Plan
1. Commit this workflow change and push `main`.
2. Re-run `Manual Release`.
3. Confirm that `release-plz release` now publishes the crate graph and emits the `v0.1.7` tag.
4. Let the dist job run and verify the GitHub release assets appear on the new tag.
