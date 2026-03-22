# Canonical docs and preset surface

**Date:** 2026-03-22 18:23
**Scope:** `AGENTS.md`, `apps/prosesmasher/presets/*.json`, `apps/prosesmasher/crates/adapters/inbound/cli/tests/fixtures/test-config.json`, `apps/prosesmasher/crates/adapters/outbound/fs/tests/fixtures/sample-config.json`

## Summary
Updated the public-facing documentation and shipped/sample config artifacts to match the new canonical config model. The handoff doc, example CLI references, sample fixtures, and curated presets now describe `quality` / `documentPolicy` and `prohibited-terms` instead of teaching the legacy `terms` / `thresholds` taxonomy.

## Context & Problem
After the config and heuristic migrations, the runtime behavior and the public documentation had drifted apart. The loader still supports the legacy schema, but that is now a compatibility path rather than the recommended interface. Leaving the old schema and old check IDs in the handoff doc and shipped configs would create immediate confusion:
- users would author the wrong shape by default
- future agent sessions would reason from stale architecture notes
- presets would continue broadcasting the deprecated model

The goal of this slice was to make the visible product surface match the actual semantic model without removing the compatibility bridge.

## Decisions Made

### Update docs to the canonical model, not the compatibility model
- **Chose:** Rewrite `AGENTS.md` examples and config documentation around `quality`, `documentPolicy`, and `prohibited-terms`.
- **Why:** The handoff doc should represent the product’s intended model, not the historical compatibility layer.
- **Alternatives considered:**
  - Document both schemas equally — rejected because that would keep the migration ambiguous and continue teaching the old taxonomy.
  - Leave docs until compatibility support is removed — rejected because that would prolong internal and external confusion.

### Convert shipped presets to canonical JSON
- **Chose:** Rewrite the preset files to use canonical `quality` / `documentPolicy` JSON.
- **Why:** Presets are product artifacts, not migration fixtures. They should model the target interface directly.
- **Alternatives considered:**
  - Keep presets on the legacy schema since the loader still accepts it — rejected because presets would then be teaching a deprecated authoring model.
  - Delay preset conversion until the compatibility bridge is gone — rejected because it would preserve the wrong surface area for no real benefit.

### Keep explicit lexical lists in presets for now
- **Chose:** Encode the curated preset lexical choices with `defaults: false` and explicit `add` lists rather than leaning on library defaults.
- **Why:** This preserves the preset-specific editorial intent and avoids silently changing preset behavior if the library default lexicon changes later.
- **Alternatives considered:**
  - Use `defaults: true` everywhere and rely mostly on library defaults — rejected because curated preset tuning would become less explicit and more brittle over time.

## Architectural Notes
This commit does not change runtime semantics. It changes the contract that future humans and agents see first:
- the handoff document now reflects the active check surface and canonical config shape
- sample fixtures still exercise the loader, but now demonstrate the target schema rather than the legacy one
- presets now act as real examples of canonical config composition

The compatibility tests and many FS adapter fixtures still intentionally cover the legacy schema. That is correct. Those fixtures exist to prove backward compatibility, not to model the preferred authoring path.

## Information Sources
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — canonical config migration context
- `.worklogs/2026-03-22-181937-heuristic-defaults-migration.md` — heuristic default migration context
- `AGENTS.md` — stale handoff doc being updated
- `apps/prosesmasher/presets/*.json` — shipped product presets
- `apps/prosesmasher/crates/adapters/inbound/cli/tests/fixtures/test-config.json` — CLI-facing sample fixture
- `apps/prosesmasher/crates/adapters/outbound/fs/tests/fixtures/sample-config.json` — loader-facing sample fixture
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- `AGENTS.md` still contains some stale totals and historical framing that may need another pass once the migration fully settles.
- The presets now use canonical shape, but there is still no first-class CLI preset selection flow.
- If the library default prohibited lexicon changes significantly later, we should revisit whether curated presets should keep fully explicit lexical lists or partially inherit defaults.

## Key Files for Context
- `AGENTS.md` — current handoff and public product framing
- `apps/prosesmasher/presets/general-en.json` — baseline canonical preset example
- `apps/prosesmasher/presets/blog-strict-en.json` — stricter canonical preset example
- `apps/prosesmasher/crates/adapters/inbound/cli/tests/fixtures/test-config.json` — canonical CLI-facing sample config
- `apps/prosesmasher/crates/adapters/outbound/fs/tests/fixtures/sample-config.json` — canonical FS sample config
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — prior canonical config migration
- `.worklogs/2026-03-22-181937-heuristic-defaults-migration.md` — prior heuristic defaults migration

## Next Steps / Continuation Plan
1. Decide whether to remove the now-inactive legacy lexical modules (`banned_words`, `banned_phrases`, `forbidden_terms`, `gendered_terms`, `race_terms`) or keep them as dormant compatibility code for one more cycle.
2. Audit the CLI and tests for any remaining old ID references beyond intentionally backward-compatibility-specific fixtures.
3. If the current state is satisfactory, add a follow-up worklog and commit for the remaining dead-code cleanup rather than mixing it into the documentation migration.
