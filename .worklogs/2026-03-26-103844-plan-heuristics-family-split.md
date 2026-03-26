# Plan Heuristics Family Split

**Date:** 2026-03-26 10:38
**Scope:** `.plans/2026-03-26-heuristics-family-split-plan.md`

## Summary
Added a concrete architecture plan for splitting the current broad `heuristics` family into smaller ownership-based families and treating `llm-slop` as active implementation scope rather than a future placeholder.

## Context & Problem
The current `heuristics` family has 15 rules and already mixes several different semantic domains: punctuation/style markers, cadence patterns, rhetorical framing, persona/posture signals, and the incoming LLM-slop ruleset. That makes the folder heavier to scan and creates pressure toward shared helpers with blurry ownership.

The user explicitly preferred earlier splitting over waiting for a family to become “large enough,” with the reasoning that several smaller coherent families are easier to maintain than one large mixed family. The plan needed to reflect that and define the exact ownership boundaries rather than leave “split heuristics later” as a vague idea.

## Decisions Made

### Split `heuristics` by semantic ownership boundary
- **Chose:** Plan five families:
  - `style-signals`
  - `cadence-patterns`
  - `rhetorical-framing`
  - `persona-signals`
  - `llm-slop`
- **Why:** These groups match how support helpers and assertions should cluster. The split becomes meaningful only if helpers and assertions also move with the rules.
- **Alternatives considered:**
  - Keep one broad `heuristics` crate longer — rejected because the current size already hides structure and will get worse once slop rules land.
  - Split by vibes without helper ownership — rejected because that would just recreate a giant shared support blob elsewhere.

### Treat `llm-slop` as in-scope now
- **Chose:** Make `llm-slop` part of the planned active split, with explicit initial rules and behavior categories.
- **Why:** The user made clear that `llm-slop` is not a future placeholder. Planning needed to reserve the real family now so implementation can land directly into the right crate.
- **Alternatives considered:**
  - Keep `llm-slop` as a future bucket — rejected because it would push immediate implementation back into the wrong family.

### Keep user-facing grouping separate from crate boundaries
- **Chose:** Note that catalog/runtime can still expose a broader heuristics umbrella even after the crate split.
- **Why:** Crate ownership and CLI grouping are different concerns. The split should improve architecture without forcing a user-facing taxonomy change unless wanted.
- **Alternatives considered:**
  - Force catalog groups to exactly mirror crates — rejected because it couples internals to UX too tightly.

## Architectural Notes
- The plan now makes the split real by specifying:
  - exact new family names
  - exact current rule mapping
  - where helpers should move
  - what should stay out of these families
  - migration order
- The ownership rule is explicit: a helper belongs in a family only if at least two rules in that family need it and other families likely should not.
- The `llm-slop` family is modeled with both immediate and accumulative rules, matching the earlier design decision for slop-rule behavior.

## Information Sources
- `apps/prosesmasher/crates/app/checks/heuristics` — current mixed heuristics family
- `.plans/2026-03-25-check-crate-restructure-plan.md` — prior family-crate restructuring direction
- `.plans/2026-03-25-llm-slop-detection-plan.md` — prior slop-rule taxonomy and behavior direction
- Conversation decisions in this session about splitting earlier rather than later and making `llm-slop` active scope

## Open Questions / Future Considerations
- Whether `style-signals` should keep both `fake-timestamps` and `colon-dramatic`, or whether one of those eventually belongs in `rhetorical-framing`
- Whether the old `heuristics` compatibility facade should exist temporarily during migration or be removed immediately
- Final ID prefix naming for `llm-slop` rules (`slop_*` vs `llm_*`)

## Key Files for Context
- `.plans/2026-03-26-heuristics-family-split-plan.md` — the new concrete split plan
- `.plans/2026-03-25-check-crate-restructure-plan.md` — broader crate-structure plan this refines
- `.plans/2026-03-25-llm-slop-detection-plan.md` — slop-rule product direction and rule taxonomy
- `.worklogs/2026-03-26-100220-assertions-source-of-truth-for-rule-tests.md` — latest state of the assertions/test architecture the new families must preserve

## Next Steps / Continuation Plan
1. Implement Phase 1 of `.plans/2026-03-26-heuristics-family-split-plan.md`: create the four replacement families for current heuristics rules and move runtime/assertions/sidecars into them.
2. Split the current `heuristics/runtime/src/support.rs` by actual ownership instead of copying it wholesale into a new shared helper location.
3. Update `app/checks/catalog/runtime` to assemble checks from the new families while keeping the desired user-facing grouping stable.
4. Implement `llm-slop` as a new family directly after the split rather than landing new slop rules in any transitional heuristics crate.
