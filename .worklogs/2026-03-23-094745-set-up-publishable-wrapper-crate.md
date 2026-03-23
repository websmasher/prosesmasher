# Set Up Publishable Wrapper Crate

**Date:** 2026-03-23 09:47
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/crates/domain/types/Cargo.toml`, `apps/prosesmasher/crates/ports/outbound/traits/Cargo.toml`, `apps/prosesmasher/crates/app/core/Cargo.toml`, `apps/prosesmasher/crates/adapters/outbound/fs/Cargo.toml`, `apps/prosesmasher/crates/adapters/outbound/parser/Cargo.toml`, `apps/prosesmasher/crates/adapters/inbound/cli/Cargo.toml`, `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs`, `apps/prosesmasher/packages/prosesmasher/*`

## Summary
Added a thin publishable wrapper crate named `prosesmasher` while keeping the existing hexagonal workspace intact. The CLI entrypoint now lives in the CLI adapter library, internal crates have publish metadata plus versioned path dependencies, and dry-runs show the remaining blockers are publication order and the external `low-expectations` crate rather than workspace structure.

## Context & Problem
The user wanted `cargo install prosesmasher` support without flattening the architecture. The existing workspace could not support that:
- every crate was `publish = false`
- the runnable CLI package was named `prosesmasher-adapters-inbound-cli`
- internal crates depended on each other by path only
- `app/core` and the CLI adapter depended on `low-expectations` via a local path outside the workspace

The design constraint was to preserve the hex split and make the installable crate an alias/wrapper for the CLI adapter, not a rewrite of it.

## Decisions Made

### Add a thin user-facing wrapper crate
- **Chose:** Create `apps/prosesmasher/packages/prosesmasher` with the public package name `prosesmasher` and a tiny `src/main.rs` that calls into the CLI adapter.
- **Why:** This gives users a clean install target while leaving the existing CLI adapter package in place as the actual composition-root implementation.
- **Alternatives considered:**
  - Rename the adapter package directly to `prosesmasher` — rejected because it would blur the adapter role and make the internal architecture less explicit.
  - Flatten the whole workspace into one binary crate — rejected because it would throw away the hex structure for packaging convenience.

### Move the CLI entrypoint into the adapter library
- **Chose:** Move the `main` logic into `prosesmasher_adapters_inbound_cli::main_entry()` / `run(...)`, and make the binary targets call that function.
- **Why:** Both the internal binary and the new public wrapper need the same CLI behavior. The library API avoids duplication and keeps the wrapper extremely thin.
- **Alternatives considered:**
  - Duplicate the full CLI main logic in the wrapper crate — rejected because it would create two composition roots that drift over time.

### Make internal crates publishable with versioned internal deps
- **Chose:** Add publish metadata (`rust-version`, `license`, `repository`, `description`) and convert internal dependencies to `version + path`.
- **Why:** A crates.io-installable wrapper requires the entire dependency chain to be publishable in order.
- **Alternatives considered:**
  - Keep internal crates unpublished and rely on path deps — rejected because published crates cannot depend on unpublished local workspace packages.

## Architectural Notes
The installable package now sits outside the internal hex tree:
- `crates/...` remains the architecture proper
- `packages/prosesmasher` is a distribution wrapper

That mirrors the right boundary:
- adapter crate owns CLI semantics
- wrapper crate owns package/install ergonomics

The entrypoint refactor also makes it easier to add a future `cargo-binstall` wrapper or release-specific package without moving core CLI logic again.

## Information Sources
- `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs` and `src/lib.rs` — existing CLI entrypoint and adapter shape
- `apps/prosesmasher/Cargo.toml` — current workspace layout
- `pipelin3r` release layout for comparison:
  - `/Users/tartakovsky/Projects/websmasher/pipelin3r/packages/shedul3r-bin/Cargo.toml`
  - `/Users/tartakovsky/Projects/websmasher/pipelin3r/.worklogs/2026-03-15-161036-binary-release-setup.md`
  - `/Users/tartakovsky/Projects/websmasher/pipelin3r/.github/workflows/release-binary.yml`
- `cargo publish --dry-run` results on:
  - `prosesmasher-domain-types` — passes
  - higher crates — fail only because dependent crates are not yet on crates.io

## Open Questions / Future Considerations
- `low-expectations` must be published before `prosesmasher-app-core` and anything above it can verify/publish cleanly.
- The internal crates are now publishable in principle, but they may want dedicated README files later if you care about crates.io polish for the implementation packages.
- We have not yet added a `cargo-binstall` release path for `prosesmasher`; this change only establishes the `cargo install`-compatible structure.

## Key Files for Context
- `apps/prosesmasher/packages/prosesmasher/Cargo.toml` — user-facing installable package
- `apps/prosesmasher/packages/prosesmasher/src/main.rs` — thin wrapper entrypoint
- `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs` — reusable CLI entry logic
- `apps/prosesmasher/Cargo.toml` — workspace members and shared publish metadata
- `apps/prosesmasher/crates/adapters/inbound/cli/Cargo.toml` — publishable adapter package wiring
- `.worklogs/2026-03-22-221842-hide-full-checks-behind-flag.md` — most recent CLI/output contract context

## Next Steps / Continuation Plan
1. Publish `low-expectations` first or make it publish-ready in its own repo, because `prosesmasher-app-core` is blocked on that external crate.
2. Publish `prosesmasher-domain-types`, then `prosesmasher-ports-outbound-traits`, then the parser/fs adapters, then `prosesmasher-app-core`, then `prosesmasher-adapters-inbound-cli`, then the public `prosesmasher` wrapper.
3. After the dependency chain is published, rerun `cargo publish --dry-run -p prosesmasher` to confirm the installable wrapper verifies cleanly against crates.io.
4. Decide whether to add a parallel `cargo-binstall` path with GitHub Releases after the `cargo install` path is working.
