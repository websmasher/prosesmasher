# Embed Preset Assets For CLI

**Date:** 2026-03-23 15:40
**Scope:** `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs`

## Summary
Replaced repo-relative preset file loading with compile-time embedded preset JSON so `cargo install` builds of `prosesmasher` can use `--preset` and `dump-config` without access to the monorepo layout. Also centralized JSON config parsing so both file-backed configs and embedded presets share the same validation path.

## Context & Problem
The published `prosesmasher 0.1.0` CLI advertised `list-presets`, `dump-config`, and `check --preset`, but the installed binary failed for preset-backed commands. The root cause was a workspace-path assumption in the filesystem adapter:

- preset names/descriptions were compiled into Rust
- preset contents were resolved from `CARGO_MANIFEST_DIR/../../../../presets`
- that path only exists in the repo checkout, not in Cargo’s installed registry source layout

This made the crates.io release look usable from `--help` while breaking the actual preset workflow after `cargo install`.

## Decisions Made

### Embed preset JSON at compile time
- **Chose:** Replace `preset_path()` / `full_config_path()` with `preset_contents()` / `full_config_contents()` using `include_str!`.
- **Why:** Embedded assets remove the repo-layout dependency entirely and guarantee preset availability in packaged crates.
- **Alternatives considered:**
  - Ship JSON files as package data and resolve them relative to the packaged crate — rejected because it still relies on filesystem lookup and packaging details.
  - Keep workspace-relative path resolution and try to special-case published crates — rejected because it preserves the wrong abstraction and is fragile.

### Parse embedded presets through the same config validation path
- **Chose:** Add `parse_config_json()` in the FS adapter and reuse it for both file-loaded configs and embedded preset strings.
- **Why:** This avoids duplicating serde/garde/domain conversion logic and keeps preset validation behavior identical to `--config`.
- **Alternatives considered:**
  - Duplicate DTO parsing inside the CLI adapter — rejected because it would split config semantics across layers.

### Keep the CLI surface unchanged
- **Chose:** Preserve `--preset`, `--full-config`, and `list-presets` UX and only swap out the underlying asset source.
- **Why:** The bug was packaging, not product semantics. The user-facing commands were already the right shape.
- **Alternatives considered:**
  - Remove preset commands from the published CLI — rejected because presets are a core usability feature and should work everywhere.

## Architectural Notes
Preset handling now has a clean separation:

- FS adapter owns shipped preset metadata and content
- CLI adapter asks for preset contents, not file paths
- file-backed configs still go through `FsConfigLoader`
- embedded presets use the same DTO/domain validation via `parse_config_json()`

This removes the workspace leak from the published crate graph without changing the runtime validation architecture.

## Information Sources
- Installed-crate failure observed in `~/.cargo/bin/prosesmasher`
- `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs`
- `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs`
- `apps/prosesmasher/presets/*.json`
- Prior release/publish worklogs:
  - `.worklogs/2026-03-23-094745-set-up-publishable-wrapper-crate.md`
  - `.worklogs/2026-03-23-101121-add-release-plz-workflow.md`

## Open Questions / Future Considerations
- The installed binary still prints a human `Error: One or more checks failed` on stderr after JSON output when checks fail. That is separate from the preset packaging bug, but it remains awkward for strict machine consumers that merge stdout/stderr.
- This fix should be republished as a new crates.io release; `0.1.0` remains broken on crates.io until a new version is cut.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs` — shipped preset registry and embedded preset contents
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader.rs` — shared JSON config parsing/validation path
- `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs` — CLI preset and dump-config wiring
- `apps/prosesmasher/presets/article-en.json` — representative shipped preset content
- `apps/prosesmasher/presets/full-config-en.json` — full editable config surface
- `.worklogs/2026-03-23-094745-set-up-publishable-wrapper-crate.md` — installable wrapper crate background
- `.worklogs/2026-03-23-101121-add-release-plz-workflow.md` — current release workflow background

## Next Steps / Continuation Plan
1. Bump the relevant crate versions so the fixed preset packaging can be republished to crates.io.
2. Publish a corrected `prosesmasher` release and verify `cargo install prosesmasher` from crates.io, not just `cargo install --path`.
3. Decide whether to fix the stderr JSON/exit-code ergonomics for machine consumers in the same release train or keep that as a separate follow-up.
