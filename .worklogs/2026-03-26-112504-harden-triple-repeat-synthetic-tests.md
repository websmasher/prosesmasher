# Harden Triple Repeat Synthetic Tests

**Date:** 2026-03-26 11:25
**Scope:** `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_07_triple_repeat_tests/synthetic.rs`

## Summary
Expanded the `triple-repeat` synthetic suite with adversarial positives and negatives that route through the rule assertions module instead of ad hoc output checks. The new cases pin the rule to the real fixture failure shape and to block/paragraph boundaries that a sloppy implementation could mishandle.

## Context & Problem
The current synthetic coverage for `triple-repeat` only proved the obvious `"It's / It's / It's"` case plus two simple negatives. We had already confirmed that the real fixture failure in `fixtures/article1.mdx` was a harder `"Your four-year-old / Your six-year-old / Your three-year-old"` pattern. The gap was that synthetic tests were not stressing paragraph boundaries, blockquotes, code blocks, or later sliding windows, so fixture parity still depended too much on the real file.

## Decisions Made

### Add fixture-parity and boundary-shape tests in the sidecar synthetic harness
- **Chose:** Keep the existing smoke cases and add seven harder cases directly to the sidecar synthetic file for `heur_07_triple_repeat`.
- **Why:** The rule’s public behavior is simple enough that the synthetic file is the right place to build adversarial documents, while assertions continue to be centralized in the assertions module.
- **Alternatives considered:**
  - Add only the exact fixture-parity case — rejected because it would still miss paragraph, blockquote, and code-block boundary mistakes.
  - Move the new scenarios into parser-backed integration tests — rejected because the goal here was synthetic parity without needing fixture input.

### Keep assertions centralized and reuse them from the sidecar
- **Chose:** Route every new positive through `assert_triple_repeat_failure(...)` and every new negative through `assert_passes(...)` from the assertions crate.
- **Why:** This preserves the “one source of truth for rule public-output assertions” rule that the project has been converging on.
- **Alternatives considered:**
  - Assert directly on suite output inside the synthetic file — rejected because it would recreate the drift problem between sidecars and reusable assertions.

### Add local builders for block-structured documents
- **Chose:** Extend the synthetic file with small local helpers for paragraph blocks, arbitrary block lists, and metadata counting.
- **Why:** The new adversarial cases needed paragraph boundaries, blockquotes, and code blocks without dragging in parser fixtures or widening shared test support for one rule.
- **Alternatives considered:**
  - Reuse only the old single-paragraph helper — rejected because it could not express the negative paragraph-boundary case or the positive blockquote case.
  - Add these builders to shared test support — rejected because they are currently specific to this rule’s synthetic adversarial coverage.

## Architectural Notes
This change keeps the intended split intact:
- the sidecar synthetic harness owns scenario construction
- the assertions module owns reusable public-behavior checks
- the rule itself is unchanged

The new cases also make the rule boundary clearer: `triple-repeat` should scan paragraph sentences and recurse through blockquotes, but it must not bridge paragraph boundaries or inspect code-block content.

## Information Sources
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_07_triple_repeat.rs` — rule behavior and boundary handling
- `apps/prosesmasher/crates/app/checks/cadence-patterns/assertions/src/heur_07_triple_repeat.rs` — reusable assertion entrypoints
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_07_triple_repeat_tests/synthetic.rs` — existing synthetic coverage
- `apps/prosesmasher/crates/domain/types/runtime/src/document.rs` — `Block`, `Paragraph`, and `ListBlock` shapes
- `fixtures/article1.mdx` — real-world failure shape used to derive the fixture-parity synthetic case

## Open Questions / Future Considerations
- The synthetic file now has enough local builders that other cadence rules may eventually want a tiny family-local block builder helper, but it is not justified yet.
- The fixture-parity approach used here should likely be repeated for other rules that currently only have toy positive cases.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_07_triple_repeat.rs` — the rule under test
- `apps/prosesmasher/crates/app/checks/cadence-patterns/assertions/src/heur_07_triple_repeat.rs` — shared public-behavior assertions for the rule
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_07_triple_repeat_tests/synthetic.rs` — synthetic adversarial harness and local builders
- `.worklogs/2026-03-26-100220-assertions-source-of-truth-for-rule-tests.md` — prior assertion-centralization pass this builds on
- `.worklogs/2026-03-26-105416-split-heuristics-into-owned-families.md` — family-crate split that placed `triple-repeat` under cadence-patterns

## Next Steps / Continuation Plan
1. Run the same “fixture failure -> synthetic parity” pass for the next rules that still only have toy positives, starting with rhetorical-framing checks.
2. When `llm-slop` rules land, require at least one fixture-parity synthetic case and one boundary negative for each accumulative rule.
3. If multiple cadence rules start needing nested-block document builders, extract a family-local helper instead of copying the current local functions further.
