# Add Boilerplate Framing Rule

**Date:** 2026-03-26 13:21
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/assertions/src/config_loader.rs`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `fixtures/why_do_we_dream.expected.general-en.json`

## Summary
Added `boilerplate-framing` as the fourth `llm-slop` rule. It is an accumulative English-only check for repeated canned setup, preview, and vague list-preface framing. I wired it through config, presets, catalog counts, rule assertions, synthetic tests, and updated the one real fixture it newly catches.

## Context & Problem
After `llm-disclaimer`, `response-wrapper`, and `generic-signposting`, the next remaining slop family was not explicit assistant leakage but canned framing language that stages the answer rather than saying anything. The design requirement from the user was important here: do not make this a raw phrase blacklist. The matcher needed to look for universal rhetorical patterns and small heuristic families rather than a pile of exact matches. The Medical Outline corpus did not justify a loose broad matcher, so the rule needed to stay narrow and conservative while still catching real article scaffolding like `In the following sections, we will explore...`.

## Decisions Made

### Keep Boilerplate Framing Accumulative
- **Chose:** model `boilerplate-framing` as another `AccumulativeCheck` with `maxPerDocument`.
- **Why:** one staging sentence can be ordinary prose; repeated staging is the slop signal.
- **Alternatives considered:**
  - Immediate failure on any framing sentence — rejected because it would punish normal transitions too aggressively.
  - Weighted severity by phrase — rejected because the project deliberately moved away from phrase-level scoring.

### Use Small Template Families Instead of Literal Phrase Lists
- **Chose:** implement three heuristic families:
  - `preview-frame`
  - `topic-frame`
  - `enumeration-preface`
- **Why:** these reflect rhetorical function and let the rule generalize beyond one exact seed phrase.
- **Alternatives considered:**
  - Exact-match phrases like `when it comes to` only — rejected because that would be too brittle and too obviously blacklist-driven.
  - Broad fuzzy rhetorical matching — rejected because it would be hard to reason about and likely noisy.

### Make Enumeration Prefaces Token-Order Based
- **Chose:** detect list-preface boilerplate by ordered token groups:
  - vague intro
  - category noun
  - enumeration verb
- **Why:** this catches shapes like `Some examples ... include:` and `Some common foods ... include:` without hardcoding each whole sentence.
- **Alternatives considered:**
  - Requiring a colon — rejected because sentence segmentation and prose style vary.
  - Matching exact starter phrases only — rejected because the user explicitly asked for broader heuristics.

### Keep the First Version Conservative
- **Chose:** only record the one real fixture hit in `why_do_we_dream`, and leave the Medical Outline files untouched because they do not cross the default threshold under this narrower framing rule.
- **Why:** the rule should reflect what it truly catches now, not what we hope it will catch later.
- **Alternatives considered:**
  - Expanding the matcher until more Medical Outline articles failed — rejected because that would likely blur into `generic-signposting` or `response-wrapper`.

## Architectural Notes
`boilerplate-framing` follows the current `llm-slop` shape:
- one runtime file
- one shared assertions file
- one sidecar synthetic test folder

I did not add more shared family support beyond what already exists because this rule’s token-order logic is specific to the rule itself. That keeps ownership local and avoids turning `llm-slop/support.rs` into a vague helper dumping ground. The config reused the existing `AccumulativeCheck` shape from `generic-signposting`, which is the right reuse boundary for this family.

## Information Sources
- Existing `llm-slop` rules:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
- Real fixture corpus:
  - `fixtures/medicaloutline/*.md`
  - `fixtures/why_do_we_dream.md`
- Prior related worklogs:
  - `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md`
  - `.worklogs/2026-03-26-124105-add-response-wrapper-rule.md`
  - `.worklogs/2026-03-26-130445-add-generic-signposting-rule.md`

## Open Questions / Future Considerations
- `boilerplate-framing` currently hits only `why_do_we_dream` in the real fixture corpus. That is acceptable for a narrow first version, but future work may add more framing families if more corpora show a stable pattern.
- The next best `llm-slop` candidate is probably `llm-vocabulary` or `softening-language`; both would complement the current rhetorical families without overlapping them too much.
- The catalog test function names still mention old counts (`collect_all_returns_32`, etc.) even though the assertions have been updated. That is harmless but sloppy and worth renaming in a cleanup pass.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs` — rule implementation and token-order matcher families.
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_04_boilerplate_framing.rs` — reusable public-output assertions.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing_tests/synthetic.rs` — adversarial synthetic rule coverage.
- `fixtures/why_do_we_dream.expected.general-en.json` — first real fixture sidecar updated for this rule.
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — `boilerplate_framing` config wiring.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON DTO mapping for `boilerplateFraming`.
- `.worklogs/2026-03-26-130445-add-generic-signposting-rule.md` — previous accumulative `llm-slop` rule and additive fixture update pattern.

## Next Steps / Continuation Plan
1. Add the next `llm-slop` family with the same runtime/assertions/sidecar pattern, likely `llm-vocabulary` or `softening-language`.
2. When updating fixture sidecars for future rules, continue the current additive process:
   - rerun corpus
   - inspect new failures
   - verify no old expected rules disappeared
   - then append sidecar entries
3. Add a black-box regression harness for the checked-in fixture sidecars so these expectations are executable, not just documented.
