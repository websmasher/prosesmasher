# Add Response Wrapper Rule

**Date:** 2026-03-26 12:41
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/{runtime,assertions}`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `apps/prosesmasher/presets/full-config-en.json`, `fixtures/medicaloutline/*.expected.general-en.json`

## Summary
Implemented `response-wrapper` as the second immediate `llm-slop` rule. The rule catches canned first-person assistant capability/limitation wrappers like “I can provide general information”, “I do not provide medical advice”, and “the ability to provide a diagnosis”, while intentionally not firing on plain third-person prose or standalone “consult a professional” guidance.

## Context & Problem
After `llm-disclaimer` landed, the Medical Outline corpus still contained a second class of obvious assistant slop: service-wrapper sentences that are not just identity leakage, but fallback/refusal framing. The user explicitly asked that this rule be heuristic and family-based rather than another bag of exact matches. That meant the new rule needed to be more structured than `llm-disclaimer`, while still precise enough to remain an immediate check instead of a warning threshold rule.

## Decisions Made

### Model `response-wrapper` as three sentence-local combo patterns
- **Chose:** Detect three pattern kinds:
  - `information-wrapper`
  - `advice-limitation`
  - `diagnosis-limitation`
- **Why:** These are the real reusable families in the fixture corpus. They are more general than raw phrases, but still narrow enough to stay high-precision.
- **Alternatives considered:**
  - One giant phrase blacklist — rejected because it would be brittle and not meaningfully heuristic.
  - One vague “wrapper score” — rejected because this class is still bad enough on one strong hit to stay immediate.

### Require signal combinations, not single words
- **Chose:** Match only when the sentence combines first-person capability/limitation language with the relevant object family:
  - capability + `general information` / `general suggestions`
  - limitation + `medical advice` / `medical expertise`
  - limitation + `diagnosis` / `provide a diagnosis`
- **Why:** This prevents the rule from degrading into a generic `information` or `advice` matcher.
- **Alternatives considered:**
  - Trigger on phrases like `consult with a qualified healthcare professional` alone — rejected because that would over-catch normal human caution language.
  - Trigger on any `I can provide` sentence — rejected because that is too broad and would create obvious false positives.

### Keep plain consultation advice out of scope for now
- **Chose:** A sentence like `Consult a qualified healthcare professional for personalized advice.` stays clean unless it also has the assistant-wrapper first-person framing.
- **Why:** The current rule is meant to catch canned assistant service/refusal language, not every responsible safety sentence in medical prose.
- **Alternatives considered:**
  - Fold consultation sentences into `response-wrapper` immediately — rejected because it would broaden the rule faster than the evidence justified.

### Strip quoted spans before matching
- **Chose:** Remove double-quoted segments before running the wrapper matcher.
- **Why:** The synthetic negative `The phrase "I cannot provide medical advice" ...` initially false-positive’d. Quoted discussion of the boilerplate should not count as the boilerplate itself.
- **Alternatives considered:**
  - Special-case that exact synthetic sentence — rejected because that would fix the test without fixing the real boundary.
  - Ignore any sentence containing quotes — rejected because that would be too blunt.

### Factor sentence traversal and normalization into family-local support
- **Chose:** Add `llm-slop/runtime/src/support.rs` for shared sentence traversal, normalization, prefix stripping, substring helpers, and quoted-span stripping, and refactor `llm-disclaimer` to use it too.
- **Why:** Two rules in the family now needed the same block traversal and normalized-text helpers. This belongs in family-local support, not duplicated in each rule and not promoted to a generic core helper.
- **Alternatives considered:**
  - Duplicate traversal/normalization in `response-wrapper` — rejected because the duplication would grow immediately with the next `llm-slop` rule.
  - Promote helper logic to a broader shared crate — rejected because it is still specific to `llm-slop` ownership.

### Update fixture sidecars additively and verify no old expectations were lost
- **Chose:** Add `response-wrapper` only to the seven fixture sidecars that now fail it, leaving all previous expected rules intact.
- **Why:** The user explicitly required that fixture rewrites must not accidentally erase the previous regression floor.
- **Alternatives considered:**
  - Regenerate sidecars from scratch — rejected because it risks silently dropping old expectations.
  - Update manually without verification — rejected because the additive requirement should be proven, not assumed.

## Architectural Notes
- `llm-slop/runtime` now has:
  - `slop_01_llm_disclaimer`
  - `slop_02_response_wrapper`
  - `support.rs` as the family-local shared helper
- `response-wrapper` evidence now includes:
  - `pattern_kind`
  - `matched_signal`
  - `sentence`
- Config surface remains under `quality.heuristics.responseWrapper`, enabled by default.
- Catalog counts moved from:
  - 32 → 33 total checks
  - 27 → 28 quality checks
  - 16 → 17 heuristics checks
- Real-world corpus impact:
  - 7 Medical Outline fixtures now also fail `response-wrapper`
  - 3 files still stay on `llm-disclaimer`/`smart-quotes` only, which is acceptable because they do not contain the first-person wrapper family this rule targets

## Information Sources
- `fixtures/medicaloutline/*.md` — real-world target corpus for the wrapper patterns
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer.rs` — immediate-rule reference and shared helper consumer
- `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md` — prior `llm-slop` rule this work builds on
- Live CLI runs against:
  - `fixtures/medicaloutline/what-food-should-psoriasis-patients-avoid.md`
  - `fixtures/medicaloutline/what-is-end-stage-bipolar.md`
- A `git show HEAD:<sidecar>` comparison script used to verify no previously expected rules disappeared during sidecar updates

## Open Questions / Future Considerations
- Consultation/professional-referral wrapper language still deserves its own rule or an extension of this one later, but it should be added carefully because the false-positive risk is higher.
- `what-are-the-10-most-common-skin-disorders` and `what-are-the-10-worst-cancers` still only hit `llm-disclaimer`, which suggests the next slop rule should likely target signposting/caution boilerplate rather than widening `response-wrapper`.
- The planned regression harness still needs to consume the field-aware sidecar schema and enforce it automatically.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs` — new heuristic rule and subpattern design
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/support.rs` — family-local traversal and normalized-text helpers now shared by two rules
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_02_response_wrapper.rs` — reusable assertions for the rule’s public behavior
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper_tests/synthetic.rs` — synthetic positives/negatives, including the quoted-span false-positive guard
- `fixtures/medicaloutline/what-is-end-stage-bipolar.expected.general-en.json` — example of additive sidecar update with the new rule
- `.worklogs/2026-03-26-122019-disambiguate-fixture-evidence-sidecars.md` — sidecar schema this change relies on

## Next Steps / Continuation Plan
1. Implement the next `llm-slop` rule as an accumulative family, most likely `generic-signposting` or `boilerplate-framing`, using the Medical Outline corpus plus synthetic negatives to keep the boundary tight.
2. Keep updating fixture sidecars additively: rerun, compare against old expectations, verify no old rules disappeared, then insert the new rule only where it actually fires.
3. Build the black-box fixture regression harness so the sidecars become executable regression contracts rather than reviewed-only artifacts.
4. Consider a separate consultation-wrapper rule later if repeated real-world corpus evidence justifies it without forcing `response-wrapper` to overreach.
