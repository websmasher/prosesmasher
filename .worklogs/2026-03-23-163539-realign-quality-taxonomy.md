# Realign Quality Taxonomy

**Date:** 2026-03-23 16:35
**Scope:** `AGENTS.md`, `apps/prosesmasher/crates/domain/types/src/config.rs`, `apps/prosesmasher/crates/domain/types/src/lib_tests.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/tests/fixtures/sample-config.json`, `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/tests/fixtures/test-config.json`, `apps/prosesmasher/crates/app/core/src/quality/flow/*`, `apps/prosesmasher/crates/app/core/src/quality/readability/*`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/packages/prosesmasher/README.md`

## Summary
Moved flow and readability out of `quality.heuristics` in the canonical config model and into their own top-level quality families: `quality.flow` and `quality.readability`. Updated the domain types, config loader, active checks, shipped config examples, and docs so the public schema now matches the public check taxonomy.

## Context & Problem
The public CLI and docs had been updated to expose five real families:

- `lexical`
- `heuristics`
- `flow`
- `readability`
- `document-policy`

But the canonical config still nested `wordRepetition`, `paragraphLength`, and all readability settings under `quality.heuristics`. That made the product internally inconsistent:

- CLI grouping said one thing
- config schema said another
- presets and full-config examples reinforced the wrong structure

The user called this out correctly. The taxonomy needed to be realigned, not just explained away.

## Decisions Made

### Make the canonical config mirror the active public check families
- **Chose:** Add `quality.flow` and `quality.readability` as first-class sections in the domain model and JSON DTOs.
- **Why:** The config should describe the same families users see in the CLI and output.
- **Alternatives considered:**
  - Keep the old nesting and only improve docs — rejected because the mismatch was real, not just confusing wording.
  - Flatten all quality settings into one object — rejected because the existing family split is useful and already exposed publicly.

### Keep `quality.heuristics` focused on rhetorical/style heuristics only
- **Chose:** Leave `quality.heuristics` responsible for the anti-slop and style-pattern checks, while moving paragraph/repetition/readability out.
- **Why:** This makes each section semantically coherent:
  - `heuristics` for rhetorical/style detectors
  - `flow` for paragraph/repetition controls
  - `readability` for formula-driven thresholds
- **Alternatives considered:**
  - Move everything under `quality.readability` or `quality.flow` — rejected because that would blur different kinds of validation.

### Treat this as a canonical schema change, not backward-compat plumbing
- **Chose:** Update the canonical DTO/examples/docs directly instead of adding alias support for the old nesting.
- **Why:** The library is already canonical-only; the right move is to keep the schema honest rather than reintroduce compatibility shims.
- **Alternatives considered:**
  - Accept both shapes — rejected because the user explicitly wanted the product corrected, not padded with transitional ambiguity.

## Architectural Notes
The runtime check modules were already physically separated into `quality/flow` and `quality/readability`. This pass makes the config model match that structure:

- domain types now expose:
  - `quality.lexical`
  - `quality.heuristics`
  - `quality.flow`
  - `quality.readability`
- FS config loading maps JSON into those same buckets
- full config examples and shipped preset assets now show the same shape
- CLI help and package README now describe the same model users actually configure

This removes the remaining taxonomy drift between:
- check grouping
- config schema
- public documentation

## Information Sources
- `apps/prosesmasher/crates/domain/types/src/config.rs`
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs`
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`
- `apps/prosesmasher/presets/full-config-en.json`
- `apps/prosesmasher/packages/prosesmasher/README.md`
- `.worklogs/2026-03-23-161101-tighten-cli-contract-and-docs.md`

## Open Questions / Future Considerations
- `quality.heuristics` still contains both boolean pattern detectors and threshold-style heuristics like `exclamationDensity` and `hedgeStacking`. That is acceptable, but if the product ever wants an even stricter taxonomy, those may deserve a narrower sub-family.
- Existing published versions before this change still document or embed the older shape. A follow-up release should publish this corrected schema to crates.io.

## Key Files for Context
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical config model
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — JSON DTO mapping and validation
- `apps/prosesmasher/crates/app/core/src/quality/flow/paragraph_length.rs` — flow checks now read `quality.flow`
- `apps/prosesmasher/crates/app/core/src/quality/readability/flesch_kincaid.rs` — readability checks now read `quality.readability`
- `apps/prosesmasher/presets/full-config-en.json` — public full-config example with the new taxonomy
- `apps/prosesmasher/packages/prosesmasher/README.md` — public package docs reflecting the new shape
- `.worklogs/2026-03-23-161101-tighten-cli-contract-and-docs.md` — prior CLI/docs pass that exposed the taxonomy mismatch

## Next Steps / Continuation Plan
1. Publish a follow-up release so crates.io `prosesmasher` ships the corrected `quality.flow` / `quality.readability` schema.
2. Add one explicit loader test that rejects the old nested `quality.heuristics.readability` / `paragraphLength` / `wordRepetition` shape with a clear unknown-field error.
3. Consider whether `exclamationDensity` and `hedgeStacking` should remain in `heuristics` or move under a narrower future family if the taxonomy is refined further.
