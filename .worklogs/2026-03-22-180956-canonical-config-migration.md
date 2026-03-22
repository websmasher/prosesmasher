# Canonical config migration for prose quality

**Date:** 2026-03-22 18:10
**Scope:** `apps/prosesmasher/crates/domain/types/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`, `apps/prosesmasher/crates/app/core/src/terms/*`, `apps/prosesmasher/crates/app/core/src/structure/*`, `apps/prosesmasher/crates/app/core/src/readability/*`, `apps/prosesmasher/crates/app/core/src/runner_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/*`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`

## Summary
Restructured `prosesmasher` config around canonical `quality` and `documentPolicy` sections while preserving legacy `terms` / `thresholds` input compatibility. Moved the active lexical, readability, and document-policy checks to read the canonical semantic model directly, and collapsed the public prohibited-term surface to one active `prohibited-terms` check.

## Context & Problem
The project had a working config format, but the semantics were wrong for the intended product boundary. The old schema mixed implementation categories and project-specific buckets:
- `bannedWords`, `bannedPhrases`, `forbiddenTerms`, `genderedTerms`, `raceTerms`
- heuristic phrase lists that implied users should enumerate library-owned AI-slop patterns
- document-shape rules mixed into generic `thresholds`

The design discussion with the user settled on a better boundary:
- `quality.lexical` for user-supplied lexical policy
- `quality.heuristics` for library-owned prose-quality detectors with overridable thresholds/toggles
- `documentPolicy` for opt-in markdown shape rules

The migration needed to preserve backward compatibility long enough to keep the tool usable while shifting the runtime toward the new semantic model. The initial compatibility layer already existed in the loader, but many checks still read synthesized legacy fields. That left the new schema as a wrapper rather than the real source of truth.

## Decisions Made

### Normalize both config schemas into one canonical domain model
- **Chose:** Keep `CheckConfig` with canonical `quality` and `document_policy` fields, but retain synthesized `terms` and `thresholds` compatibility views during the migration.
- **Why:** This lets the loader accept both schemas immediately while allowing the check layer to move incrementally onto the semantic model without a full flag day.
- **Alternatives considered:**
  - Replace the old config outright and break compatibility — rejected because too many checks, fixtures, and tests still assumed the old shape.
  - Keep only the loader normalization and leave checks on legacy fields indefinitely — rejected because that would freeze the wrong abstraction in the runtime.

### Collapse public lexical prohibitions into one active `prohibited-terms` check
- **Chose:** Add a canonical `ProhibitedTermsCheck` and remove the old lexical bucket checks from the active registry.
- **Why:** The semantic distinction is matcher behavior, not editorial category. The separate public checks were leaking project-specific policy into library semantics.
- **Alternatives considered:**
  - Keep all five public lexical checks and just map them internally — rejected because it would preserve the wrong API surface.
  - Merge the config but keep the old check IDs as the active interface — rejected because it would blur the migration and keep the product taxonomy inconsistent.

### Move readability and prose-flow checks onto canonical quality defaults
- **Chose:** Make `paragraph-length`, `word-repetition`, and all readability checks read `quality.heuristics.*` directly.
- **Why:** These checks are core prose quality and are supposed to run from built-in defaults rather than requiring explicit threshold config.
- **Alternatives considered:**
  - Keep them under legacy thresholds until the whole migration is done — rejected because it would leave the most important quality defaults dormant.
  - Move only the schema and not the checks — rejected because it would keep tests and runtime behavior misleading.

### Keep document policy opt-in
- **Chose:** Make `word-count`, `heading-counts`, `bold-density`, `heading-hierarchy`, `sentence-case`, and `code-fences` run only from `documentPolicy`.
- **Why:** These are artifact-shape rules, not universal prose-quality defaults.
- **Alternatives considered:**
  - Give them built-in defaults as part of core quality — rejected because that would force document-shape assumptions onto all prose validation.

## Architectural Notes
The domain model now reflects the intended product layering:
- `quality.lexical`
- `quality.heuristics`
- `document_policy`

The FS adapter is the compatibility boundary. It accepts both the legacy and target JSON schemas, applies locale-aware defaults, merges overrides, and still synthesizes old `terms` / `thresholds` for the not-yet-migrated checks.

The runtime migration is intentionally incremental:
- active lexical prohibition is canonical now
- required/recommended/simplicity are canonical now
- paragraph length, word repetition, readability, and document-policy checks are canonical now
- some heuristic pattern checks still depend on legacy compatibility fields and remain a follow-up slice

This preserves forward movement without introducing a risky all-at-once refactor across every check family.

## Information Sources
- `AGENTS.md` — current project architecture, config shape, and check inventory
- `.worklogs/2026-03-22-151320-rewrite-json-evidence.md` — recent output/evidence context and the earlier `low-expectations` rationale
- `.worklogs/2026-03-22-151813-config-presets.md` — recent config-oriented work and evidence that preset work jumped ahead of schema clarification
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical domain shape and defaults
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — normalization layer for legacy and target schemas
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — active term registry and override resolution helpers
- `apps/prosesmasher/crates/app/core/src/runner_tests.rs` — end-to-end expectations around active checks
- Verification runs:
  - `cargo test -q -p prosesmasher-app-core`
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- The heuristic-pattern family still needs migration off legacy list fields:
  - `llm-openers`
  - `affirmation-closers`
  - `summative-closer`
  - `false-question`
  - `humble-bragger`
  - `jargon-faker`
  - `negation-reframe`
  - `exclamation-density`
  - `hedge-stacking`
- The handoff doc still describes the old term taxonomy and old config format. It needs to be updated after the heuristic migration settles.
- The compatibility fields on `CheckConfig` should disappear once all active checks read canonical config directly.

## Key Files for Context
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical config structs and built-in defaults
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — dual-schema loader and normalization logic
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — active term check registry and override resolvers
- `apps/prosesmasher/crates/app/core/src/terms/prohibited_terms.rs` — canonical prohibited-term check
- `apps/prosesmasher/crates/app/core/src/structure/word_repetition.rs` — representative quality heuristic now reading canonical config
- `apps/prosesmasher/crates/app/core/src/structure/word_count.rs` — representative document-policy check now reading canonical config
- `apps/prosesmasher/crates/app/core/src/readability/flesch_kincaid.rs` — representative readability check now reading canonical config
- `apps/prosesmasher/crates/app/core/src/runner_tests.rs` — integration expectations around the active check surface
- `.worklogs/2026-03-22-151320-rewrite-json-evidence.md` — prior worklog for the rewrite-loop output contract
- `.worklogs/2026-03-22-151813-config-presets.md` — prior worklog showing why config/preset semantics needed correction

## Next Steps / Continuation Plan
1. Migrate the remaining heuristic-pattern checks from legacy list fields to canonical `quality.heuristics` toggles and library-owned defaults. Read first:
   - `apps/prosesmasher/crates/app/core/src/patterns/llm_openers.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/affirmation_closers.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/summative_closer.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/false_question.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/humble_bragger.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/jargon_faker.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/negation_reframe.rs`
   - `apps/prosesmasher/crates/app/core/src/patterns/exclamation_density.rs`
2. After those checks move, remove dead reliance on synthesized `terms` / `thresholds` from the active runtime path and prune the unused lexical compatibility modules from docs and CLI examples.
3. Update `AGENTS.md` and any sample config/docs to show the canonical schema, the `prohibited-terms` public surface, and the distinction between always-on quality defaults and opt-in `documentPolicy`.
