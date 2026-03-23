# Vendor Preset Assets Into FS Crate

**Date:** 2026-03-23 15:48
**Scope:** `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/presets/*.json`

## Summary
Moved the embedded preset JSON assets into the filesystem adapter crate itself and switched `include_str!` to crate-local paths. This fixes the publish-time verification failure that remained after the initial preset embedding change.

## Context & Problem
The first preset fix embedded JSON with `include_str!`, but the included paths still pointed from the FS crate source file out to the monorepo-level `apps/prosesmasher/presets` directory. That was enough for local builds, but `cargo publish` caught the packaging flaw:

- local builds compiled because the monorepo path existed
- packaged tarball verification failed because the published crate cannot reliably resolve assets outside its own packaged root

The actual symptom appeared while publishing `prosesmasher-adapters-outbound-fs v0.1.1`: package verification failed before upload because the `include_str!` paths were invalid in the packaged layout.

## Decisions Made

### Vendor preset JSON into the FS crate
- **Chose:** Copy the shipped preset JSON files into `crates/adapters/outbound/fs/presets/`.
- **Why:** The FS adapter is the crate that owns shipped preset access, so the preset assets should live inside that crate’s published package boundary.
- **Alternatives considered:**
  - Keep referencing the top-level workspace `presets/` directory — rejected because it breaks `cargo publish` verification.
  - Generate presets at build time from the workspace root — rejected because it still reintroduces workspace assumptions into a published crate.

### Use crate-local `include_str!` paths
- **Chose:** Update `src/lib.rs` to load `../presets/*.json`.
- **Why:** This makes the package self-contained and stable in both local and packaged builds.
- **Alternatives considered:**
  - Use `env!(\"CARGO_MANIFEST_DIR\")` with different relative path math — rejected because it still depends on non-local assets unless the presets are already inside the crate.

## Architectural Notes
This makes shipped presets a true responsibility of the FS adapter crate:

- preset names/descriptions remain in Rust
- preset contents now live inside the crate package
- CLI `--preset` and `dump-config` no longer depend on any workspace-level asset path, either at runtime or package verification time

This is the proper boundary for published distribution.

## Information Sources
- `cargo publish --dry-run -p prosesmasher-adapters-outbound-fs`
- `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs`
- `apps/prosesmasher/presets/*.json`
- `.worklogs/2026-03-23-154018-embed-preset-assets-for-cli.md`
- `.worklogs/2026-03-23-154505-prepare-and-release-0-1-1.md`

## Open Questions / Future Considerations
- The workspace now has two copies of the preset JSON: top-level product presets and FS-crate packaged presets. If preset churn increases, we may want a generation/sync step, but for now duplication is acceptable because it fixes the released package boundary cleanly.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs` — crate-local embedded preset loading
- `apps/prosesmasher/crates/adapters/outbound/fs/presets/article-en.json` — representative packaged preset asset
- `apps/prosesmasher/presets/article-en.json` — top-level product preset source currently mirrored
- `.worklogs/2026-03-23-154018-embed-preset-assets-for-cli.md` — first-stage preset embedding change
- `.worklogs/2026-03-23-154505-prepare-and-release-0-1-1.md` — release-hardening and version bump context

## Next Steps / Continuation Plan
1. Commit this crate-local preset packaging fix.
2. Resume publishing `0.1.1` from `prosesmasher-adapters-outbound-fs`, since `domain-types` and `ports-outbound-traits` are already live.
3. Continue the ordered publish chain through parser, app-core, inbound CLI, and public wrapper, handling crates.io rate limits as needed.
