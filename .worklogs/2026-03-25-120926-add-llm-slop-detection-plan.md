# Add LLM Slop Detection Plan

**Date:** 2026-03-25 12:09
**Scope:** `.plans/2026-03-25-llm-slop-detection-plan.md`

## Summary
Added a dedicated planning document for the next wave of LLM-style slop detection. The plan captures the behavior model, new check families, anti-overfitting rules, integration constraints with the existing checker set, and the intended implementation and test sequence.

## Context & Problem
The current prose checker already has a substantial set of heuristic checks, but the next set of user-reported problems is a broader family of LLM-style boilerplate, canned assistant wrappers, and overused diction. The key risk was drifting into a bad design:

- a dumb phrase blacklist built directly from chat examples
- per-phrase severity assignments that would become arbitrary and unmaintainable
- a second parallel scoring system that would not fit the current architecture

The user pushed for a design that stays simple, accounts for the existing check set, and generalizes from examples into broader phrase/rhetorical families instead of blindly encoding literals.

## Decisions Made

### Checks should only have two behavioral modes
- **Chose:** define new slop checks as either `immediate` or `accumulative`.
- **Why:** this is the only behavioral split that actually matters for the implementation. Adding extra "kinds" such as separate range/structural/scored classes would overcomplicate the design and not help the check architecture.
- **Alternatives considered:**
  - More granular behavior taxonomies — rejected because they describe output semantics, not useful implementation structure.
  - A weighted global score system — rejected because it is harder to reason about and was explicitly not wanted.

### Accumulation must be per check, never cross-check
- **Chose:** warnings accumulate only within the check that produced them.
- **Why:** one weak signal in several unrelated families should not mechanically escalate into an error. The system should preserve interpretability and let users inspect each family independently.
- **Alternatives considered:**
  - Cross-category warning aggregation — rejected because it would overstate weak evidence and make results harder to trust.

### Phrase examples are seed data, not the final matcher design
- **Chose:** require each user-provided example to be mapped to a broader family before implementation.
- **Why:** this avoids overfitting to literal strings and keeps the checks extensible. The design needs to capture "why this is bad" rather than just "this exact string appeared in chat."
- **Alternatives considered:**
  - Literal phrase-only blacklists — rejected because they are brittle and easy to game.

### Do not assign severity phrase-by-phrase
- **Chose:** split unlike signals into separate checks rather than hand-labeling each phrase with its own severity.
- **Why:** per-phrase severity quickly becomes arbitrary and editorially unstable. If one family is "bad on sight" and another is only bad in repetition, they should not be in the same check.
- **Alternatives considered:**
  - One mixed category with phrase-level severity — rejected because it would rot into case-by-case patchwork.

## Architectural Notes
The plan intentionally preserves the current architecture:

- existing checks remain intact
- new slop detection is added as new checks, not a new engine
- each new check fits the same parser → check runner → output flow

The key addition is a new family of LLM-era style checks with simple behavior:
- immediate checks for hard assistant leakage or canned wrappers
- accumulative checks for weak but repetitive diction and framing

The plan also explicitly calls out overlap review with existing heuristics such as `llm-openers`, `false-question`, `fragment-stacking`, `negation-reframe`, and `hedge-stacking` before implementation.

## Information Sources
- Chat discussion on 2026-03-25 about slop phrase families, thresholds, and architecture
- Existing planning docs under `.plans/`
- Existing heuristic module layout under `apps/prosesmasher/crates/app/core/src/quality/heuristics/`
- Existing output/result model in `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`

## Open Questions / Future Considerations
- Whether quoted/meta-discussion contexts should be suppressed in v1
- Whether current `llm-openers` should be folded into a broader `boilerplate-framing` check or remain separate
- Final public config field names for the new checks
- Threshold tuning after running on real documents

## Key Files for Context
- `.plans/2026-03-25-llm-slop-detection-plan.md` — the full design note for the new slop-detection work
- `.plans/check_structure.md` — broader check architecture and existing planning context
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/mod.rs` — current heuristic registry and helper structure
- `apps/prosesmasher/crates/domain/types/src/config.rs` — current check config model that new settings will need to fit into
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — current result output model

## Next Steps / Continuation Plan
1. Review the new planning doc and lock the first batch of checks to implement.
2. Audit overlap between the proposed new families and existing heuristics in `quality/heuristics/`.
3. Decide which new checks are `immediate` and which are `accumulative`, and define the exact config fields for each.
4. Implement matcher helpers and the first immediate checks before moving on to the accumulative families.
