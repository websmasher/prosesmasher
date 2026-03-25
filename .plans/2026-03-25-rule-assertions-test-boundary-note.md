# Rule Assertions and Test Boundary Note

## Purpose

This is not an implementation checklist.

It is a structural note for a later agent evaluating how `prosesmasher` check crates should organize:
- rule files
- reusable semantic assertions
- adversarial test harnesses
- access boundaries between production code and tests

The goal is to preserve strong family-crate boundaries without recreating the test-topology confusion that happens when rule semantics, input generation, and orchestration are all proved in the same place.

## Core Distinction

There are three different things that must not be conflated:

1. **Rule semantics**
   What the rule should flag or ignore.

2. **Input construction**
   How test input is produced:
   - typed synthetic documents
   - parser-backed fixtures
   - real directory crawls

3. **Harness/orchestration context**
   Whether the rule is being exercised:
   - directly
   - through a family harness
   - through app orchestration
   - through CLI/package integration

The current shape in many Rust codebases becomes garbage when these three concerns all end up encoded as "rule tests."

## Recommended Per-Rule Shape

The default shape for a rule should be:

```text
rule.rs
rule_assertions.rs
rule_tests.rs
```

If the test matrix becomes large:

```text
rule.rs
rule_assertions.rs
rule_tests/
  mod.rs
  synthetic.rs
  thresholds.rs
  fixtures.rs
  near_misses.rs
```

## Meaning of Each File

### `rule.rs`

Owns:
- the rule implementation
- private rule helpers
- the exported check type if needed by the family `lib.rs`

Does not own:
- large adversarial input matrices
- reusable semantic assertions

### `rule_assertions.rs`

Owns:
- reusable semantic assertions about the rule
- the actual contract of what the rule should do

Examples:
- should match canonical positive cases
- should stay clean for known near-misses
- should escalate correctly at thresholds

This file should **not** know where input came from.

It should not care whether the input was built by:
- a unit-style typed builder
- a parser-backed fixture
- a real repo crawler

It only knows:
- "given a way to run this rule, these invariants must hold"

### `rule_tests.rs` or `rule_tests/`

Owns:
- actual test harnesses
- synthetic/adversarial input generation
- invoking the reusable assertions with different input providers

This is where large scenario matrices belong.

## Why Assertions Should Stay a Single Sidecar File by Default

The reusable semantic contract is usually small.

It tends to contain:
- positive assertions
- negative assertions
- threshold assertions

That is rarely large enough to justify a directory split by default.

What grows large is:
- adversarial input construction
- fixture-backed scenarios
- threshold case matrices
- real-directory coverage

So the default should be:
- `rule_assertions.rs` as one sidecar file
- `rule_tests/` becomes a directory only when needed

Do not pre-fragment the assertions layer unless the rule is unusually complex.

## Important Rule: Tests Should Reuse Assertions, Not Other Tests

The reusable thing should be assertion helpers, not `#[test]` functions.

Correct:
- multiple harnesses call shared assertion functions

Wrong:
- one test function calls another test function

Why:
- failure attribution stays clear
- layering stays explicit
- filtering/running subsets stays sane
- test code does not become the architectural API

## Access Boundary Recommendation

This is the key structural decision.

### Production API should stay narrow

A family crate should publicly expose only what production code needs:
- concrete check types if required
- `all_checks()` or equivalent family registry

It should **not** expose:
- rule-private helpers
- assertion helpers
- test harness utilities

### Rule assertions should be crate-private test support

`rule_assertions.rs` should be available to tests inside the family crate, but not become public API.

Recommended visibility:
- `#[cfg(test)] pub(crate)` module
- or private module visible to sibling test modules inside the crate

The important constraint is:
- no fake public testing API just to make integration tests compile

### Most family-level integration should remain internal to the crate

If a test needs access to:
- crate-private assertions
- family-local helper functions
- internal matcher support

then it should remain an internal crate test under `src/`, not an external Cargo `tests/*.rs` harness.

This is a deliberate choice.

Do not use external integration tests by default if doing so would force:
- widening the public API
- inventing fake exported helpers
- coupling through public test-only surfaces

### Use external `tests/*.rs` only for real public-surface testing

External integration tests are appropriate for:
- CLI adapters
- packaged binaries
- parser adapter black-box tests
- catalog/public registry behavior if intentionally tested through public APIs

They are not the default place for family rule semantics.

## Practical Family-Crate Test Shape

For a family crate, the intended shape is:

```text
src/
  lib.rs
  mod_support.rs              # optional family-local runtime helpers
  test_support.rs             # optional family-local test glue, cfg(test)
  some_rule.rs
  some_rule_assertions.rs
  some_rule_tests.rs
  another_rule.rs
  another_rule_assertions.rs
  another_rule_tests/
    mod.rs
    synthetic.rs
    fixtures.rs
    thresholds.rs
```

And optionally, only where it is truly public-surface testing:

```text
tests/
  public_surface.rs
```

## Relationship to Shared Workspace Test Support

This note assumes the workspace may also have a dev-only shared crate such as:

```text
crates/app/checks/test-support
```

That crate should own:
- typed `Document` builders
- generic assertion helpers for common check result shapes
- reusable snippets for synthetic documents

But it should **not** own:
- rule-specific semantic contracts

Rule-specific semantic expectations belong beside the rule in `rule_assertions.rs`.

So the split is:
- shared builders in workspace test-support
- rule semantics beside the rule

## Why This Boundary Is Useful

This structure lets multiple layers exercise the same semantic contract:

- local synthetic tests
- parser-backed fixture tests
- family harness tests
- real-repo scenario tests

without:
- duplicating semantic assertions
- widening public APIs
- hiding orchestration under rule-sidecar tests

It also keeps the most important line sharp:
- the rule contract stays beside the rule
- the scenario explosion stays in test harnesses

## Recommended Policy for a Later Agent

When introducing or refactoring a rule:

1. Put the implementation in `rule.rs`
2. Put reusable semantic assertions in `rule_assertions.rs`
3. Start with `rule_tests.rs`
4. Split to `rule_tests/` only if the scenario matrix gets large
5. Keep assertions crate-private
6. Prefer internal crate tests when the same assertions need crate-private access
7. Use external `tests/*.rs` only when intentionally validating public surfaces

## Non-Goals

This note does **not** decide:
- exact crate names
- exact migration order
- which existing rules should be moved first

Those belong in the main restructure plan.

This note exists only to preserve the intended **test boundary model** so a later implementation does not blur semantics, inputs, and orchestration again.
