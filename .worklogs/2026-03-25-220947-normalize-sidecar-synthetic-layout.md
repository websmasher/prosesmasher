# Normalize Sidecar Synthetic Layout

**Date:** 2026-03-25 22:09
**Scope:** `apps/prosesmasher/crates/app/checks/**/_tests/*`, `apps/prosesmasher/crates/app/checks/test-support/**`, `apps/prosesmasher/crates/app/checks/heuristics/assertions/src/em_dashes.rs`, `apps/prosesmasher/crates/app/checks/document-policy/assertions/src/word_count.rs`, `apps/prosesmasher/Cargo.lock`

## Summary
Normalized the new sidecar test folders so they actually follow a `mod.rs` entrypoint plus `synthetic.rs` payload layout instead of keeping all test bodies in `mod.rs`. Added generic result/evidence helpers in `test-support` and moved the first real rule semantics out of sidecars into the assertions layer for `em-dashes` and `word-count`.

## Context & Problem
The large family-crate refactor landed the right crate boundaries, but the test topology was still half-migrated. The new `*_tests/` directories existed, yet most `mod.rs` files still directly owned the test bodies. At the same time, the sibling assertions crates were still too thin: most of them only wrapped pass/fail/metadata, while actual rule semantics remained embedded in the sidecars.

That left the project in a structurally inconsistent state:

- the filesystem shape looked closer to the GuardRails pattern
- the actual test ownership still looked like the old flat sidecar model
- assertions crates existed but were not yet earning their place

This follow-up pass was meant to fix the tree shape everywhere and prove the assertions pattern with concrete rule semantics instead of only macros.

## Decisions Made

### Make every sidecar folder use `mod.rs` as an entrypoint only
- **Chose:** move the bodies of all `crates/app/checks/**/src/*_tests/mod.rs` files into sibling `synthetic.rs` files, leaving `mod.rs` as the module entrypoint.
- **Why:** this aligns the test tree with the intended GuardRails-style shape and removes the ambiguity between “folder exists” and “folder actually owns layered scenarios.”
- **Alternatives considered:**
  - Leave test bodies in `mod.rs` — rejected because it defeats the point of introducing sidecar folders.
  - Split only a few families first — rejected because the layout change is mechanical and safer when done consistently.

### Preserve sidecar access with explicit parent re-exports only where needed
- **Chose:** use `pub(super) use super::*;` in most sidecar `mod.rs` entrypoints so existing `super::RuleCheck` references in `synthetic.rs` still resolve cleanly.
- **Why:** this preserves rule-owned access without flattening the modules back out or rewriting every test body in one shot.
- **Alternatives considered:**
  - Rewrite every synthetic file immediately to import rule types directly from `crate` — rejected as too much churn for a layout-normalization pass.

### Expand `test-support` into actual shared result/evidence helpers
- **Chose:** add generic helpers for:
  - retrieving a validation result by check id
  - retrieving evidence arrays
  - asserting observed values
  - asserting expectation kwargs
  - asserting evidence length and first evidence fields
- **Why:** assertions crates cannot absorb real semantics if the only shared support is pass/fail/skip wrappers.
- **Alternatives considered:**
  - Put all result spelunking directly into each assertions crate — rejected because that would duplicate low-level result access and recreate drift.

### Move the first real rule semantics into assertions
- **Chose:** promote concrete semantic assertions for:
  - `heuristics/assertions/src/em_dashes.rs`
  - `document-policy/assertions/src/word_count.rs`
- **Why:** the architecture needed one real demonstration of the intended split:
  - sidecars own scenarios
  - assertions own expected output semantics
- **Alternatives considered:**
  - Keep delaying semantics migration until every helper existed — rejected because the pattern would remain unproven.

## Architectural Notes
This pass does not complete the whole semantic migration. It does two narrower things:

1. It makes the sidecar layout consistent across the new family runtime crates.
2. It establishes a workable pattern for moving rule semantics into assertions without widening production APIs.

The current state is now:

- `mod.rs` is a real entrypoint layer
- `synthetic.rs` is the scenario layer
- assertions crates can now own richer output expectations when needed
- shared result spelunking lives in `test-support`, not in rule-specific modules

This is still intentionally incremental. The broader assertions migration can now continue rule by rule instead of all semantics remaining trapped in sidecars.

## Information Sources
- Previous refactor worklog: `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md`
- Current family runtime/assertions tree under `apps/prosesmasher/crates/app/checks/`
- `low-expectations` result model:
  - `/Users/tartakovsky/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/low-expectations-0.1.0/src/types.rs`
  - `/Users/tartakovsky/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/low-expectations-0.1.0/src/suite_scalar.rs`
- Full verification:
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`

## Open Questions / Future Considerations
- Many assertions crates are still thin wrappers; they should gradually absorb rule-output semantics from sidecars the same way `em-dashes` and `word-count` now do.
- `catalog/runtime/src/catalog_tests/` now has the same `synthetic.rs` layout as the rule sidecars, but it is still fundamentally a public-surface test candidate rather than a true rule sidecar. That can be revisited later.
- CLI adapter tests and parser adapter tests have not yet been normalized into the same layered pattern.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/test-support/src/result_helpers.rs` — new shared result/evidence helpers used by assertions modules
- `apps/prosesmasher/crates/app/checks/heuristics/assertions/src/em_dashes.rs` — first heuristic rule with real semantic assertions
- `apps/prosesmasher/crates/app/checks/document-policy/assertions/src/word_count.rs` — first document-policy rule with real semantic assertions
- `apps/prosesmasher/crates/app/checks/heuristics/runtime/src/em_dashes_tests/mod.rs` — normalized entrypoint-only sidecar module
- `apps/prosesmasher/crates/app/checks/heuristics/runtime/src/em_dashes_tests/synthetic.rs` — scenario layer now calling shared assertions
- `apps/prosesmasher/crates/app/checks/document-policy/runtime/src/word_count_tests/mod.rs` — normalized entrypoint-only sidecar module
- `apps/prosesmasher/crates/app/checks/document-policy/runtime/src/word_count_tests/synthetic.rs` — scenario layer now calling shared assertions
- `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md` — prior crate-split refactor this pass builds on

## Next Steps / Continuation Plan
1. Continue migrating rule-output semantics out of sidecars into the sibling assertions crates, prioritizing the checks with complex evidence payloads:
   - `false-question`
   - `negation-reframe`
   - `smart-quotes`
   - `heading-hierarchy`
   - `heading-counts`
2. Normalize adapter tests next:
   - split CLI `args_tests.rs` / `output_tests.rs` into sidecar folders
   - decide which CLI and parser tests are true black-box `tests/*.rs` cases
3. Revisit `catalog/runtime` and decide whether its current sidecar test folder should remain internal or move to a public-surface black-box harness.
