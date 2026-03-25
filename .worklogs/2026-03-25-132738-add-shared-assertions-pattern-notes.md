# Add Shared Assertions Pattern Notes

**Date:** 2026-03-25 13:27
**Scope:** `.plans/2026-03-25-shared-assertions-crate-pattern.md`, `.plans/2026-03-25-prosesmasher-shared-assertions-plan.md`

## Summary
Added two architectural notes describing the shared-assertions-crate pattern: one general note for any project, and one adaptation note for `prosesmasher` check families. The notes formalize how reusable rule-output assertions can be shared across local synthetic tests and parser-backed integration tests without duplicating semantic checks.

## Context & Problem
The earlier discussion around `rule_assertions.rs` exposed a real boundary issue:

- keeping assertions crate-private makes them unavailable to external parser/integration harnesses
- duplicating assertions in both synthetic and integration tests causes drift
- forcing production APIs open just for tests damages the architecture

The constraint set pushed toward a cleaner split:
- runtime crate for production code
- dev-only assertions crate for shared rule-output assertions
- local internal tests in the runtime crate for private helper behavior

This needed to be recorded explicitly, because it is a separate architectural pattern from the broader family-crate migration plan.

## Decisions Made

### Separate runtime and shared assertions into sibling crates
- **Chose:** describe a sibling `runtime/` and `assertions/` crate pattern for check families.
- **Why:** this keeps the production graph clean while allowing multiple test harnesses to reuse the same rule-output assertions.
- **Alternatives considered:**
  - Duplicate assertions in unit and integration tests — rejected because semantic drift is exactly the problem being avoided.
  - Source-sharing assertion files with `#[path]` across crates — rejected as too brittle and ugly as the default pattern.

### Keep private helper tests local to the runtime crate
- **Chose:** treat private-helper or matcher-internal checks as runtime-local internal tests, not part of the shared assertions surface.
- **Why:** shared assertions must be about public rule behavior only. Anything that needs private internals is not actually shareable across crates.
- **Alternatives considered:**
  - Put private-helper assertions into the shared assertions crate — rejected because that would require exposing internals or bending dependency flow.

### Write both a general pattern note and a `prosesmasher` adaptation note
- **Chose:** split the documentation into one general architectural pattern note and one `prosesmasher`-specific application note.
- **Why:** the pattern is reusable beyond this project, but the adaptation note is still needed so another agent can see how it should map onto `crates/app/checks/*`.
- **Alternatives considered:**
  - Fold this into the existing restructure plan — rejected because this boundary question is specific enough to deserve its own note.

## Architectural Notes
The pattern now looks like:

- runtime crate owns production code and local internal tests
- assertions crate owns reusable rule-output assertions over public behavior
- synthetic and parser-backed harnesses both consume the same assertions crate

For `prosesmasher`, the first intended application is the future `heuristics` family split, since that family is the most likely to need shared rule-output assertions across synthetic and parser-backed test contexts.

## Information Sources
- Chat discussion on 2026-03-25 about reusable `rule_assertions`, test drift, and crate dependency flow
- `.plans/2026-03-25-check-crate-restructure-plan.md`
- `.plans/2026-03-25-rule-assertions-test-boundary-note.md`

## Open Questions / Future Considerations
- Whether every family will need a sibling assertions crate, or whether this should be introduced only where cross-crate reuse is real
- Whether some very small families should stay with runtime-local tests only
- Whether a shared `test-support` crate should be introduced before or after the first assertions crate extraction

## Key Files for Context
- `.plans/2026-03-25-shared-assertions-crate-pattern.md` — general architectural pattern
- `.plans/2026-03-25-prosesmasher-shared-assertions-plan.md` — `prosesmasher`-specific application plan
- `.plans/2026-03-25-check-crate-restructure-plan.md` — broader family-crate migration context
- `.plans/2026-03-25-rule-assertions-test-boundary-note.md` — earlier note on rule assertions versus harness layering

## Next Steps / Continuation Plan
1. Hand both new notes to the next agent together with the broader restructure plan.
2. When the first family extraction begins, decide whether `heuristics` gets the first sibling assertions crate.
3. Keep private-helper tests inside the runtime crate and move only reusable rule-output assertions into the assertions crate.
4. Re-evaluate whether the pattern is worth applying to smaller families after the first extraction proves itself.
