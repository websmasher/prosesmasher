# Add LLM Disclaimer Rule

**Date:** 2026-03-26 12:12
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/{runtime,assertions}`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/{runtime,assertions}`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `apps/prosesmasher/presets/full-config-en.json`, `fixtures/medicaloutline/*.expected.general-en.json`

## Summary
Implemented the first real `llm-slop` rule, `llm-disclaimer`, as an immediate English-only check for explicit model/disclaimer leakage. Wired it through config, presets, catalog counts, assertions, synthetic tests, and refreshed the Medical Outline sidecar expectations so the real-world fixture corpus now records the new failure.

## Context & Problem
The `llm-slop` family had only been scaffolded structurally. The newly added Medical Outline corpus immediately exposed the most obvious missing check: explicit LLM self-disclaimers like "As a language model", "As of my knowledge cutoff", and "I do not have access to real-time information". Those patterns are high-value and high-precision, so they were the right first rule to land before the softer accumulative slop families.

## Decisions Made

### Make `llm-disclaimer` an immediate English-only rule
- **Chose:** Add `slop_01_llm_disclaimer` under `app/checks/llm-slop/runtime` with `supported_locales()` restricted to `en`.
- **Why:** The current pattern set is explicitly English phrasing, and these disclaimers are bad on sight rather than threshold-based.
- **Alternatives considered:**
  - Make it locale-agnostic — rejected because the initial matcher vocabulary is English-specific.
  - Treat it as an accumulative rule — rejected because one explicit disclaimer is already unacceptable.

### Use normalized anchored phrases plus a small contains set
- **Chose:** Normalize case and curly quotes, then detect a compact set of anchored openings plus a few contained variants for "I am an AI language model", knowledge-cutoff language, and real-time-data disclaimers.
- **Why:** These are closer to signatures than to fuzzy rhetoric. A little normalization improves robustness without turning the check into a noisy free-form heuristic.
- **Alternatives considered:**
  - Exact raw-string matching only — rejected because apostrophe/quote variants and light leading prefixes would miss obvious real-world cases.
  - Broader heuristic patterning — rejected because the first rule should stay precise and easy to reason about.

### Allow small leading discourse prefixes but keep the rule exact-ish
- **Chose:** Accept disclaimers after prefixes like `However,` and `But`.
- **Why:** Real AI-written prose frequently wraps the disclaimer in a discourse marker, and the Medical Outline corpus already contains those shapes.
- **Alternatives considered:**
  - Require the disclaimer to start at column 0 — rejected because it would miss common real-world phrasing.
  - Accept arbitrary leading filler — rejected because that would broaden the rule too much for v1.

### Recurse into blockquotes and ignore code/list content
- **Chose:** Scan paragraph sentences and recurse into blockquotes, but ignore code blocks and lists.
- **Why:** This matches the existing check boundary pattern in the project and avoids flagging opaque code content.
- **Alternatives considered:**
  - Ignore blockquotes too — rejected because quoted prose is still prose in this checker model.
  - Traverse list items immediately — rejected because the current `Document` list shape does not expose sentence-level list content the same way paragraphs do, and the first version did not need that extra complexity.

### Keep assertions rule-specific and update real-fixture baselines now
- **Chose:** Add a reusable `assert_disclaimer_failure(...)` helper in the assertions crate and update all Medical Outline sidecar expectations that now fail on `llm-disclaimer`.
- **Why:** The project rule is that public API assertions live in the assertions module, while sidecars generate inputs. The new real-world corpus should move with the new shipped behavior instead of keeping stale "smart-quotes only" expectations.
- **Alternatives considered:**
  - Let sidecar tests assert directly on raw output — rejected because it would reintroduce assertion drift.
  - Delay sidecar updates until a future regression harness exists — rejected because the checked-in corpus baseline would immediately be stale.

## Architectural Notes
- `llm-slop/runtime` now owns a real first rule instead of exporting an empty `all_checks()`.
- `llm-slop/assertions` now follows the same rule-specific assertion pattern as the other families.
- The new rule is wired through the existing public config surface under `quality.heuristics.llmDisclaimer`, preserving the current external schema and umbrella `heuristics` grouping.
- Catalog counts moved from 31 to 32 total checks, with heuristics rising from 15 to 16.
- The Medical Outline corpus is now more useful as a regression seed for future `llm-slop` work because it records the new exact disclaimer failures per article.

## Information Sources
- `fixtures/medicaloutline/*.md` — real-world seed corpus for the disclaimer patterns
- `apps/prosesmasher/crates/app/checks/style-signals/runtime/src/*.rs` and other existing immediate rules — reference shape for immediate-check implementation
- `.worklogs/2026-03-26-113930-add-medicaloutline-ai-fixtures.md` — provenance for the Medical Outline corpus
- `.worklogs/2026-03-26-115433-add-fixture-expected-failures.md` — sidecar expectation structure that this change updates
- `.worklogs/2026-03-26-105416-split-heuristics-into-owned-families.md` — family split that established `llm-slop` as the right home

## Open Questions / Future Considerations
- This first rule does not yet suppress quoted/meta-discussion contexts beyond the naturally safe non-leading quoted case; future slop rules will need a more explicit treatment of quotation semantics.
- The next obvious immediate rule is `response-wrapper`, followed by the softer accumulative `generic-signposting` and `llm-vocabulary` families.
- A black-box regression harness still needs to be added to consume the sidecar expectation files automatically.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer.rs` — the new rule implementation and matcher vocabulary
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_01_llm_disclaimer.rs` — reusable public-behavior assertions for the rule
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer_tests/synthetic.rs` — synthetic harness cases, including knowledge-cutoff and real-time disclaimer variants
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical config default for `llm_disclaimer`
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON loader wiring for `llmDisclaimer`
- `fixtures/medicaloutline/how-can-i-help-my-child-with-adhd-without-medication.expected.general-en.json` — example sidecar showing the new real-world baseline
- `.worklogs/2026-03-26-115433-add-fixture-expected-failures.md` — prior fixture-sidecar decision record

## Next Steps / Continuation Plan
1. Implement `response-wrapper` in `app/checks/llm-slop/{runtime,assertions}` as the next immediate rule, using the Medical Outline corpus as the first real-world target.
2. Rerun the Medical Outline fixture set after each new `llm-slop` rule and update the per-article sidecars immediately so the corpus baseline stays aligned with shipped behavior.
3. Add the black-box fixture regression harness that reads `*.expected.general-en.json` sidecars and enforces `rule` + `evidenceContains` automatically.
4. After the immediate rules, add the first accumulative `llm-slop` checks (`generic-signposting`, `llm-vocabulary`) with the same rule-assertions structure and real-fixture parity tests.
