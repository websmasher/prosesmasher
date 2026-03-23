# Changelog

## 0.1.2

- Added clean machine-readable CLI semantics:
  - exit `0` for success
  - exit `1` for check failures
  - exit `2` for operational failures
- JSON mode now keeps stdout pure JSON on check failures and keeps stderr empty in that case.
- Added `schema_version`, `exit_reason`, and failure/check `kind` fields to JSON output.
- Added `check --list-checks` with check IDs, groups, default-enabled state, and supported locales.
- Added text output modes: `failures`, `full`, `summary`, and `paths`, with failure-focused text as the default.
- Expanded the published README with install, presets, config semantics, CI usage, limitations, and audience guidance.
- Added packaged smoke tests to catch published-asset regressions.

## 0.1.1

- Fixed published `cargo install` builds so preset-backed commands work:
  - `check --preset ...`
  - `dump-config --preset ...`
  - `dump-config --full-config`
- Embedded shipped preset JSON into the published crate graph instead of relying on monorepo-relative asset paths.
- Added stricter CLI exit semantics:
  - `0` for success
  - `1` for check failures
  - `2` for operational failures
- JSON mode now keeps stdout machine-readable on lint failures and reports `schema_version`, `exit_reason`, and failure `kind`.
- Added `check --list-checks`.
- Added text output modes: `failures`, `full`, `summary`, `paths`.
- Tightened publish metadata for the internal crate graph.

## 0.1.0

- Initial crates.io release.
- Included the core prose validation engine, shipped presets, JSON output, and document-policy checks.
- Preset-backed commands in the published package were broken because preset assets were not packaged correctly. This is fixed in `0.1.1`.
