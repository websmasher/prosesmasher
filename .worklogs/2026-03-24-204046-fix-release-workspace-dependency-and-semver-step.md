# Fix Release Workspace Dependency And Semver Step

**Date:** 2026-03-24 20:40
**Scope:** `.github/workflows/release.yml`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`

## Summary
This patch fixes the next release blocker after the manifest-path repair. The GitHub Actions release job could not resolve the locally-pathed `low-expectations` workspace dependency on the runner, so the workspace now depends on the published crates.io version instead. The workflow also drops a redundant manual `cargo-semver-checks` install step because `release-plz/action` already provisions that tool itself.

## Context & Problem
After fixing `release-plz`'s manifest path and pushing `main`, the next `Release` workflow still failed. The failing run showed `cargo metadata` erroring inside GitHub Actions because `apps/prosesmasher/Cargo.toml` referenced:

- `low-expectations = { version = "0.1.0", path = "../../../low-expectations/crates/low-expectations" }`

That path exists on the local machine because this repo lives next to the sibling `low-expectations` repository, but GitHub Actions checks out only `websmasher/prosesmasher`. As a result, the nested workspace could not even resolve metadata on CI, so `release-plz` could not open the release PR.

While inspecting the failed workflow and current config, it was also clear that the explicit `cargo install cargo-semver-checks --locked` step was unnecessary. The `release-plz/action` logs showed the action already fetching a `cargo-semver-checks` binary through its own setup flow, so the manual install was just extra runtime and another network-dependent step.

## Decisions Made

### Switch `low-expectations` to the published crates.io dependency
- **Chose:** replace the local `path` dependency with `low-expectations = "0.1.0"` in the workspace dependencies.
- **Why:** the release workflow must resolve the full dependency graph from a clean checkout on GitHub Actions, and a sibling path outside this repository breaks that assumption.
- **Alternatives considered:**
  - Keep the local path and teach GitHub Actions to clone the sibling repository — rejected because the dependency is already published and the release workflow should not depend on custom multi-repo checkout state.
  - Vendor `low-expectations` into this repo — rejected because it would duplicate an intentionally separate crate and create maintenance churn.

### Remove the redundant manual `cargo-semver-checks` install step
- **Chose:** delete the standalone install step from `.github/workflows/release.yml`.
- **Why:** `release-plz/action` already installs the tool it needs, so the manual step wastes time without changing behavior.
- **Alternatives considered:**
  - Keep the step but pin a version — rejected because it still duplicates the action's own setup path.
  - Replace `release-plz/action` with raw CLI commands to control tool installation — rejected because the current action-based workflow is otherwise correct after the manifest-path fix.

## Architectural Notes
This is a release-environment boundary fix, not an application logic change. The workspace still keeps its multi-crate architecture and still depends on `low-expectations`; the only change is that CI and released builds now resolve it the same way end users do, from crates.io rather than from the author's neighboring checkout layout.

Using the registry dependency here is important for reproducibility. A release pipeline that only works when the repository happens to be checked out next to a sibling project is not a valid public release setup.

## Information Sources
- Failed GitHub Actions run `23510904591`
- `.github/workflows/release.yml`
- `apps/prosesmasher/Cargo.toml`
- `apps/prosesmasher/Cargo.lock`
- Prior worklogs:
  - `.worklogs/2026-03-24-203252-fix-release-plz-manifest-path.md`
  - `.worklogs/2026-03-24-202631-html-parser-quality-release-plumbing.md`

## Open Questions / Future Considerations
- The workflow is now trusted-publishing-ready on the GitHub side, but crates.io Trusted Publishing still needs to be enabled for every published workspace crate before `CARGO_REGISTRY_TOKEN` can be removed.
- The next live release run should prove the full sequence: release PR creation, merge, crates.io publish, public tag creation, and dist artifact upload.

## Key Files for Context
- `.github/workflows/release.yml` — release-plz orchestration and current CI publish path
- `apps/prosesmasher/Cargo.toml` — workspace dependency source of truth, including `low-expectations`
- `apps/prosesmasher/Cargo.lock` — lockfile proof that the workspace now resolves `low-expectations` from crates.io
- `.worklogs/2026-03-24-203252-fix-release-plz-manifest-path.md` — previous repair for the same release workflow
- `.worklogs/2026-03-24-202631-html-parser-quality-release-plumbing.md` — broader feature/release batch that led into this fix

## Next Steps / Continuation Plan
1. Stage this worklog with the workflow and Cargo dependency changes, then commit them together.
2. Push `main` and confirm the next `Release` workflow completes the release-PR creation step.
3. Merge the generated release PR once it is ready.
4. Watch the follow-up `Release` run publish the crate graph, create the public `v...` tag, and trigger the dist artifact workflow.
