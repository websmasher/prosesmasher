# Split Heuristics Into Owned Families

**Date:** 2026-03-26 10:54
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/crates/app/checks/{catalog,style-signals,cadence-patterns,rhetorical-framing,persona-signals,llm-slop}`, `apps/prosesmasher/crates/app/core`

## Summary
Split the old `app/checks/heuristics` family into four concrete ownership-based families plus an `llm-slop` scaffold. Moved all existing heuristic rules, assertions, and sidecars into the new families, updated catalog/core/workspace wiring, deleted the old broad heuristics crate, and kept the user-facing `heuristics` group as a catalog umbrella.

## Context & Problem
The previous `heuristics` family had grown into a mixed 15-rule crate that combined punctuation/style markers, cadence patterns, rhetorical framing, and persona/posture signals. That was exactly the kind of ownership blur we had been trying to avoid with the GuardRails-style family structure. We had already planned the split in `.plans/2026-03-26-heuristics-family-split-plan.md`; the next step was to make the codebase match it rather than keep treating `heuristics` as one giant bucket.

The user explicitly preferred splitting earlier rather than waiting for even larger families, on the grounds that several coherent small folders are easier to reason about than one oversized family with vague support ownership. That meant the implementation needed to do the real structural move, not a facade-only rename.

## Decisions Made

### Replace the broad `heuristics` crate with four concrete owned families
- **Chose:** Create:
  - `style-signals`
  - `cadence-patterns`
  - `rhetorical-framing`
  - `persona-signals`
- **Why:** These are the real ownership boundaries already present in the code. Style and cadence rules were almost entirely self-contained; rhetorical and persona rules had shared helpers that could be split cleanly.
- **Alternatives considered:**
  - Keep the old `heuristics` crate and just add subfolders inside it — rejected because it would preserve the broad family boundary and fake the split.
  - Split later when more rules land — rejected because the current 15-rule family was already large enough to hide structure.

### Add `llm-slop` as a real workspace family now
- **Chose:** Create `llm-slop/runtime` and `llm-slop/assertions` as an empty scaffold in the workspace.
- **Why:** The user explicitly said `llm-slop` is active scope now, not a future placeholder. Adding the crate boundary now gives the new slop rules a real home instead of encouraging them to land back in a generic heuristics bucket.
- **Alternatives considered:**
  - Omit `llm-slop` until the first rule lands — rejected because it would weaken the architectural commitment and risk backsliding into the wrong family.

### Keep the catalog’s `heuristics` group as an umbrella
- **Chose:** `catalog/runtime` now aggregates the new families under `Some("heuristics")` and in `all_quality_checks()`.
- **Why:** User-facing grouping and internal crate boundaries are separate concerns. The split improves ownership without forcing a CLI group-taxonomy churn in the same change.
- **Alternatives considered:**
  - Introduce new top-level user-facing groups immediately — rejected because that would conflate internal architecture and CLI surface.

### Split helper ownership instead of dragging the old `support.rs` forward wholesale
- **Chose:** Keep rhetorical helpers in `rhetorical-framing/runtime/src/support.rs` and create a separate `persona-signals/runtime/src/support.rs`.
- **Why:** The old helper file mixed section-position and sentence-phrase matching concerns. Persona rules only needed sentence-phrase collection and phrase resolution, so a family-local support file is clearer and keeps dead-code lints honest.
- **Alternatives considered:**
  - Reuse one shared helper file from a generic location — rejected because it would reintroduce the ownership blur the split was supposed to eliminate.
  - Leave all helpers in `rhetorical-framing` and make persona depend on it — rejected because it would create the wrong dependency direction between sibling families.

## Architectural Notes
- New runtime/assertions families now exist at:
  - `crates/app/checks/style-signals`
  - `crates/app/checks/cadence-patterns`
  - `crates/app/checks/rhetorical-framing`
  - `crates/app/checks/persona-signals`
  - `crates/app/checks/llm-slop`
- The old `crates/app/checks/heuristics` crate was removed from workspace membership and deleted from the source tree.
- `app/core` remains a compatibility facade, but its `quality::heuristics` surface now re-exports subfamilies as nested modules instead of a single broad runtime crate. This avoids symbol collisions from multiple `all_checks()` exports while preserving a compatibility layer.
- `catalog/runtime` now owns the umbrella aggregation logic:
  - `all_heuristics_checks()` combines `style-signals`, `cadence-patterns`, `rhetorical-framing`, `persona-signals`, and `llm-slop`
  - `check_kind()` still classifies those rules as `"heuristics"` for CLI/catalog purposes
- `llm-slop` is intentionally minimal today. Its runtime crate exports `all_checks() -> Vec::new()`, and the assertions crate is empty. That is acceptable as a structural scaffold, but real rule implementation needs to follow soon.

## Information Sources
- `.plans/2026-03-26-heuristics-family-split-plan.md` — exact intended split and rule mapping
- `apps/prosesmasher/crates/app/checks/heuristics/runtime/src/support.rs` before deletion — source helper ownership map
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib.rs` — catalog assembly and CLI group semantics
- `apps/prosesmasher/crates/app/core/src/lib.rs` — compatibility facade boundary
- Full workspace compile/test output from:
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace --no-run`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`

## Open Questions / Future Considerations
- `llm-slop` is only scaffolded, not populated. The next meaningful architectural step is to implement the first immediate and accumulative slop rules directly inside that family.
- `catalog/runtime` test names still reference older counts in their function names (`collect_all_returns_32`, `collect_quality_returns_24`, etc.) even though the assertions are correct. Cosmetic cleanup only.
- If the CLI or docs should eventually expose the new subfamilies explicitly instead of the umbrella `heuristics` group, that should be a separate product decision.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — workspace membership after removing the old heuristics crate and adding new families
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib.rs` — umbrella `heuristics` aggregation over the split families
- `apps/prosesmasher/crates/app/core/src/lib.rs` — compatibility facade over the split families
- `apps/prosesmasher/crates/app/checks/style-signals/runtime/src/lib.rs` — style family runtime boundary
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/lib.rs` — cadence family runtime boundary
- `apps/prosesmasher/crates/app/checks/rhetorical-framing/runtime/src/lib.rs` — rhetorical family runtime boundary
- `apps/prosesmasher/crates/app/checks/persona-signals/runtime/src/lib.rs` — persona family runtime boundary
- `apps/prosesmasher/crates/app/checks/rhetorical-framing/runtime/src/support.rs` — section-position helper ownership after the split
- `apps/prosesmasher/crates/app/checks/persona-signals/runtime/src/support.rs` — persona phrase-matching helper ownership after the split
- `.plans/2026-03-26-heuristics-family-split-plan.md` — the design this implementation follows
- `.worklogs/2026-03-26-100220-assertions-source-of-truth-for-rule-tests.md` — prior assertions/test architecture state that the new families preserve

## Next Steps / Continuation Plan
1. Implement the first real `llm-slop` rules directly in `crates/app/checks/llm-slop/{runtime,assertions}`:
   - `llm-disclaimer`
   - `response-wrapper`
   - `empty-transition`
2. Add the first accumulative `llm-slop` rules next:
   - `boilerplate-framing`
   - `generic-signposting`
   - `llm-vocabulary`
   - `softening-language`
   - `universalizing-claims`
3. Extend catalog/runtime tests once `llm-slop` has real rules so the umbrella `heuristics` count changes intentionally, not incidentally.
4. If desired, rename catalog test function names that still mention stale numeric labels even though their asserted counts are already correct.
