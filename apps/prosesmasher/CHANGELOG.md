# Changelog

## 0.2.2

- Tightened `softening-language` so a weaker `may + often` sentence no longer fails on its own.
- Kept the stronger Medical Outline softening hits intact while dropping the borderline `why_do_we_dream` baseline.

## 0.2.1

- Added `softening-language` as the next accumulative `llm-slop` rule for repeated low-commitment phrasing such as modal-heavy variability disclaimers and tentative reporting.
- Expanded the fixture regression sidecars to cover the new real-world hits in `why_do_we_dream` and the Medical Outline corpus.
- Kept the packaged CLI release contract intact while bumping the workspace to the next semantic version.

## 0.2.0

- Added the `llm-slop` family with the first live rules for:
  - `llm-disclaimer`
  - `response-wrapper`
  - `generic-signposting`
  - `boilerplate-framing`
  - `llm-vocabulary`
- Split the old heuristic bucket into smaller owned rule families and aligned their sidecar/assertions test structure with the GuardRails test architecture.
- Added per-fixture expected-failure sidecars for the growing corpus, including the new Medical Outline AI-written fixture set.
- Standardized release versioning across the workspace and now enforce `prosesmasher --version` through packaged smoke coverage.

## 0.1.7

- Preserve visible text inside raw HTML embedded in markdown instead of dropping it. This fixes aside-heavy content copied from tools like Notion.
- Narrow the em-dash heuristic so it flags only closed em dashes like `word—word`, while allowing spaced forms like `word — word`.
- Reclassify `sentence-case` as a general quality heuristic while keeping its current heading-targeted behavior.
- Clarify internal crate metadata on crates.io and tighten the release/distribution plumbing around the public `prosesmasher` package.

## 0.1.6

- Refreshed the GitHub-facing and crates.io-facing README copy after the public repo launch.
- Tightened the opener and command walkthrough text so the docs read like product documentation instead of generic tool boilerplate.
- Removed the stale `CLAUDE.md` repo note from version control.

## 0.1.5

- Extended `negation-reframe` with a narrow internal-state corrective branch so it catches patterns like:
  - `don't stop having feelings`
  - `they stop showing them`
- Kept that branch narrow enough to avoid generic behavioral follow-up pairs such as `Children don't stop at the corner. They turn left instead.`

## 0.1.4

- Added a new `fragment-stacking` heuristic to catch clipped cadence runs such as `Short. Short. Longer payoff.` without emitting overlapping duplicate hits.
- Broadened affirmation-closer detection to catch short standalone `That's the ...` formula sentences.
- Tightened corrective-negation detection:
  - still catches `Not to X. To Y.`
  - still catches `This isn't X. It's Y.`
  - now also catches curated same-root framing pairs like `does not mean -> it means`
  - avoids broad generic same-root matches outside the framing verb family
- Updated the `substack-en` preset word-count envelope to `500..1000`.

## 0.1.3

- Realigned the canonical config taxonomy with the public check families:
  - `quality.heuristics` for rhetorical/style heuristics
  - `quality.flow` for paragraph and repetition controls
  - `quality.readability` for readability thresholds
- Updated the domain config model, JSON loader, full config example, shipped preset assets, fixtures, and docs to use the new shape.
- Removed the remaining drift where flow/readability settings were still nested under `quality.heuristics`.

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
