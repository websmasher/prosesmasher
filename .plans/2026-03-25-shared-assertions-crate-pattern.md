# Shared Assertions Crate Pattern

## Purpose

This is a general architectural note, not a project-specific implementation checklist.

It describes a reusable pattern for projects that need:
- one source of truth for behavioral assertions
- multiple test harnesses in different crates
- no duplication of those assertions
- no widening of production APIs just for tests

## The Problem

Many projects end up with duplicated assertions because the same behavior must be checked from:
- local synthetic tests
- parser-backed or crawler-backed tests
- integration or end-to-end harnesses

If assertions are duplicated, they drift.

If assertions stay crate-private, external harnesses cannot reuse them.

If private helpers are made public so external tests can reach them, the production API gets polluted.

## The Pattern

Split the world into:

1. **Runtime crate**
   Production code only.

2. **Assertions crate**
   Dev-only reusable assertions over the runtime crate's **public behavior**.

3. **Internal tests**
   Tests inside the runtime crate for private internals that are not shareable.

## Minimal Folder Shape

```text
crate_smth/
  Cargo.toml
  src/
    lib.rs
    code.rs
    code_internal_tests.rs

crate_smth_assertions/
  Cargo.toml
  src/
    lib.rs
    code_assertions.rs
```

Recommended dependency direction:

- `crate_smth_assertions` depends on `crate_smth`
- `crate_smth` has a `dev-dependency` on `crate_smth_assertions`
- other test harness crates can also `dev-depend` on `crate_smth_assertions`

This keeps the production dependency graph clean while allowing test reuse.

## What Goes Where

### Runtime crate

Owns:
- production implementation
- private helper functions
- local tests for private internals

Must not own:
- duplicated copies of reusable behavior assertions

### Assertions crate

Owns:
- reusable assertions about public behavior
- helper functions that check rule/output/result semantics

Must not own:
- private helper testing
- production implementation

The assertions crate should assert on things like:
- given input X, the runtime emits output Y
- given threshold case Z, escalation behaves like this

It should **not** require access to:
- private helper functions
- internal intermediate states
- hidden matcher internals

## Public vs Private Behavior

### Public behavior

What other crates can observe:
- inputs
- public entrypoints
- final outputs/results

This belongs in the assertions crate.

### Private behavior

What only the runtime crate can see:
- internal helper functions
- intermediate decomposition steps
- internal matcher subroutines

This stays in local internal tests inside the runtime crate.

## Why This Pattern Works

It gives you:
- one source of truth for shared assertions
- reuse across multiple test harnesses
- no duplicated semantic assertion logic
- no production API widening
- no runtime dependency on test code

## When This Pattern Fits

Use it when all of these are true:
- multiple crates or harnesses need the same assertions
- the shared assertions only need public behavior
- duplicated assertions would be a real maintenance risk

Examples:
- rule engines
- validators
- parsers
- analyzers
- compilers
- service handlers

## When This Pattern Does Not Fit

Do **not** use it when:
- the project is tiny
- there is no real cross-crate assertion reuse
- the assertions need private internals heavily

In those cases, local crate tests are simpler and better.

## Key Rule

If a test assertion needs private internals, it is **not** a shared assertion.

It belongs in the runtime crate as a local internal test.

If it can be stated entirely in terms of public behavior, it can live in the assertions crate and be reused.

## Practical Result

The reusable thing is:
- shared assertions over public behavior

The non-reusable thing is:
- private implementation tests

That distinction is what keeps the architecture clean.
