# Fix Release-Plz Manifest Path

**Date:** 2026-03-24 20:32
**Scope:** `.github/workflows/release.yml`

## Summary
The first post-push release workflow failed before opening a release PR because `release-plz/action` looked for `Cargo.toml` at the repo root instead of under `apps/prosesmasher`. This patch passes the workspace manifest path explicitly to both `release-pr` and `release` action invocations.

## Context & Problem
After pushing the feature batch to `main`, the `Release` workflow ran and failed in the `Release-plz release-pr` step. The logs showed:

- `Failed to read metadata from manifest at /home/runner/work/prosesmasher/prosesmasher/Cargo.toml`
- `manifest path .../Cargo.toml does not exist`

The workflow already set `working-directory: apps/prosesmasher` under `defaults.run`, but that only affects shell `run:` steps. GitHub composite/action steps do not inherit that as their execution root, so `release-plz/action` still resolved manifests from the repository root.

## Decisions Made

### Pass `manifest_path` directly to `release-plz/action`
- **Chose:** add `manifest_path: apps/prosesmasher/Cargo.toml` to both the `release-pr` and `release` action invocations.
- **Why:** this fixes the actual resolution boundary for the action instead of relying on workflow shell defaults that do not apply to actions.
- **Alternatives considered:**
  - Move the entire workspace manifest to repo root — rejected because the app intentionally lives under `apps/prosesmasher`.
  - Wrap `release-plz` in manual shell commands instead of the action — rejected because the repo already uses the first-party action and only needed the correct manifest input.

## Architectural Notes
This is a GitHub Actions scoping fix, not a product/runtime change. The important lesson is that `defaults.run.working-directory` is not a substitute for explicit path inputs when a third-party action shells out to Cargo internally.

## Information Sources
- Failed workflow run: `Release` run `23510678172`
- Failure log from `Release-plz release-pr`
- `.github/workflows/release.yml`

## Open Questions / Future Considerations
- If additional GitHub actions are added around the nested workspace, prefer explicit manifest/config paths over assumptions about working directory inheritance.
- The workflow still uses `CARGO_REGISTRY_TOKEN` until crates.io Trusted Publishing is configured for all published workspace crates.

## Key Files for Context
- `.github/workflows/release.yml` — release automation entrypoint
- `.worklogs/2026-03-24-202631-html-parser-quality-release-plumbing.md` — broader feature/release-plumbing batch that triggered the failed run

## Next Steps / Continuation Plan
1. Commit this workflow fix and push `main`.
2. Watch the next `Release` workflow run to confirm `release-plz` opens the release PR.
3. Merge the generated release PR.
4. Watch the follow-up release run publish crates and invoke the dist workflow.
