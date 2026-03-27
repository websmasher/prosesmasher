# Setup Binstall GitHub Release Surface

**Date:** 2026-03-27 14:16
**Scope:** `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `.github/workflows/release.yml`, `README.md`, `apps/prosesmasher/CHANGELOG.md`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, workspace crate manifests under `apps/prosesmasher/**/Cargo.toml`, `apps/prosesmasher/packages/prosesmasher/Cargo.toml`, `apps/prosesmasher/packages/prosesmasher/README.md`, `apps/prosesmasher/release-plz.toml`

## Summary
Reworked the public release surface so `prosesmasher` installs from GitHub release binaries via `cargo-binstall --git ...` instead of relying on crates.io publishing for the whole internal workspace. Added a repo-root shim package with explicit `cargo-binstall` metadata, switched release automation to tag-driven `cargo-dist`, bumped the project to `0.3.5`, and verified both the root install path and the dist artifact layout locally.

## Context & Problem
The existing release story was internally inconsistent with the desired distribution model:

- the repo already had `cargo-dist` GitHub release packaging
- the public docs already implied `cargo-binstall` support
- but the active release workflow still used `release-plz` to publish multiple internal crates

The user explicitly did not want to keep releasing the internal crate graph just to make the CLI installable. A quick dry-run of `cargo publish -p prosesmasher` on the existing structure confirmed the deeper issue: the published CLI package depends on internal crates by semver, so a normal crates.io-only publish path requires those internal crates to be published at matching versions too.

That meant the practical target had to become:

1. GitHub Releases as the canonical binary distribution channel
2. `cargo-binstall --git https://github.com/websmasher/prosesmasher prosesmasher` as the fast install path
3. `cargo install --git https://github.com/websmasher/prosesmasher prosesmasher` as the source fallback

## Decisions Made

### Add A Repo-Root Shim Package For Git Installs
- **Chose:** Add a new root `Cargo.toml` + `src/main.rs` package named `prosesmasher` that simply delegates to `prosesmasher_adapters_inbound_cli::main_entry()`.
- **Why:** `cargo-binstall --git` clones the repository root and resolves package metadata from that manifest. The existing installable binary crate lived inside `apps/prosesmasher/packages/prosesmasher`, so there was no root manifest for binstall to own.
- **Alternatives considered:**
  - Make the repository root a workspace pointing at `apps/prosesmasher/packages/prosesmasher` — rejected because the outer workspace hijacked the inner workspace’s `workspace.package`, `workspace.dependencies`, and `workspace.lints` inheritance and broke manifest resolution.
  - Keep only the nested package and require users to pass a nested manifest path — rejected because that is not a viable public install story.

### Use Explicit `cargo-binstall` Metadata And Disable Source Fallback
- **Chose:** Add root `package.metadata.binstall` with:
  - `bin-dir = "{ name }-{ target }/{ bin }{ binary-ext }"`
  - `disabled-strategies = ["quick-install", "compile"]`
  - `pkg-fmt = "zip"` on Windows
  - `pkg-fmt = "txz"` elsewhere
- **Why:** The user wanted a precompiled-only binstall path. The metadata now tells `cargo-binstall` to look for the exact `cargo-dist` archive layout on GitHub Releases and refuse to silently fall back to source compilation.
- **Alternatives considered:**
  - Rely on `cargo-binstall` defaults with no metadata — rejected because the default strategy set includes compile fallback and the archive format/directory layout would be implicit rather than pinned.
  - Hardcode `pkg-url` immediately — rejected because `cargo-binstall`’s built-in GitHub release templates already cover the observed `cargo-dist` asset names once `pkg-fmt` and `bin-dir` are set correctly.

### Keep The Nested Package Distable
- **Chose:** Restore `apps/prosesmasher/packages/prosesmasher/Cargo.toml` so it is visible to `cargo-dist`.
- **Why:** Marking the nested package `publish = false` made `cargo-dist` think the workspace had nothing releasable. Local `cargo dist build --artifacts=lies` immediately failed until that was reverted.
- **Alternatives considered:**
  - Force distability with extra `package.metadata.dist` overrides while keeping `publish = false` — rejected because the simpler and more predictable answer was to stop conflating crates.io publishability with GitHub binary release packaging.
  - Move dist to the root shim package — rejected because the existing inner workspace already has the correct packaging and test coverage around the real CLI package.

### Replace `release-plz` Automation With Tag-Driven `cargo-dist`
- **Chose:** Rewrite `.github/workflows/release.yml` to trigger on `v*` tag pushes and call the existing reusable `dist.yml` workflow directly.
- **Why:** Once GitHub Releases became the canonical public surface, the old `release-plz` / crates.io pipeline was the wrong automation model and was actively pushing toward publishing internal crates.
- **Alternatives considered:**
  - Keep `release-plz` and merely stop publishing most crates — rejected because the public install path being set up here does not need crates.io publishing at all.
  - Keep the dead `release-plz.toml` around — rejected because after removing the workflow there were no remaining references, so keeping it would only preserve confusion.

