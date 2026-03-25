# Complete RS-TEST Runtime/Assertions Split

**Date:** 2026-03-25 22:33
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/crates/domain/types/**`, `apps/prosesmasher/crates/adapters/outbound/fs/**`, `apps/prosesmasher/crates/adapters/outbound/parser/**`, `apps/prosesmasher/crates/adapters/inbound/cli/**`, `apps/prosesmasher/packages/prosesmasher/**`

## Summary
Completed the repo-wide RS-TEST migration so the remaining tested crates now follow the runtime/assertions split, sidecar `*_tests/mod.rs` layout, and black-box external test boundary used by the new `app/checks` crates. The old published crates were kept as thin facades, test assets moved under the runtime crates that actually own them, and the packaged wrapper got its own sibling assertions crate so even the top-level smoke test package fits the same shape.

## Context & Problem
Earlier work had already split `app/checks` into family crates with sibling assertions crates, but the rest of the workspace was still inconsistent. `domain/types`, `adapters/outbound/fs`, `adapters/outbound/parser`, and `adapters/inbound/cli` still had flat `*_tests.rs` files or partially migrated assertions crates, and the wrapper package still had an external smoke test with no sibling assertions crate at all.

The user explicitly wanted the project brought all the way up to the GuardRails RS-TEST README shape, not just made to compile. That meant finishing the structural split across the whole workspace and then verifying the result with a full workspace test run, not stopping once the first compile errors were resolved.

## Decisions Made

### Split Remaining Tested Crates Into Runtime/Assertions Pairs
- **Chose:** keep the old published crate paths as compatibility facades while moving real implementation and tests into sibling `runtime/` crates, with sibling `assertions/` crates for reusable semantic assertions.
- **Why:** this preserves the existing public package graph and install story while matching the enforced GuardRails test architecture.
- **Alternatives considered:**
  - Leave the old crates as-is and only patch tests in place — rejected because it would keep the repo in a half-migrated state and violate the structure the user wants to enforce elsewhere.
  - Collapse everything back into fewer crates — rejected because it would undo the crate-boundary work the user explicitly wants to keep.

### Keep Published Facades Thin And Move Assets To The Owning Runtime Crates
- **Chose:** move parser fixtures and fs presets/fixtures into the runtime crates that now own the implementation, and update facade crates to simple reexports.
- **Why:** once tests and implementation live in runtime crates, assets should follow ownership. Keeping them in the facade crates would leave the split cosmetic.
- **Alternatives considered:**
  - Duplicate assets in both facade and runtime crates — rejected because it creates drift and packaging ambiguity.
  - Keep assets only in the old facade crates — rejected because runtime tests and packaging checks would keep reaching across the ownership boundary.

### Treat The Wrapper Package As Another Tested Component
- **Chose:** add `packages/prosesmasher/assertions` as a dev-only sibling assertions crate and move the smoke-test helpers there.
- **Why:** the wrapper package has an external test harness, so under the same RS-TEST rules it should not be a special exception.
- **Alternatives considered:**
  - Leave `packages/prosesmasher/tests/packaged_cli_smoke.rs` with inline helper functions — rejected because it would keep one package outside the architecture being imposed on the rest of the repo.
  - Move the smoke test into another crate — rejected because the smoke test is specifically about the wrapper package’s packaged/binary behavior.

### Fix Compile Breaks By Respecting The New Boundaries Instead Of Punching Through Them
- **Chose:** repair parser test includes, move fs/domain flat tests into sidecar folders, add explicit assertion modules, and update smoke tests to check the runtime fs package now that preset assets live there.
- **Why:** these were the minimal changes that preserved the new boundaries while getting the workspace back to a green state.
- **Alternatives considered:**
  - Reintroduce flat test files to get green quickly — rejected because it would directly undo the migration.
  - Expose more runtime internals just to satisfy tests — rejected because it would weaken the whole point of the architecture.

## Architectural Notes
- `crates/domain/types`, `crates/adapters/outbound/fs`, `crates/adapters/outbound/parser`, and `crates/adapters/inbound/cli` now follow the same broad pattern as `crates/app/checks/**`:
  - facade crate at the old path
  - sibling `runtime/` crate with production code and sidecar `*_tests/`
  - sibling `assertions/` crate with reusable semantic assertions
- `packages/prosesmasher` now also has a sibling `assertions/` crate because it owns an external black-box smoke test.
- The smoke test for shipped fs assets now targets `prosesmasher-adapters-outbound-fs-runtime` rather than the old facade crate, because the preset JSON files moved with the implementation ownership.
- `app/core` remains the already-created compatibility facade from the prior split; this work finished the rest of the workspace around it instead of changing that decision again.

## Information Sources
- GuardRails RS-TEST family doc:
  - `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- Prior local planning/architecture notes:
  - `.plans/2026-03-25-check-crate-restructure-plan.md`
  - `.plans/2026-03-25-rule-assertions-test-boundary-note.md`
  - `.plans/2026-03-25-shared-assertions-crate-pattern.md`
  - `.plans/2026-03-25-prosesmasher-shared-assertions-plan.md`
- Prior worklogs that set up the first half of the split:
  - `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md`
  - `.worklogs/2026-03-25-220947-normalize-sidecar-synthetic-layout.md`
- Verification commands run locally:
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace --no-run`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - `rg -n '_tests\\.rs|#\\[path = \".*_tests\\.rs\"\\]' apps/prosesmasher/crates apps/prosesmasher/packages`

## Open Questions / Future Considerations
- `domain/types` still uses a broad `lib_tests` sidecar rather than one sidecar per exported module. This is structurally compliant with the current deterministic checks, but if GuardRails later enforces per-module assertion coverage more strictly, that crate will likely need another pass.
- `fs/assertions/src/support.rs` is private support code inside an assertions crate. That is fine under the current rules, but if the rule set later forbids any non-module files in assertions crates, that support layer would need to move into a shared test-support crate.
- The final external subagent audit did not return before shutdown, so the final confidence comes from local deterministic scans plus the green workspace test run rather than from a final subagent confirmation.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — final workspace membership including the new runtime/assertions crates and the packaged assertions crate.
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/lib.rs` — final CLI runtime ownership and test boundary.
- `apps/prosesmasher/crates/adapters/inbound/cli/assertions/src/args.rs` — example of reusable adapter-level semantic assertions.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_loader.rs` — fs runtime with sidecar test pathing.
- `apps/prosesmasher/crates/adapters/outbound/fs/assertions/src/config_loader.rs` — fs assertion-module split.
- `apps/prosesmasher/crates/adapters/outbound/parser/runtime/src/markdown.rs` — parser runtime using sidecar folder modules.
- `apps/prosesmasher/crates/adapters/outbound/parser/assertions/src/markdown.rs` — parser assertion module split.
- `apps/prosesmasher/crates/domain/types/runtime/src/lib.rs` — domain runtime crate after facade split.
- `apps/prosesmasher/crates/domain/types/assertions/src/lib_root.rs` — domain assertions helper module.
- `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs` — wrapper package black-box smoke test after moving helper logic out.
- `apps/prosesmasher/packages/prosesmasher/assertions/src/packaged_cli_smoke.rs` — sibling assertions crate for the wrapper package.
- `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md` — first half of the split and compatibility-facade pattern.
- `.worklogs/2026-03-25-220947-normalize-sidecar-synthetic-layout.md` — sidecar normalization pass that this work finished across the remaining crates.

## Next Steps / Continuation Plan
1. If GuardRails tightens the deterministic rules further, start with `crates/domain/types/runtime/src/lib_tests/synthetic.rs` and decide whether to split it into per-module sidecars (`config_tests`, `document_tests`, `error_tests`, `locale_tests`, `metadata_tests`) with matching assertion modules.
2. If assertion helper duplication starts creeping in, evaluate promoting truly generic pieces from crate-local assertion support into `crates/app/checks/test-support` or a sibling repo-wide test-support crate, but keep module-specific semantics inside the owning assertions crate.
3. Re-run the external GuardRails family against this repo once available and compare any reported structural violations against the local scans used here, especially around `assertions/src/lib.rs` for lib-root-tested crates.
