# Tighten CLI Contract And Docs

**Date:** 2026-03-23 16:11
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`, `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/args_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/checks_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs`, `apps/prosesmasher/packages/prosesmasher/Cargo.toml`, `apps/prosesmasher/packages/prosesmasher/README.md`, `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs`

## Summary
Tightened the public CLI contract around machine-readable output and discovery: JSON mode now has clean exit semantics, `check --list-checks` exposes the shipped check catalog, text output supports compact modes, and the published README now documents the actual product surface. Also added package-level smoke tests aimed at catching published-asset regressions before release.

## Context & Problem
The latest feedback correctly called out that the published tool still had several product-level problems even after `0.1.1` fixed preset packaging:

- JSON mode was not clean enough for strict machine consumers
- exit codes did not distinguish lint failures from operational failures
- users could not discover checks directly from the CLI
- help text and README were too thin for the real config surface
- the project still lacked packaged smoke coverage for the installed CLI contract

These were not “nice-to-have” issues. They directly affected whether the tool behaved like a real CI/editor integration surface or just a locally usable CLI.

## Decisions Made

### Treat check failures as a first-class non-error outcome
- **Chose:** Split CLI outcomes into success, check-failure, and operational error.
- **Why:** Lint failure is not the same thing as command failure. The CLI now exits `1` for failing checks and `2` for operational errors.
- **Alternatives considered:**
  - Keep one generic failure exit path — rejected because it makes CI and editor integrations needlessly brittle.

### Keep JSON stdout pure on lint failures
- **Chose:** Stop routing check failures through the generic `Error: ...` stderr path.
- **Why:** For `--format json`, stdout should remain machine-readable without stderr contamination when the only outcome is failed checks.
- **Alternatives considered:**
  - Keep emitting the human stderr line and tell consumers to split streams correctly — rejected because the tool can provide a better contract itself.

### Expose a real check catalog from the CLI
- **Chose:** Add `check --list-checks`, backed by the real registered checks rather than a second hand-maintained list.
- **Why:** Check discovery should be possible from the installed CLI, including IDs, families, default-enabled state, and locale support.
- **Alternatives considered:**
  - Add a separate `list-checks` top-level subcommand — rejected because the requested and product-correct surface is `check --list-checks`.

### Make human text output compact by default
- **Chose:** Add `--text-mode` with `failures`, `full`, `summary`, and `paths`, and default text output to `failures`.
- **Why:** The old all-check text dump was too noisy for normal human usage.
- **Alternatives considered:**
  - Keep only one text mode — rejected because human CLI and CI-log needs are meaningfully different.

### Document the actual tool, not a toy surface
- **Chose:** Expand the published README and add a changelog.
- **Why:** The tool now has presets, JSON contract behavior, CI usage, machine-readable semantics, and a real configuration model. The published docs needed to match that.
- **Alternatives considered:**
  - Keep the tiny README and rely on `--help` — rejected because the installed package needs proper reference material.

### Add packaging-level smoke coverage despite nested cargo execution
- **Chose:** Add tests that call `cargo package --list` and `cargo run -p prosesmasher` from the wrapper package.
- **Why:** The previous shipped break happened at the package boundary, not in unit-level code. This test shape is the right coverage for that class of regression.
- **Alternatives considered:**
  - Only add more unit tests — rejected because unit tests would miss packaged-asset regressions again.

## Architectural Notes
This pass did not change the validation engine. It changed the CLI contract and public distribution surface:

- CLI exit semantics are now explicit and stable
- JSON output carries a schema version and high-level exit reason
- failure/check objects now carry a stable family kind
- the check catalog is derived from the real registered checks
- packaging smoke tests now guard the released CLI boundary

This also reinforces the distinction between:
- runtime prose validation behavior
- the public API/UX contract exposed by the installed CLI

## Information Sources
- prior CLI/output worklogs:
  - `.worklogs/2026-03-22-221842-hide-full-checks-behind-flag.md`
  - `.worklogs/2026-03-23-154018-embed-preset-assets-for-cli.md`
  - `.worklogs/2026-03-23-154812-vendor-preset-assets-into-fs-crate.md`
- current CLI files:
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs`
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs`
- smoke runs against `cargo run -p prosesmasher` and the installed binary behavior

## Open Questions / Future Considerations
- The config taxonomy still has some conceptual drift between execution groups (`flow`, `readability`) and config nesting (`quality.heuristics.readability`). That was documented more clearly here, but not structurally redesigned in this pass.
- False-positive suppression is still limited. The feedback was right about that, but it needs a real product design rather than a rushed patch.
- Locale support is now documented more honestly as English-first, but the multilingual heuristic story is still partial.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs` — CLI surface and help text
- `apps/prosesmasher/crates/adapters/inbound/cli/src/lib.rs` — exit semantics and command routing
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — JSON/text contract
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs` — check catalog metadata and grouping
- `apps/prosesmasher/packages/prosesmasher/README.md` — published user-facing docs
- `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs` — packaging regression coverage
- `apps/prosesmasher/CHANGELOG.md` — release notes for `0.1.2`
- `.worklogs/2026-03-23-154812-vendor-preset-assets-into-fs-crate.md` — prior packaging fix that this pass builds on

## Next Steps / Continuation Plan
1. Commit this CLI-contract/docs pass on top of `0.1.1`.
2. Publish the full crate graph as `0.1.2`, since the public wrapper and internal dependencies are already on crates.io at `0.1.1`.
3. Reinstall `prosesmasher 0.1.2` from crates.io and re-run the machine-readable smoke checks:
   - `check --format json` on a failing file
   - `check --list-checks --format json`
   - `dump-config --preset article-en`