### Bump The Project To `0.3.5`
- **Chose:** Bump the root shim and the full `apps/prosesmasher` workspace from `0.3.4` to `0.3.5`.
- **Why:** The public release/install surface changed materially:
  - new supported install commands
  - new root package
  - new release automation model
  - versioned GitHub release URLs used by `cargo-binstall`
- **Alternatives considered:**
  - Leave the version at `0.3.4` — rejected because that would make the release/install contract change invisible and would not line up with the GitHub release tag expected by the new setup.

## Architectural Notes
- The repo now has two package entrypoints with the same binary name but different purposes:
  - repo-root `prosesmasher`: public git/binstall shim, `publish = false`
  - `apps/prosesmasher/packages/prosesmasher`: canonical packaged CLI used by the inner workspace and `cargo-dist`
- This split is intentional. It keeps the public install surface simple without flattening the actual application workspace.
- `cargo-binstall` now resolves from the root manifest, then targets GitHub release assets that are still built from the real inner workspace package.
- `cargo-dist` continues to run from `apps/prosesmasher`, and local `dist` inspection showed the exact archive layout:
  - assets like `prosesmasher-x86_64-unknown-linux-gnu.tar.xz`
  - archive root like `prosesmasher-x86_64-unknown-linux-gnu/`
  - binary path like `prosesmasher-x86_64-unknown-linux-gnu/prosesmasher`

## Information Sources
- Local code and release config:
  - `.github/workflows/dist.yml`
  - previous `.github/workflows/release.yml`
  - `apps/prosesmasher/dist-workspace.toml`
  - `apps/prosesmasher/packages/prosesmasher/Cargo.toml`
- Current `cargo-binstall` CLI behavior:
  - `cargo-binstall -V` -> `1.17.7`
  - `cargo binstall --help`
  - debug run showing root metadata resolution and GitHub release URL probing
- Local verification commands run in this pass:
  - `cargo install --path . --root /tmp/prosesmasher-install-test-035`
  - `/tmp/prosesmasher-install-test-035/bin/prosesmasher --version`
  - `cargo dist build --artifacts=lies --allow-dirty --tag=v0.3.5 --output-format=json`
  - `tar -tf apps/prosesmasher/target/distrib/prosesmasher-aarch64-apple-darwin.tar.xz`
  - `cargo binstall --manifest-path Cargo.toml --no-confirm --dry-run --log-level debug prosesmasher`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - `cargo run --manifest-path apps/prosesmasher/Cargo.toml -q -p prosesmasher -- --version`
  - `cargo run --manifest-path Cargo.toml -q -- --version`
- Prior worklog for project state before this release-surface work:
  - `.worklogs/2026-03-27-134731-add-short-form-slop-families.md`

## Open Questions / Future Considerations
- `cargo-dist` still emits a `source.tar.gz` artifact alongside the binary archives. That does not affect the binstall path, but if the product decision becomes “GitHub releases must contain binaries only,” the dist configuration should be revisited explicitly rather than assumed.
- The root shim is intentionally `publish = false`. If a future requirement brings back plain `cargo install prosesmasher` / `cargo binstall prosesmasher` from crates.io, the package graph will need a different solution than this shim because the current internal dependency tree cannot be published as a single crate without also releasing its semver dependencies.
- The repo now has a version boundary that depends on tag pushes (`v0.3.5`, `v0.3.6`, ...). Any future release automation should preserve that assumption unless the binstall metadata is redesigned.

## Key Files for Context
- `Cargo.toml` — repo-root shim package and the public `cargo-binstall` metadata
- `src/main.rs` — root shim main entrypoint delegating to the real CLI
- `.github/workflows/release.yml` — tag-driven GitHub release workflow
- `.github/workflows/dist.yml` — reusable `cargo-dist` release pipeline
- `apps/prosesmasher/dist-workspace.toml` — `cargo-dist` package/target/install metadata
- `README.md` — public install instructions for binstall and source install
- `apps/prosesmasher/packages/prosesmasher/Cargo.toml` — canonical inner CLI package that `cargo-dist` packages
- `apps/prosesmasher/packages/prosesmasher/README.md` — nested package documentation kept in sync with the new install story
- `apps/prosesmasher/CHANGELOG.md` — `0.3.5` release note for this release-surface change
- `.worklogs/2026-03-27-134731-add-short-form-slop-families.md` — previous project-state worklog before the distribution setup change

## Next Steps / Continuation Plan
1. Push a `v0.3.5` tag after this commit and verify the GitHub Actions `Release` workflow publishes the four platform archives plus installer scripts.
2. After the release exists, run a real end-to-end install test from a clean location:
   - `cargo binstall --git https://github.com/websmasher/prosesmasher prosesmasher`
   - `prosesmasher --version`
3. If the release needs a smoother maintainer workflow, add a short `RELEASING.md` or Make/just task that documents:
   - bump version
   - commit
   - tag `vX.Y.Z`
   - push commit + tag
4. If future public installs should not require `--git`, revisit the package graph rather than trying to patch around it piecemeal. The current setup is intentionally optimized for GitHub binary releases, not crates.io registry distribution.
