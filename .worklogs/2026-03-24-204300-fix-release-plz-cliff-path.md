# Fix Release-Plz Cliff Path

**Date:** 2026-03-24 20:43
**Scope:** `apps/prosesmasher/release-plz.toml`

## Summary
This fixes the next release-plz path boundary after the manifest-path and dependency fixes. The workflow was loading `apps/prosesmasher/release-plz.toml` correctly, but the config still pointed to `cliff.toml` as if it were resolved relative to the config file rather than the repository root.

## Context & Problem
The `Release` workflow run triggered by commit `b6e155c` failed in `release-plz release-pr` with:

- `ERROR cannot read "cliff.toml"`

The config file exists at `apps/prosesmasher/cliff.toml`, and the workspace release config lives at `apps/prosesmasher/release-plz.toml`. The earlier manifest-path fix proved that the action is running from the repository root and requires explicit nested-workspace paths. `changelog_config = "cliff.toml"` was therefore still wrong in CI even though it works if commands are run from inside `apps/prosesmasher`.

## Decisions Made

### Make the changelog config path repository-root-relative
- **Chose:** change `changelog_config` from `cliff.toml` to `apps/prosesmasher/cliff.toml`.
- **Why:** it matches how the GitHub Action resolves the rest of the nested workspace configuration and removes another implicit working-directory assumption.
- **Alternatives considered:**
  - Move `cliff.toml` to the repository root — rejected because the app workspace intentionally lives under `apps/prosesmasher`.
  - Rework the workflow to `cd` into the app directory before invoking `release-plz` — rejected because the action already needs explicit paths for manifest/config and this is the same class of problem.

## Architectural Notes
This is another nested-workspace CI fix. The broader rule is now clear: for GitHub Actions in this repo, all release-related config paths need to be rooted from the repository top level, not assumed relative to the workspace file that references them.

## Information Sources
- Failed GitHub Actions run `23511251360`
- `apps/prosesmasher/release-plz.toml`
- `apps/prosesmasher/cliff.toml`
- Prior related worklogs:
  - `.worklogs/2026-03-24-203252-fix-release-plz-manifest-path.md`
  - `.worklogs/2026-03-24-204046-fix-release-workspace-dependency-and-semver-step.md`

## Open Questions / Future Considerations
- If more nested release config is added later, verify its path resolution in CI immediately instead of assuming local working-directory behavior will carry over.
- The next release run should confirm whether any remaining path assumptions exist beyond the manifest and changelog config.

## Key Files for Context
- `apps/prosesmasher/release-plz.toml` — workspace release-plz configuration
- `apps/prosesmasher/cliff.toml` — changelog generator configuration used by release-plz
- `.github/workflows/release.yml` — workflow that invokes release-plz from the repository root
- `.worklogs/2026-03-24-203252-fix-release-plz-manifest-path.md` — first nested-workspace path repair
- `.worklogs/2026-03-24-204046-fix-release-workspace-dependency-and-semver-step.md` — second release blocker fix

## Next Steps / Continuation Plan
1. Stage this worklog with the `release-plz.toml` path fix and commit them together.
2. Push `main` and watch the next `Release` workflow run.
3. If the run succeeds, merge the generated release PR and watch the publish/tag/dist follow-up run.
4. If another path-related failure appears, patch it in the same explicit-rooted style rather than changing the repo layout.
