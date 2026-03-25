# Add Check Crate Restructure Plan

**Date:** 2026-03-25 12:22
**Scope:** `.plans/2026-03-25-check-crate-restructure-plan.md`

## Summary
Added a concrete workspace-restructure plan for splitting `prosesmasher` checks into nested family crates under `crates/app/checks/`. The plan specifies the target tree, per-crate ownership, file moves, dependency arrows, test topology, and migration order.

## Context & Problem
The current `prosesmasher` app layer keeps all concrete checks, the check contract, the runner, and shared test helpers inside one `app/core` crate, while the CLI adapter still owns check-registry logic. That is workable at the current size, but it is the same direction that led to topology confusion in `guardrail3` once families, orchestration, and tests started growing.

The user wanted to stop before repeating that mistake and establish a proper hexagonal family-crate split now:
- one crate per family
- one file per rule
- actual nested directories under `app/checks`
- orchestration separated from concrete checks
- layered tests with parser behavior kept out of rule crates

The key architectural correction during the discussion was that the split should be by **family ownership boundary**, not one crate per rule, and the filesystem should show real nesting instead of flattening names with prefixes.

## Decisions Made

### Split checks into nested family crates under `crates/app/checks/`
- **Chose:** create nested crates for `core`, `catalog`, `lexical`, `heuristics`, `flow`, `readability`, `document-policy`, and `test-support`.
- **Why:** this preserves real ownership boundaries and keeps the filesystem structure honest. Family crates are large enough to justify their own compile boundary; individual rules are not.
- **Alternatives considered:**
  - Keep everything in `app/core` — rejected because it leaves orchestration, concrete checks, and shared test support coupled.
  - One crate per rule — rejected because it creates unnecessary crate explosion and turns shared helpers into a maintenance burden.

### Move registry logic out of the CLI adapter
- **Chose:** introduce a dedicated `catalog` crate that owns check listing, group mapping, and registry assembly.
- **Why:** registry/orchestration is app-layer logic, not adapter logic. Leaving it in the CLI adapter would preserve the current misplaced ownership.
- **Alternatives considered:**
  - Keep `src/checks.rs` in CLI — rejected because the adapter should compose app-layer services, not define them.

### Make shared test support explicit and non-runtime
- **Chose:** introduce `crates/app/checks/test-support` with `publish = false`.
- **Why:** current typed document builders in `app/core/src/test_helpers.rs` are useful across families, but they should not remain hidden inside one runtime crate or force public runtime APIs to widen for tests.
- **Alternatives considered:**
  - Leave helpers in a single family crate — rejected because all families would end up depending on one arbitrary concrete family for tests.
  - Duplicate builders across crates — rejected because it would rot quickly.

### Keep layered tests, but do not make tests call tests
- **Chose:** define rule-sidecar tests, family harness tests, orchestration tests, and adapter integration tests as separate layers that share helpers, not test functions.
- **Why:** this follows the real lesson from the `guardrail3` `hexarch` note: separate rule semantics, collector/orchestrator behavior, and full integration instead of hiding them under sidecar rule tests.
- **Alternatives considered:**
  - Wrap core tests inside higher-level tests — rejected because it makes failure attribution muddy and couples layers through test code instead of reusable helpers.

## Architectural Notes
The target shape keeps the current parser boundary intact:
- parser facts and markdown/HTML extraction stay in the parser adapter
- checks consume parsed `Document` values only

The plan also preserves the hex dependency direction:
- domain and ports stay unchanged
- app layer becomes more granular internally
- adapters depend on app crates, not the other way around

The resulting app structure is:
- `core` for contracts and runner
- `catalog` for family assembly and metadata
- family crates for actual checks
- a dev-only `test-support` crate for shared typed fixtures and assertions

## Information Sources
- Current `prosesmasher` workspace layout under `apps/prosesmasher/crates/`
- Current check registry ownership in `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs`
- Current check/runner ownership in `apps/prosesmasher/crates/app/core/src/check.rs` and `runner.rs`
- `guardrail3` family-crate layout under `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families`
- `guardrail3` architecture note: `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/33-hexarch-layered-test-architecture-note.md`

## Open Questions / Future Considerations
- Whether `app/core` should remain as a compatibility facade during migration or be retired once extraction is complete
- Exact package names and workspace member ordering for the new nested crates
- Whether any family-specific shared matcher helpers deserve their own small support crate later, or should remain family-local
- How much of the current sidecar test corpus should be promoted into family harnesses during the extraction

## Key Files for Context
- `.plans/2026-03-25-check-crate-restructure-plan.md` — full target tree, ownership map, and migration sequence
- `apps/prosesmasher/Cargo.toml` — current workspace membership and dependency baseline
- `apps/prosesmasher/crates/app/core/src/lib.rs` — current monolithic app-core surface
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs` — current misplaced registry logic
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/33-hexarch-layered-test-architecture-note.md` — the cautionary note this restructuring is meant to get ahead of

## Next Steps / Continuation Plan
1. Review and lock the target crate list and package names in the new restructure plan.
2. Add the new nested `crates/app/checks/*` workspace members with empty crates.
3. Extract `check.rs` and `runner.rs` into `checks/core` first and repair imports.
4. Extract family crates one by one, with `heuristics` last because it has the most churn and the most likely future slop-check growth.
5. Move CLI registry logic into `checks/catalog`.
6. Move `test_helpers.rs` into `checks/test-support` and update dev-dependencies.
