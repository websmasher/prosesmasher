# Split Checks Into Family Crates

**Date:** 2026-03-25 22:00
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/crates/app/checks/**`, `apps/prosesmasher/crates/app/core/**`, `apps/prosesmasher/crates/adapters/inbound/cli/**`, `apps/prosesmasher/crates/domain/types/**`, parser/fs adapter test files, packaged smoke test

## Summary
Restructured `prosesmasher` from a monolithic `app/core` crate into nested family crates under `crates/app/checks/`, with sibling runtime/assertions crates, a catalog runtime crate, and shared test support. Moved concrete checks and their tests into family-owned locations, converted flat `*_tests.rs` files into sidecar `*_tests/mod.rs` directories, extracted check registry logic out of the CLI adapter, and left `app/core` as a compatibility facade so the rest of the workspace did not need to flip all imports at once.

## Context & Problem
The project had already converged on a stricter test architecture discussion inspired by GuardRails. The risk was repeating the old `guardrail3` failure mode: one growing app crate owning rule implementations, orchestration, registry assembly, and test helpers, with tests scattered between flat sidecars and adapter-owned harnesses. The user wanted the codebase reorganized so that:

- checks are split by family under a real nested tree
- tests follow the GuardRails-style shape instead of the old flat `_tests.rs` scatter
- registry/orchestration logic lives in the app layer, not in the CLI adapter
- reusable rule-output assertions exist beside families without polluting production surfaces

There was also a second constraint: this needed to remain satisfiable with the current code, not just produce a beautiful plan. That meant the refactor had to preserve behavior, keep the full workspace passing, and avoid widening public APIs merely to make tests compile.

## Decisions Made

### Split the app layer into nested check-family crates
- **Chose:** add new workspace members under `apps/prosesmasher/crates/app/checks/`:
  - `core/runtime`
  - `core/assertions`
  - `catalog/runtime`
  - `catalog/assertions`
  - `test-support`
  - `lexical/{runtime,assertions}`
  - `heuristics/{runtime,assertions}`
  - `flow/{runtime,assertions}`
  - `readability/{runtime,assertions}`
  - `document-policy/{runtime,assertions}`
- **Why:** this is the actual family boundary the user wanted, and it aligns with the GuardRails family/test layout far better than one monolithic `app/core`.
- **Alternatives considered:**
  - Keep all checks in `app/core` and only move tests — rejected because the registry/orchestration and compile boundaries would still be wrong.
  - One crate per rule — rejected because that would explode the workspace and turn shared family support into overhead.

### Keep `app/core` as a compatibility facade
- **Chose:** rewrite `apps/prosesmasher/crates/app/core/src/lib.rs` into a thin re-export facade instead of deleting the crate immediately.
- **Why:** the repo still has consumers and tests that refer to `prosesmasher_app_core`. A facade let the family split land without forcing a second mass migration of every downstream import in the same commit.
- **Alternatives considered:**
  - Remove `app/core` entirely in this pass — rejected because it would unnecessarily widen the blast radius and make the refactor harder to validate.

### Move check catalog assembly out of the CLI adapter
- **Chose:** move the check collection/list/filter logic from `adapters/inbound/cli/src/checks.rs` into `app/checks/catalog/runtime`, and leave the CLI file as a thin compatibility re-export.
- **Why:** check metadata and group assembly are app-layer concerns. Keeping that logic in the adapter would preserve the same misplaced ownership the restructure was meant to correct.
- **Alternatives considered:**
  - Leave catalog assembly in the CLI adapter and only move concrete checks — rejected because it leaves orchestration split across the wrong boundary.

### Adopt sibling assertions crates for family rule-output assertions
- **Chose:** create per-family `assertions` crates that depend on the corresponding runtime crate and shared `test-support`.
- **Why:** this matches the design discussion that reusable rule-output assertions should be specific, single-source, and shareable across harnesses without living in production APIs. Runtime crates see them only through `dev-dependencies`.
- **Alternatives considered:**
  - Keep all assertions crate-private inside runtime crates — rejected because that blocks reuse from outside harnesses.
  - Duplicate assertions in unit and integration tests — rejected because it recreates the semantic-drift problem this architecture is meant to kill.

### Convert rule tests to sidecar directories and delete dead duplicate layers
- **Chose:** move rule tests into `src/<rule>_tests/mod.rs` directories and remove the unused duplicate `synthetic.rs` files that were left behind during migration.
- **Why:** the GuardRails shape calls for owned sidecar folders, and keeping an unused second layer beside them would immediately rot and confuse the intended structure.
- **Alternatives considered:**
  - Keep flat `*_tests.rs` files — rejected because it does not match the new family/test architecture.
  - Keep both `mod.rs` and `synthetic.rs` layers temporarily — rejected because only `mod.rs` was compiled, so the extra layer was dead code from day one.

### Decouple core runner tests from concrete rule families
- **Chose:** rewrite `core/runtime` runner tests to use local stub checks instead of importing real family checks.
- **Why:** the runner should test runner semantics, not concrete rule implementations. This also eliminated trait-instance churn from pulling family runtime crates into core tests.
- **Alternatives considered:**
  - Keep using real family checks in core runner tests — rejected because it made the boundary muddier and created unnecessary dev-dependency coupling.

## Architectural Notes
The resulting app-layer shape is now:

- `checks/core/runtime` owns `Check`, `BoxedCheck`, `run_checks`
- `checks/catalog/runtime` owns check listing/filtering/registry assembly
- family runtime crates own concrete rule implementations plus family-local support code
- family assertions crates own reusable rule-output assertions
- `checks/test-support` owns shared typed document builders and result helpers
- `app/core` remains only as a compatibility facade

The directory shape now matches the architectural intent instead of flattening everything into one crate:

- runtime behavior is separated by family
- rule-output assertions are explicit and not mixed with production code
- sidecar rule tests sit under their owning rule/family rather than in a global dump

Two intentional compromises were kept:

1. `app/core` still exists as a facade.
   This is temporary technical debt to reduce migration blast radius.

2. Assertions crates currently focus on public rule-output semantics.
   Private helper behavior stays local to runtime crates if needed, rather than forcing internals outward.

## Information Sources
- `.plans/2026-03-25-check-crate-restructure-plan.md`
- `.plans/2026-03-25-rule-assertions-test-boundary-note.md`
- `.plans/2026-03-25-shared-assertions-crate-pattern.md`
- `.plans/2026-03-25-prosesmasher-shared-assertions-plan.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- Existing app-layer files:
  - `apps/prosesmasher/crates/app/core/src/check.rs`
  - `apps/prosesmasher/crates/app/core/src/runner.rs`
  - `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs`
- Workspace verification:
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`

## Open Questions / Future Considerations
- Whether `app/core` should be removed entirely after downstream imports are migrated to the new family crates.
- Whether every family needs deeper assertion coverage immediately, or whether some assertions crates remain thin until more cross-harness reuse appears.
- Whether the current family support modules (`lexical/runtime/src/support.rs`, `heuristics/runtime/src/support.rs`) should later be split further as new slop checks land.
- Whether the GuardRails-style architecture should next be pushed further into stricter ownership of parser-backed rule harnesses.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — workspace membership for the new nested family crates
- `apps/prosesmasher/crates/app/checks/core/runtime/src/lib.rs` — new check contract and runner surface
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib.rs` — catalog assembly moved out of the CLI adapter
- `apps/prosesmasher/crates/app/checks/test-support/src/lib.rs` — shared test-only support entrypoint
- `apps/prosesmasher/crates/app/checks/test-support/src/builders.rs` — shared typed `Document` builders
- `apps/prosesmasher/crates/app/checks/test-support/src/result_helpers.rs` — shared result/assertion helpers used by assertions crates
- `apps/prosesmasher/crates/app/checks/heuristics/runtime/src/lib.rs` — representative family runtime crate with moved rule modules and test sidecars
- `apps/prosesmasher/crates/app/checks/heuristics/runtime/src/support.rs` — representative family-level support ownership after the split
- `apps/prosesmasher/crates/app/checks/heuristics/assertions/src/lib.rs` — representative assertions-crate pattern
- `apps/prosesmasher/crates/app/checks/lexical/runtime/src/lib.rs` — representative lexical family runtime crate
- `apps/prosesmasher/crates/app/checks/document-policy/runtime/src/lib.rs` — representative document-policy family runtime crate
- `apps/prosesmasher/crates/app/core/src/lib.rs` — compatibility facade preserving the old app-core surface
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs` — thin adapter facade after catalog extraction
- `.plans/2026-03-25-check-crate-restructure-plan.md` — the original target shape for this migration
- `.plans/2026-03-25-prosesmasher-shared-assertions-plan.md` — the test/assertions pattern that informed the new sibling assertions crates

## Next Steps / Continuation Plan
1. Migrate remaining downstream imports away from `prosesmasher_app_core` and onto the family runtime crates directly, then remove the compatibility facade when nothing depends on it.
2. Review each assertions crate and expand the current thin per-rule modules into richer shared rule-output assertion bodies where cross-harness reuse is genuinely needed.
3. Audit family support modules (`support.rs`) to ensure they are either clearly family-generic or pulled down into rule-owned subtrees; do not let them become a new blurry dumping ground.
4. When the new LLM slop checks are implemented, add them directly into the `heuristics` family runtime/assertions structure instead of reviving flat files or adapter-owned registries.
