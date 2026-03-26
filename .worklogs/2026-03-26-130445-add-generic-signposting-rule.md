# Add Generic Signposting Rule

**Date:** 2026-03-26 13:04
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/assertions/src/config_loader.rs`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `fixtures/medicaloutline/*.expected.general-en.json`

## Summary
Added `generic-signposting` as the third `llm-slop` rule. It is an accumulative English-only check that flags repeated stock signposting phrases per document. I wired the rule through config, presets, catalog tests, assertions, and updated the Medical Outline regression sidecars additively.

## Context & Problem
After shipping `llm-disclaimer` and `response-wrapper`, the new Medical Outline corpus still showed a recurring family of canned rhetorical scaffolding: `it's important to note`, `that being said`, `as such`, and consultation signposts like `it's always best to consult`. These are not hard signatures like explicit model disclaimers, but they are still strong slop markers in repetition. The project already committed to two behavior modes for new slop checks: immediate and accumulative. This rule needed to be the first accumulative family in `llm-slop`, and its tests needed to follow the already-established runtime/assertions/sidecar split.

## Decisions Made

### Generic Signposting Is Accumulative, Not Immediate
- **Chose:** model `generic-signposting` as a count-based document check with `maxPerDocument`.
- **Why:** one signposting phrase can be ordinary prose; repeated stock scaffolding is the actual failure mode.
- **Alternatives considered:**
  - Immediate failure on any signposting phrase — rejected because it would overflag normal prose and make the rule noisy.
  - Weighted or distinct-term scoring — rejected as unnecessary complexity for the first version.

### Use Phrase Families, Not One-Off Literal Blacklists
- **Chose:** group matches into four deterministic subfamilies:
  - `important-to`
  - `transition`
  - `consultation-signpost`
  - `note-signpost`
- **Why:** the rule should catch stable rhetorical families while staying understandable and deterministic.
- **Alternatives considered:**
  - Exact-match list only — rejected because it would be too brittle and too narrow.
  - Broad fuzzy rhetoric matching — rejected because it would become hard to reason about and false-positive prone.

### Keep Consultation Language Narrow
- **Chose:** only match consultation phrasing when it uses explicit signposting forms like `it's always best to consult` or `it's recommended to consult`.
- **Why:** plain `consult a doctor` sentences can be legitimate human prose, especially in medical writing.
- **Alternatives considered:**
  - Matching any consultation recommendation — rejected because it would collapse real safety language into slop detection.

### Preserve Fixture Expectations Additively
- **Chose:** update only the five Medical Outline sidecars that now fail `generic-signposting`, while explicitly checking that no previously expected rule disappeared.
- **Why:** fixture sidecars are intended to be a regression floor, not a blind rewrite snapshot.
- **Alternatives considered:**
  - Rewriting all sidecars from fresh JSON output — rejected because it could silently erase already-established expectations.

## Architectural Notes
`generic-signposting` follows the current `llm-slop` family structure:
- runtime rule in its own file
- reusable public-output assertions in the sibling assertions crate
- sidecar synthetic harness in `slop_03_generic_signposting_tests/`

I extended `llm-slop/runtime/src/support.rs` instead of creating a second parallel traversal helper path. The support module now owns shared normalization, prefix stripping, quoted-span stripping, and sentence evidence assembly for the family. This keeps helper ownership local to `llm-slop` rather than leaking into app-wide support.

The config model needed a small new reusable type: `AccumulativeCheck { enabled, max_per_document }`. That keeps this rule’s shape simple and reusable for future slop families with the same threshold semantics.

## Information Sources
- Existing `llm-slop` rules and helpers:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/support.rs`
- Current Medical Outline fixtures:
  - `fixtures/medicaloutline/*.md`
- Existing fixture expectation work:
  - `.worklogs/2026-03-26-115433-add-fixture-expected-failures.md`
  - `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md`
  - `.worklogs/2026-03-26-124105-add-response-wrapper-rule.md`

## Open Questions / Future Considerations
- `generic-signposting` currently counts every matched sentence equally. If later corpora show too many weak hits, the boundary to tighten first is the transition family, especially bare `as such`.
- The next `llm-slop` rule should likely be another accumulative family such as `boilerplate-framing` or `llm-vocabulary`.
- Medical corpus sidecars now expose a clear recurring slop distribution. That corpus should become a black-box regression harness instead of relying only on checked-in expectation files.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — accumulative rule implementation and family matching.
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_03_generic_signposting.rs` — reusable assertions used by sidecar tests.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs` — adversarial synthetic cases for the rule.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/support.rs` — shared llm-slop normalization and evidence helpers.
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — `AccumulativeCheck` and `generic_signposting` config wiring.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON DTO mapping for the new rule config.
- `fixtures/medicaloutline/*.expected.general-en.json` — additive regression floor for real-world slop fixtures.
- `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md` — first immediate `llm-slop` rule and fixture rollout.
- `.worklogs/2026-03-26-124105-add-response-wrapper-rule.md` — second immediate `llm-slop` rule and additive sidecar update pattern.

## Next Steps / Continuation Plan
1. Add a black-box fixture regression harness that runs `general-en` over checked-in fixtures and verifies the sidecar `requiredFailures` contracts without exact-output snapshot brittleness.
2. Implement the next accumulative `llm-slop` family, likely `boilerplate-framing` or `llm-vocabulary`, using the same runtime/assertions/sidecar pattern and additive fixture updates.
3. Rerun the Medical Outline corpus after each new rule and only append new sidecar expectations after verifying no previously expected rules disappeared.
