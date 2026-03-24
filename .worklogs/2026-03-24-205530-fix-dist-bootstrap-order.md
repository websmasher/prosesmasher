# Fix Dist Bootstrap Order

**Date:** 2026-03-24 20:55
**Scope:** `.github/workflows/dist.yml`

## Summary
This fixes the first dist matrix failure that appeared during the live `v0.1.7` release. The matrix job tried to run a shell step inside `apps/prosesmasher` before checkout had created that directory, so every target failed before any build work started.

## Context & Problem
The manual release workflow successfully published `0.1.7` to crates.io and created the `v0.1.7` tag, which then triggered the reusable `Dist` workflow. The first matrix jobs all failed immediately at:

- `Enable Windows long paths`

The error was:

- `An error occurred trying to start process '/usr/bin/bash' with working directory '/home/runner/work/prosesmasher/prosesmasher/apps/prosesmasher'. No such file or directory`

That happened because `build-local-artifacts` sets:

- `defaults.run.working-directory: apps/prosesmasher`

but its first step was a plain `run:` step before checkout. On a fresh runner, that working directory does not exist until the checkout step populates the repository. The step was also unnecessary on non-Windows targets.

## Decisions Made

### Move the long-path step after checkout and restrict it to Windows
- **Chose:** place `Enable Windows long paths` after `actions/checkout@v6` and add `if: runner.os == 'Windows'`.
- **Why:** the working directory exists after checkout, and only Windows benefits from the long-path git config.
- **Alternatives considered:**
  - Remove the step entirely — rejected because Windows artifact builds can still benefit from the setting.
  - Keep it before checkout and override the working directory just for that step — rejected because it is more awkward than simply moving it behind checkout.

## Architectural Notes
This is a reusable-workflow bootstrap fix. The broader lesson is that any job using `defaults.run.working-directory` must not start with a shell step that assumes the repo is already checked out.

## Information Sources
- Failed `Dist / build-local-artifacts (x86_64-unknown-linux-gnu)` log from run `23511709337`
- `.github/workflows/dist.yml`
- `.worklogs/2026-03-24-205232-force-manual-release-plz-publish.md`

## Open Questions / Future Considerations
- After this fix, rerun `Dist` directly on `v0.1.7` to expose the next artifact-build issue, if any.
- The workflow still uses `actions/checkout@v6` and `actions/upload/download-artifact@v6/v7`; keep an eye on any future GitHub Actions platform warnings, but they are not blocking the current release.

## Key Files for Context
- `.github/workflows/dist.yml` — reusable build/upload/release pipeline for binary artifacts
- `.github/workflows/manual-release.yml` — workflow that published `v0.1.7` and kicked off the first failing dist run
- `.worklogs/2026-03-24-205232-force-manual-release-plz-publish.md` — release-plz manual publish override

## Next Steps / Continuation Plan
1. Commit this dist workflow fix and push `main`.
2. Re-run `Dist` manually with `tag=v0.1.7`.
3. Verify the matrix builds complete and the GitHub release for `v0.1.7` receives the generated assets.
4. If another dist issue appears, patch it in place and rerun `Dist` without republishing crates.
