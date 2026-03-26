# Make Assertions the Single Source of Truth for Rule Tests

**Date:** 2026-03-26 10:02
**Scope:** `apps/prosesmasher/crates/app/checks/{document-policy,flow,heuristics,lexical,readability}/{assertions,runtime}`, `apps/prosesmasher/crates/app/checks/test-support/src/result_helpers.rs`

## Summary
Moved the remaining public-surface rule expectations out of sidecar synthetic tests and into the owning assertions modules. Sidecars now generate scenarios and call reusable assertion helpers instead of inspecting suite results directly or relying on thin macro-only wrappers.

## Context & Problem
The runtime/assertions split had been applied structurally, but many assertion modules were still mostly ceremonial. Several sidecar synthetic tests were still defining what the rule should prove by checking pass/fail counts or observed values themselves. That violated the intended GuardRails-style architecture: the assertions module must be the single reusable source of truth for public rule behavior, while sidecars only own input construction and harnessing.

This work builds directly on:
- `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md`
- `.worklogs/2026-03-25-220947-normalize-sidecar-synthetic-layout.md`
- `.worklogs/2026-03-25-223349-complete-rs-test-runtime-assertions-split.md`
- `.worklogs/2026-03-26-003511-five-round-test-attack-hardening.md`
- `.worklogs/2026-03-26-083431-id-rule-file-layout-cleanup.md`

## Decisions Made

### Make rule-output assertions live in assertions crates, not sidecars
- **Chose:** Move reusable public-behavior checks into the owning rule assertion modules across document-policy, flow, heuristics, lexical, and readability families.
- **Why:** This removes semantic drift. Synthetic sidecars and any future real-input harnesses can now reuse one source of truth for what each rule should emit.
- **Alternatives considered:**
  - Keep generic `assert_passes` / `assert_fails` wrappers only — rejected because it leaves rule-specific expectations implicit in sidecars.
  - Let sidecars inspect `SuiteValidationResult` directly — rejected because it duplicates rule semantics and recreates drift risk.

### Enrich thin assertion modules instead of inventing a second test layer
- **Chose:** Add rule-specific assertion helpers to the remaining thin modules: `bold-density`, `prohibited-terms`, `simplicity`, and `recommended-terms`.
- **Why:** These were the last files still acting as wrappers instead of owning reusable assertions.
- **Alternatives considered:**
  - Leave them macro-only because the rules are simple — rejected because simple rules still need reusable expected-behavior definitions.
  - Create family-level shared assertions for multiple rules at once — rejected because the semantics are rule-owned, not family-owned.

### Add small result helpers only where the reusable assertions needed them
- **Chose:** Add `assert_observed_strs` to shared test support.
- **Why:** Multiple lexical rules expose observed matched terms as arrays, and their assertions needed a common stable helper.
- **Alternatives considered:**
  - Rebuild array parsing ad hoc in each assertions file — rejected because it duplicates suite-result plumbing.
  - Leave lexical assertions at pass/fail only — rejected because it weakens the rule-specific semantic checks.

## Architectural Notes
- Sidecar `synthetic.rs` files now act as scenario providers and harnesses, not semantic owners.
- Assertion modules now define reusable public-surface expectations for all active rule families under `app/checks`.
- Internal helper tests still remain local where they exercise private implementation details, for example fragment classification internals. That line stays important: shared assertions are for public rule behavior, not private machinery.
- The runtime/assertions split is now semantically real in the rule families, not just structurally present on disk.

## Information Sources
- `apps/prosesmasher/crates/app/checks/*/runtime/src/*_tests/synthetic.rs` — current sidecar ownership and remaining ad hoc public assertions
- `apps/prosesmasher/crates/app/checks/*/assertions/src/*.rs` — existing assertion module patterns
- `apps/prosesmasher/crates/app/checks/test-support/src/result_helpers.rs` — shared suite-result helpers
- `apps/guardrail3/crates/app/rs/families/test/README.md` in the sibling repo — target test architecture being enforced
- Prior worklogs listed above — architectural backstory and migration sequence

## Open Questions / Future Considerations
- `domain/types/runtime/src/lib_tests` is still broader than the ideal per-module sidecar shape. That was intentionally left outside this rule-family assertions pass.
- Adapter/package assertion crates already exist, but not every one of them has the same “rule-specific helper richness” as the check families. If the GuardRails rules are extended from checks to all modules, they may need the same hardening.
- `core/runtime/src/runner_tests/synthetic.rs` still inspects suite internals directly because it is testing the runner/orchestrator itself, not a rule-facing public API. That distinction should remain explicit.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/test-support/src/result_helpers.rs` — shared suite-result assertion primitives
- `apps/prosesmasher/crates/app/checks/lexical/assertions/src/lex_01_prohibited_terms.rs` — example of upgrading a thin rule wrapper into real reusable assertions
- `apps/prosesmasher/crates/app/checks/lexical/assertions/src/lex_03_simplicity.rs` — example of reusable observed-term assertions for a lexical rule
- `apps/prosesmasher/crates/app/checks/lexical/assertions/src/lex_05_recommended_terms.rs` — example of reusable numeric threshold assertions
- `apps/prosesmasher/crates/app/checks/document-policy/assertions/src/doc_04_bold_density.rs` — example of upgrading a simple range-style rule assertion module
- `apps/prosesmasher/crates/app/checks/heuristics/assertions/src/heur_10_llm_openers.rs` — representative heuristic assertions module after the earlier pass
- `.worklogs/2026-03-25-223349-complete-rs-test-runtime-assertions-split.md` — original runtime/assertions migration context
- `.worklogs/2026-03-26-083431-id-rule-file-layout-cleanup.md` — latest structural cleanup before this semantic assertions pass

## Next Steps / Continuation Plan
1. Audit the non-check crates (`adapters`, `domain/types`, `packages/prosesmasher`) against the same semantic standard: any shared public-surface test expectation should live in their assertions crate, not in sidecars or black-box tests.
2. Tighten `domain/types` toward the per-module sidecar/assertions shape if the GuardRails family will enforce that structure there as strictly as it does in check families.
3. If future slop-rule families are added, require the assertion module to ship rule-specific helpers in the same change, not as a later cleanup.
