# Add Rule Assertions Test Boundary Note

**Date:** 2026-03-25 12:37
**Scope:** `.plans/2026-03-25-rule-assertions-test-boundary-note.md`

## Summary
Added a focused architecture note for how individual rule files, reusable semantic assertions, and test harnesses should be separated inside future family crates. The note formalizes the intended `rule.rs` / `rule_assertions.rs` / `rule_tests...` pattern and clarifies the access boundary between internal crate tests and true public-surface integration tests.

## Context & Problem
During the discussion about family-crate restructuring, a specific ambiguity emerged around test layering. The user correctly pointed out that much of the historical test garbage in other codebases comes from conflating:

- rule semantics
- input generation / collection
- orchestration or integration context

The initial wording "tests should not call tests" was directionally right but incomplete, because what actually needs to be shared is the semantic contract of the rule. The important distinction is:

- do not make `#[test]` functions wrap other `#[test]` functions
- do reuse shared semantic assertion helpers across multiple harnesses

That required a separate note, because it affects later family-crate structure and access boundaries but is orthogonal to the main crate-restructure plan.

## Decisions Made

### Rule semantics should live in sidecar assertion modules
- **Chose:** define a default per-rule shape of `rule.rs`, `rule_assertions.rs`, and either `rule_tests.rs` or `rule_tests/`.
- **Why:** this keeps the semantic contract physically next to the rule while preventing scenario explosion from bloating the rule file itself.
- **Alternatives considered:**
  - Put all semantics directly in sidecar tests — rejected because the same assertions then get duplicated across synthetic and integration-style harnesses.
  - Split assertions into many files by default — rejected because the assertion layer is usually small; the scenario matrix, not the semantic layer, is what grows.

### Assertions should be reusable, but crate-private
- **Chose:** treat `rule_assertions.rs` as crate-private test support rather than public API.
- **Why:** family crates should not widen their exported surface just to let tests reuse semantic contracts.
- **Alternatives considered:**
  - Public test helper APIs — rejected because they pollute production surfaces.
  - Pure external integration tests only — rejected because many useful family tests need access to crate-private assertion helpers and local matcher support.

### Internal crate tests should remain the default for family semantics
- **Chose:** prefer internal crate tests for family-level rule semantics and scenario matrices, and reserve external `tests/*.rs` for true public-surface black-box testing.
- **Why:** this lets tests share crate-private assertions and helpers without inventing fake public hooks.
- **Alternatives considered:**
  - Force all "integration" style tests into Cargo `tests/` harnesses — rejected because that would either duplicate semantics or widen APIs.

## Architectural Notes
This note refines the test topology from the broader restructure plan:

- shared workspace test-support may own typed document builders and generic helpers
- rule-specific semantic contracts stay beside the rule in `rule_assertions.rs`
- adversarial or large scenario matrices move into `rule_tests/`

The key conceptual split is:
- reusable **assertions**, yes
- reusable **test functions**, no

This keeps semantics central without turning tests into a tangled second API layer.

## Information Sources
- Chat discussion on 2026-03-25 about rule assertions, sidecar tests, and integration boundaries
- `.plans/2026-03-25-check-crate-restructure-plan.md`
- `guardrail3` `hexarch` test-architecture note at `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/33-hexarch-layered-test-architecture-note.md`

## Open Questions / Future Considerations
- Whether some especially large rules will eventually need assertion submodules rather than a single `rule_assertions.rs`
- Whether any family crates need a family-level `test_support.rs` in addition to the shared workspace `test-support` crate
- How much of the current sidecar test corpus can be migrated directly versus rewritten around assertion helpers

## Key Files for Context
- `.plans/2026-03-25-rule-assertions-test-boundary-note.md` — the dedicated note describing the per-rule assertion/test boundary model
- `.plans/2026-03-25-check-crate-restructure-plan.md` — the broader crate-structure plan this note complements
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/33-hexarch-layered-test-architecture-note.md` — external cautionary note about layered test concerns

## Next Steps / Continuation Plan
1. Hand this note to the next agent together with the main restructure plan.
2. When implementing family crates, make each migrated rule follow the default pattern:
   - `rule.rs`
   - `rule_assertions.rs`
   - `rule_tests.rs` or `rule_tests/`
3. Introduce a shared `test-support` crate only for typed builders and generic helpers, not rule-specific semantics.
4. Keep external Cargo integration tests limited to genuine public-surface validation.
