# Add Release Plz Workflow

**Date:** 2026-03-23 10:11
**Scope:** `.github/workflows/release.yml`, `apps/prosesmasher/release-plz.toml`

## Summary
Added a GitHub Actions release workflow for `prosesmasher` mirroring the `pipelin3r` pattern: release automation runs on pushes to `production` and uses `release-plz` with `CARGO_REGISTRY_TOKEN` from repository secrets. Also replaced the placeholder `release-plz.toml` package entry with the real publishable crate list.

## Context & Problem
`prosesmasher` now has a publishable crate graph and a public wrapper crate, but there was still no CI release path. Unlike `pipelin3r`, this repo had:
- no `.github/workflows` directory
- a placeholder `release-plz.toml` containing `your-crate-name`

Without CI release wiring, publishing would be manual-only and inconsistent with the existing `pipelin3r` release process that already uses `production` pushes and `release-plz`.

## Decisions Made

### Mirror `pipelin3r`’s production-branch release flow
- **Chose:** Add `.github/workflows/release.yml` that triggers on pushes to `production` and runs `release-plz` for both `release-pr` and `release`.
- **Why:** This keeps `prosesmasher` operationally aligned with `pipelin3r`, which already uses the same release model.
- **Alternatives considered:**
  - Publish manually from local machines — rejected because local auth is not present and CI is the cleaner release path.
  - Trigger from `main` — rejected because the established pattern in sibling repos is `production`.

### Populate the real crate list in `release-plz.toml`
- **Chose:** Replace the placeholder config with all publishable crates in dependency order.
- **Why:** `release-plz` needs the real package set to generate and execute release actions correctly.
- **Alternatives considered:**
  - Only list the public wrapper crate — rejected because the wrapper depends on internal crates that also need to be released to crates.io.

## Architectural Notes
This change does not alter the runtime architecture. It is purely about release orchestration:
- internal crates remain individually publishable
- the public wrapper crate remains the user-facing install target
- CI now has an opinionated path to publish them through crates.io when the secret is configured

The current practical blocker is not code anymore. It is repository configuration:
- `CARGO_REGISTRY_TOKEN` must exist in `websmasher/prosesmasher` secrets
- `gh secret list --repo websmasher/prosesmasher` returned no visible secrets in this environment

## Information Sources
- `pipelin3r` release workflow:
  - `/Users/tartakovsky/Projects/websmasher/pipelin3r/.github/workflows/release.yml`
  - `/Users/tartakovsky/Projects/websmasher/pipelin3r/.worklogs/2026-03-16-095845-fix-release.md`
  - `/Users/tartakovsky/Projects/websmasher/pipelin3r/.worklogs/2026-03-15-160258-publish-setup.md`
- `apps/prosesmasher/release-plz.toml`
- `cargo test -q` in `apps/prosesmasher`

## Open Questions / Future Considerations
- The repo may still need a normal CI workflow alongside release automation, since it currently has no `.github/workflows` other than this newly added release file.
- Once `low-expectations` is published and the repo secret exists, the next test should be a real `production` release run rather than more local dry-runs.

## Key Files for Context
- `.github/workflows/release.yml` — CI publish path
- `apps/prosesmasher/release-plz.toml` — publishable crate list
- `.worklogs/2026-03-23-094745-set-up-publishable-wrapper-crate.md` — packaging refactor that made the crate graph publishable

## Next Steps / Continuation Plan
1. Ensure `CARGO_REGISTRY_TOKEN` is configured in the `websmasher/prosesmasher` GitHub repository secrets.
2. Publish `low-expectations` first, since it is the external dependency gate for `prosesmasher-app-core`.
3. Push or merge into `production` in `prosesmasher` to let `release-plz` publish the crate chain through CI.
