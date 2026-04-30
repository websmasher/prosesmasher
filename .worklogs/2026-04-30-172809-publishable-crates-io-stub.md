# Publishable crates.io stub for binstall without --git

## Summary

Restructured the repo-root `prosesmasher` package into a publishable crates.io stub so `cargo binstall prosesmasher` resolves without the `--git` flag. The stub holds only the binstall metadata that points at the GitHub release artifacts. The real CLI continues to be built and shipped from `apps/prosesmasher/packages/prosesmasher` via cargo-dist.

## Decisions

The 2026-03-27 worklog "Setup Binstall GitHub Release Surface" left the door open: "If a future requirement brings back plain `cargo install prosesmasher` / `cargo binstall prosesmasher` from crates.io, the package graph will need a different solution than this shim." That requirement arrived. Cleanest solution: keep the shim; remove its real dependency; flip `publish = true`.

**Stub package shape:**
- Root `Cargo.toml` has no `[dependencies]` block, no path deps, no internal coupling.
- Root `src/main.rs` is a 12-line `eprintln!` + `ExitCode::from(1)`.
- `[package.metadata.binstall]` block stays unchanged so binstall finds the GitHub release artifacts at the matching tag.

**Why a runtime error instead of a build error:** A `compile_error!` or `build.rs` panic would block `cargo publish --verify` itself, requiring `--no-verify` and brittle workarounds. The runtime-error path means the stub builds cleanly on crates.io, but anyone who actually runs the resulting binary gets a loud, instruction-rich message and a non-zero exit. Tradeoff accepted: `cargo install prosesmasher` from crates.io will succeed-then-fail-at-runtime rather than fail-at-install. The error message is unambiguous about what went wrong and how to fix it.

**Why not publish the inner workspace tree:** Substantial coordination work (30+ crates in dep order, version-locked, with a hard external dep on `low-expectations` not yet on crates.io). The stub approach is the smallest change that satisfies the actual user need (binstall without flags).

**Both `prosesmasher` packages are intentionally name-collisioned:**
- Root `/Cargo.toml` — crates.io-published stub, `publish = true` (default).
- `apps/prosesmasher/packages/prosesmasher/Cargo.toml` — real CLI, never published, built by cargo-dist into the GitHub release artifact.

cargo doesn't see them as a conflict because they're in separate workspaces with separate `Cargo.lock` files.

## Audit

- `cargo publish --dry-run --allow-dirty` from the root packages 502 files (1.8 MiB), verifies cleanly.
- Local `./target/release/prosesmasher` (the stub) prints the error message and exits 1 as expected.
- `~/.cargo/bin/prosesmasher` is the v0.3.13 working CLI from earlier `cargo install --path .` and is unaffected by these changes.

After v0.3.14 is published:
- `cargo binstall prosesmasher` (no flags) → reads metadata from crates.io v0.3.14 → fetches the v0.3.14 GitHub release artifact → installs the real CLI.
- `cargo install prosesmasher` from crates.io → installs the stub → running it prints the error.
- `cargo binstall --git ...` and `cargo install --git ...` continue to work as documented.

## Key files

- `Cargo.toml` — stub package, no deps, binstall metadata
- `src/main.rs` — 12-line runtime error
- `apps/prosesmasher/CHANGELOG.md` — 0.3.14 entry
- All `apps/prosesmasher/**/Cargo.toml` — bumped 0.3.13 → 0.3.14 for tag/release-version consistency

## Next steps

1. Tag `v0.3.14`, push commit + tag. The existing `release.yml` workflow auto-builds the GitHub release artifacts from `apps/prosesmasher/packages/prosesmasher`.
2. After the GitHub release exists at v0.3.14, run `cargo publish` from the repo root to push the stub to crates.io. Requires a `CARGO_REGISTRY_TOKEN` available in the local shell.
3. Verify with a clean test: `cargo binstall prosesmasher` (no `--git`) should succeed.
4. If the publish is going to be recurring, add a workflow step that runs `cargo publish` from the root after the GitHub release succeeds. Today it's manual.
