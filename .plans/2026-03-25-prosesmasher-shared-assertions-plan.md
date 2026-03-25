# Prosesmasher Shared Assertions Plan

## Purpose

This is the `prosesmasher`-specific adaptation of the shared assertions crate pattern.

It explains how to apply the pattern to future check family crates so that:
- synthetic rule tests
- parser-backed real-input tests
- other integration harnesses

can all reuse the same rule-output assertions without duplicating semantics.

## Goal

For each check family, separate:

1. production runtime code
2. reusable rule-output assertions
3. local internal tests for private helpers

## Recommended Family Shape

For a family like heuristics:

```text
crates/app/checks/
  heuristics/
    runtime/
      Cargo.toml
      src/
        lib.rs
        rule1.rs
        rule2.rs
        rule1_internal_tests.rs
        rule2_internal_tests.rs

    assertions/
      Cargo.toml
      src/
        lib.rs
        rule1_assertions.rs
        rule2_assertions.rs
```

This pattern can be repeated for:
- `lexical`
- `flow`
- `readability`
- `document-policy`

only when that family actually needs cross-crate assertion reuse.

## Dependency Direction

For a family:

- `heuristics/assertions` depends on:
  - `heuristics/runtime`
  - `checks/core`
  - `domain/types`

- `heuristics/runtime` has a **dev-dependency** on:
  - `heuristics/assertions`

- parser or other integration test crates can also have **dev-dependencies** on:
  - `heuristics/assertions`
  - `heuristics/runtime`

This keeps production code from depending on test code, while still allowing runtime local tests to reuse the same assertions.

## What Goes Into `runtime`

### Runtime crate owns

- concrete rule implementations
- family `all_checks()`
- family-local runtime helpers
- local internal tests for private helpers or internals

### Runtime crate does not own

- duplicated copies of shared behavior assertions

## What Goes Into `assertions`

### Assertions crate owns

- reusable assertions for rule **outputs**
- threshold and escalation checks over public behavior
- helpers that inspect:
  - final result level
  - matched phrases
  - emitted evidence

### Assertions crate must not own

- private matcher helper tests
- production logic
- parser-specific behavior

## How It Gets Used

### Synthetic/local rule tests

Inside `heuristics/runtime`:
- build synthetic `Document`s using typed builders
- call shared assertions from `heuristics/assertions`

### Parser-backed tests

Inside parser or higher integration test crates:
- parse real markdown/HTML into `Document`
- call the same shared assertions from `heuristics/assertions`

That gives one source of truth for:
- what `rule1` should output

across both synthetic and real-input test contexts.

## What Stays Local Only

Anything that needs private runtime internals stays inside the runtime crate.

Examples:
- private matcher edge cases
- normalization helper tests
- intermediate decomposition helper tests

These should remain in:
- `rule1_internal_tests.rs`

and should **not** be moved into the assertions crate.

## Suggested First Application in Prosesmasher

Apply this pattern first where cross-crate reuse is most likely:

1. `heuristics`
   Best candidate because:
   - many future slop checks
   - future parser-backed phrase-family tests
   - likely need for shared rule-output assertions

Potential later adoption:
2. `lexical`
3. `document-policy`

Likely lower priority:
4. `readability`
5. `flow`

These may not need dedicated assertions crates immediately if their cross-crate reuse stays small.

## Relationship to Shared Test Support

This plan assumes a separate shared test-support crate may also exist:

```text
crates/app/checks/test-support/
```

That crate should own:
- typed `Document` builders
- generic result assertion helpers

It should **not** own:
- rule-specific output assertions

Rule-specific output assertions belong in the family's `assertions` crate.

## Why This Is Better Than the Alternatives

### Better than duplicating assertions

Because duplicated rule semantics drift.

### Better than making assertions crate-private only

Because parser-backed and higher-level tests can reuse them.

### Better than widening production APIs

Because the assertions crate uses only public behavior, not private internals.

### Better than source-sharing via `#[path]`

Because crate imports stay clean and maintainable.

## First Migration Rule

When extracting a family:

1. Move rule implementation into the family runtime crate
2. Move reusable rule-output assertions into the family assertions crate
3. Keep private-helper tests in the runtime crate
4. Make synthetic and parser-backed tests both call the same assertions crate

That preserves one source of truth for rule semantics without corrupting the runtime API.
