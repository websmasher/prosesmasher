# Heuristic defaults migration for quality checks

**Date:** 2026-03-22 18:19
**Scope:** `apps/prosesmasher/crates/app/core/src/patterns/*`, `apps/prosesmasher/crates/app/core/src/terms/hedge_words.rs`, `apps/prosesmasher/crates/app/core/src/terms/mod.rs`

## Summary
Moved the remaining active heuristic quality checks away from legacy config-authored phrase/signal lists and onto canonical `quality.heuristics` behavior with built-in English defaults. Kept a temporary compatibility rule where explicitly supplied legacy lists still override the defaults, then updated the tests to reflect that quality heuristics run under default config.

## Context & Problem
After the canonical config migration, the major remaining inconsistency was that several high-value prose-quality checks still depended on the old `terms` and `thresholds` views:
- `llm-openers`
- `affirmation-closers`
- `summative-closer`
- `false-question`
- `humble-bragger`
- `jargon-faker`
- `negation-reframe`
- `exclamation-density`
- `hedge-stacking`

That contradicted the design decisions made with the user. These are not project-owned vocab lists; they are library-owned anti-slop heuristics. Requiring users to enumerate them would undermine the point of the library and preserve the wrong semantics in the runtime.

The migration needed to make the library own those pattern defaults while still not breaking old config files abruptly.

## Decisions Made

### Make heuristic phrase defaults library-owned
- **Chose:** Add built-in English pattern lists in `patterns/mod.rs` for opener/closer/rhetorical checks and resolve them automatically under default config.
- **Why:** These patterns are part of the library’s knowledge of common LLM prose failures. Users should not need to author them manually.
- **Alternatives considered:**
  - Keep them fully config-authored forever — rejected because it forces users to recreate library knowledge and keeps the wrong abstraction in the public model.
  - Move the defaults into the loader/domain config layer immediately — rejected for this slice because the check layer already needed a local compatibility bridge and the pattern families are still implementation-specific enough to keep close to the checks for now.

### Preserve legacy overrides during the transition
- **Chose:** If a legacy config explicitly provides a phrase/signal list, the check still uses that instead of the built-in defaults.
- **Why:** This preserves backward compatibility while the canonical schema continues settling and while not all docs/examples have been updated yet.
- **Alternatives considered:**
  - Ignore legacy lists and always use built-in defaults — rejected because it would silently change behavior for existing user configs.
  - Keep both active simultaneously and merge them — rejected because it makes behavior less predictable and could inflate false positives in old setups.

### Treat exclamation density and hedge stacking as default quality heuristics
- **Chose:** Read `quality.heuristics.exclamation_density` and `quality.heuristics.hedge_stacking` directly, with built-in defaults active by default.
- **Why:** These are quality signals, not document policy. They should behave like other default-on heuristics rather than disappearing unless a threshold is explicitly configured.
- **Alternatives considered:**
  - Leave them on legacy thresholds until later — rejected because that would keep two quality models alive at once.

### Introduce a built-in English hedge lexicon
- **Chose:** Add a canonical English default hedge list in `terms/mod.rs`, while still honoring explicit legacy `hedge_words` if provided.
- **Why:** `hedge-stacking` is useless as a default quality heuristic without a built-in lexicon.
- **Alternatives considered:**
  - Disable hedge stacking by default until a more advanced lexicon system exists — rejected because the user explicitly wanted core quality checks to run on defaults.
  - Put hedge terms in general lexical config — rejected because they are analysis inputs for a specific heuristic, not general lexical policy.

## Architectural Notes
This slice completes the semantic move for the active quality checks:
- lexical policy lives in the canonical quality model
- readability/flow checks already used canonical quality fields
- the remaining rhetoric/style heuristics now run from default library knowledge plus toggles

The pattern defaults are currently localized in `patterns/mod.rs` and the hedge defaults in `terms/mod.rs`. That is an acceptable intermediate state because:
- they are tightly coupled to the implementation of the checks
- locale behavior is simple right now: English defaults only, others empty
- compatibility fallback logic is easier to reason about close to the checks

Longer term, there may be a cleaner central “quality defaults” module that owns all heuristic defaults in one place. I did not do that in this slice because the bigger priority was to remove legacy-field dependence from active runtime behavior without a large unrelated refactor.

## Information Sources
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — previous migration slice and the current canonical model
- `AGENTS.md` — current public architecture/config description, including the now-stale old term taxonomy
- `apps/prosesmasher/crates/app/core/src/patterns/*_tests.rs` — existing phrase inventories and behavioral expectations
- `apps/prosesmasher/crates/app/core/src/terms/hedge_words_tests.rs` — existing hedge behavior and threshold expectations
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical heuristic toggles and defaults
- Verification runs:
  - `cargo test -q -p prosesmasher-app-core`
  - `cargo test -q`
  - `cargo clippy -q -p prosesmasher-app-core --all-targets`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- The defaults are English-only. That is correct for now, but multilingual heuristic defaults remain an open design task.
- The pattern defaults currently live near the checks, not in a centralized defaults layer.
- Some active checks still indirectly rely on synthesized legacy compatibility fields for override behavior. The next cleanup should decide whether to keep that override bridge or move legacy adaptation fully into the loader.
- `AGENTS.md` and other docs still describe the old config shape and old check taxonomy.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/patterns/mod.rs` — built-in heuristic pattern defaults and resolution helpers
- `apps/prosesmasher/crates/app/core/src/patterns/llm_openers.rs` — representative section-opener heuristic now driven by canonical defaults
- `apps/prosesmasher/crates/app/core/src/patterns/negation_reframe.rs` — representative sentence-pair heuristic now driven by canonical defaults
- `apps/prosesmasher/crates/app/core/src/patterns/exclamation_density.rs` — representative threshold-based heuristic now reading canonical config
- `apps/prosesmasher/crates/app/core/src/terms/hedge_words.rs` — hedge stacking now reading canonical config
- `apps/prosesmasher/crates/app/core/src/terms/mod.rs` — default hedge resolution
- `.worklogs/2026-03-22-180956-canonical-config-migration.md` — prior canonical config migration

## Next Steps / Continuation Plan
1. Remove remaining active runtime dependence on synthesized legacy `terms` / `thresholds` where possible and make the compatibility bridge purely an adapter concern.
2. Audit the CLI/docs/test fixtures for old IDs and old config shape. Update:
   - `AGENTS.md`
   - sample config fixtures
   - any CLI examples still mentioning `banned-words` or the old `terms` / `thresholds` model
3. Decide whether heuristic defaults should stay distributed by check family or move into a centralized defaults module once the migration dust settles.
