# Add preset CLI and config dump flow

**Date:** 2026-03-22 20:11
**Scope:** `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/args_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs`, `apps/prosesmasher/presets/README.md`, `apps/prosesmasher/presets/full-config-en.json`

## Summary
Added the missing preset/config CLI surface so the validator is usable without knowing internal file paths: `list-presets`, `dump-config`, and `check --preset`. Tightened the command semantics so `check` requires exactly one source (`--preset` or `--config`) and moved the full config file to a top-level shipped location instead of treating it like an example subtree.

## Context & Problem
After the preset cleanup, the repository had shipped preset JSON files but no public CLI for discovering or using them. The in-progress implementation also carried a bad assumption from the previous pass: it still talked about "examples" and left `check` able to run with neither `--preset` nor `--config`, which contradicted the user requirement that validation should be driven by one explicit config source.

The user clarified three concrete constraints:
- no invented `example` mode in the CLI surface
- only `dump-config --preset <name>` and `dump-config --full-config`
- global `--help` and `-h` should explain the real command combinations and usage patterns

## Decisions Made

### Make config source explicit for `check`
- **Chose:** Require exactly one of `--preset` or `--config` on `check`.
- **Why:** This matches the product model: validation should run against a concrete shipped preset or a concrete user config, not an implicit default path hidden behind the CLI.
- **Alternatives considered:**
  - Keep falling back to `CheckConfig::default()` when no source is provided — rejected because it hides the config contract and contradicts the user’s "either preset or config" requirement.
  - Allow both `--preset` and `--config` and merge them — rejected because there was no agreed merge model and the user explicitly wanted them mutually exclusive.

### Expose preset discovery and config dumping as first-class commands
- **Chose:** Add `list-presets` and `dump-config`.
- **Why:** Shipped presets are not useful unless the CLI can tell the user what exists and can print the exact JSON they can inspect or copy.
- **Alternatives considered:**
  - Add `init` — rejected because this is a validator, not a project scaffolder, and the verb is ambiguous.
  - Hide preset knowledge in docs only — rejected because discoverability should be in the binary, not just in repository prose.

### Remove "example" framing from the user-facing surface
- **Chose:** Treat the shipped files as presets plus one full config, and move `full-config-en.json` to the top-level preset directory.
- **Why:** The user explicitly rejected the invented "example" concept for the CLI. The filesystem layout should reinforce the public API instead of contradicting it.
- **Alternatives considered:**
  - Keep `presets/examples/full-config-en.json` and only rename the CLI text — rejected because the directory structure would still leak the wrong concept into future maintenance.
  - Rename `list-presets` into a broader `list-configs` command — rejected because the user specifically asked for a presets API and the only non-preset artifact is the one-off full config dump.

### Put the real workflow into top-level help text
- **Chose:** Expand the global Clap help with a workflow section, explicit rules, and concrete command combinations.
- **Why:** The user wanted `--help` / `-h` to show the real usage patterns directly instead of forcing subcommand-by-subcommand discovery.
- **Alternatives considered:**
  - Keep only terse subcommand docs — rejected because the top-level command relationships (`list-presets`, `dump-config`, `check --preset`, `check --config`) are the whole usability story.

## Architectural Notes
The adapter boundary is now cleaner:
- outbound FS owns the shipped preset catalog and path resolution
- inbound CLI owns user-facing command semantics and discovery text
- app/domain stay untouched

This keeps the preset mechanism file-backed and explicit without smuggling preset knowledge into the core validation engine. The only filesystem convention exposed is the shipped preset registry in `adapters/outbound/fs`, which the CLI consumes through small helper functions.

The full config file is still a static JSON artifact, not generated at runtime. That keeps `dump-config --full-config` predictable and versioned alongside the shipped presets.

## Information Sources
- `.worklogs/2026-03-22-195213-structure-only-presets.md` — latest preset philosophy and the decision that real presets differ only by structure
- `.worklogs/2026-03-22-194123-split-preset-examples.md` — prior split that this change intentionally simplified
- `.worklogs/2026-03-22-192945-rationalize-presets.md` — earlier preset taxonomy cleanup
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs` — existing Clap surface
- `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs` — runtime dispatch and error handling
- `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs` — best place to centralize shipped preset lookup
- Verification runs:
  - `cargo test -q -p prosesmasher-adapters-inbound-cli`
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- --help`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- list-presets`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- dump-config --full-config`
  - `cargo run -q -p prosesmasher-adapters-inbound-cli -- dump-config --preset article-en`

## Open Questions / Future Considerations
- `dump-config` currently prints static JSON files. If the shipped preset/full-config set later becomes generated from code defaults, this command may need to switch to rendering the normalized config instead of reading files verbatim.
- The help text is now explicit, but if the binary name is changed from the current package name to a published `prosesmasher` binary target, help examples should be rechecked so they match the installed executable exactly.
- Default config discovery is still not implemented. This change intentionally kept the command model explicit rather than adding hidden config lookup behavior.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs` — top-level CLI contract, subcommands, and help text
- `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs` — command dispatch, preset/config loading, dump behavior
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args_tests.rs` — parser coverage for required combinations and conflicts
- `apps/prosesmasher/crates/adapters/outbound/fs/src/lib.rs` — shipped preset registry and file lookup helpers
- `apps/prosesmasher/presets/full-config-en.json` — the single full-surface dump artifact exposed by `dump-config --full-config`
- `apps/prosesmasher/presets/README.md` — user-facing explanation of presets vs full config
- `.worklogs/2026-03-22-195213-structure-only-presets.md` — the preset model this CLI now exposes

## Next Steps / Continuation Plan
1. Add binary-target polish if needed so help output uses the final executable name rather than the current crate/package name when run through `cargo run`.
2. Decide whether `list-presets` should eventually emit JSON as well as text for scripting, or whether plain text is enough for the immediate product surface.
3. Revisit config loading errors in `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader.rs` so unreadable configs report the correct error class instead of `NotFound`.
