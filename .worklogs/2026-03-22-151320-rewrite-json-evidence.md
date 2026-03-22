# Rewrite-oriented JSON evidence for validation loops

**Date:** 2026-03-22 15:13
**Scope:** `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs`, `apps/prosesmasher/crates/app/core/Cargo.toml`, `apps/prosesmasher/crates/app/core/src/patterns/*`, `apps/prosesmasher/crates/app/core/src/structure/*`, `apps/prosesmasher/crates/app/core/src/readability/*`, `apps/prosesmasher/crates/app/core/src/terms/banned_words.rs`, `apps/prosesmasher/crates/app/core/src/terms/banned_phrases.rs`, `apps/prosesmasher/crates/app/core/src/**/*_tests.rs`, `/Users/tartakovsky/Projects/websmasher/low-expectations/crates/low-expectations/src/suite.rs`

## Summary
Expanded `prosesmasher` JSON output from simple pass/fail reporting into a rewrite-oriented contract suitable for LLM validation loops. Added deterministic `rewrite_brief`, `failures`, `expected`, `checking`, and `evidence` fields, then upgraded checks across terms, patterns, structure, and readability to emit structured evidence objects instead of only scalar counts where that was feasible.

## Context & Problem
The project was already capable of validating prose and returning JSON, but the JSON was optimized for CI rather than for iterative rewrite loops. The user’s target workflow is: have an LLM write prose, run `prosesmasher`, feed the failures back to the LLM, and request a rewrite. That loop needs deterministic, machine-readable rewrite guidance and evidence, not just `success: false` and a raw observed value.

The initial output shape lacked:
- top-level rewrite intent (`rewrite_needed`)
- a compact list of rewrite actions (`rewrite_brief`)
- per-failure deterministic guidance (`message`, `rewrite_hint`)
- explicit thresholds (`expected`)
- sentence/paragraph/heading-level evidence for many checks

Without those, the orchestration layer had to reverse-engineer what went wrong and what to do next. That is wasted logic and makes the rewrite loop less reliable.

## Decisions Made

### Keep backward compatibility while adding rewrite fields
- **Chose:** Preserve the existing `checks` array and top-level summary counts while adding `summary`, `rewrite_needed`, `rewrite_brief`, and `failures`.
- **Why:** Existing consumers can continue using the old shape, while rewrite-loop consumers get a richer contract without a migration cliff.
- **Alternatives considered:**
  - Replace `checks` entirely with `failures` only — rejected because it would break existing JSON consumers and remove full-suite visibility.
  - Add a second output format flag for “rewrite JSON” — rejected because it would fragment the CLI surface and duplicate serializer logic.

### Add a structured custom evidence path in `low-expectations`
- **Chose:** Extend `ExpectationSuite` with `record_custom_values()` that accepts JSON `expected`, JSON `found`, and JSON error/evidence values.
- **Why:** Many checks can provide deterministic evidence objects, but the existing custom path only accepted strings. Stringifying JSON into `partial_unexpected_list` would have been an ugly, lossy contract.
- **Alternatives considered:**
  - Keep using `record_custom()` with stringified JSON — rejected because downstream parsing would be brittle and the output contract would be dishonest.
  - Modify all checks to fit existing scalar expectation helpers — rejected because many checks are inherently sentence/paragraph-level and need custom evidence payloads.

### Prefer deterministic evidence over “smart” rewrite advice
- **Chose:** Emit exact matched text, sentence text, paragraph text, heading text, counts, thresholds, and indexes where available, plus template-based rewrite hints.
- **Why:** This keeps the system deterministic and composable. The model can do the semantic rewrite; `prosesmasher` should diagnose precisely and reproducibly.
- **Alternatives considered:**
  - Generate semantic replacement advice per check — rejected because that quickly drifts into LLM-like behavior and heuristic guesswork.
  - Only emit counts and matched terms — rejected because it is too weak for the rewrite-loop use case.

### Centralize common evidence collection for pattern checks
- **Chose:** Add shared helpers in `patterns/mod.rs` for section opener/closer sentence selection and sentence phrase matching.
- **Why:** Several rhetoric checks share the same structure. Centralizing their evidence builders keeps evidence shape consistent and reduces duplicated traversal code.
- **Alternatives considered:**
  - Hand-roll evidence collection in each check — rejected because it would create drift in structure and make future maintenance harder.

## Architectural Notes
This work kept the existing dependency boundaries intact:
- `low-expectations` remains a generic validation substrate, but now supports structured JSON payloads for custom expectations.
- `app/core` remains responsible for deterministic diagnosis and evidence generation.
- `adapters/inbound/cli` remains the place where suite results are shaped into the external JSON contract.

The main architectural shift is that checks now act as evidence producers, not just counters. That is important because the rewrite loop needs localized targets. The serializer layer was deliberately kept generic: it reads `ExpectationConfiguration` and `ResultDetail` and exposes them as `expected`, `checking`, `observed`, and `evidence`, rather than hard-coding per-check parsing everywhere.

The initial evidence rollout covered:
- terms: banned words, banned phrases
- patterns: em dashes, smart quotes, llm openers, affirmation closers, summative closer, false question, humble bragger, jargon faker, negation-reframe, triple-repeat, fake timestamps, colon-dramatic, exclamation density
- structure: paragraph length, sentence case, heading hierarchy, heading counts, code fences, word repetition
- readability: flesch-kincaid, gunning fog, coleman-liau, avg sentence length

Trade-off accepted: some checks still expose document-level or aggregated evidence rather than precise spans because span-level tracking does not currently exist in the parsed document model. The evidence is still substantially better than pre-change scalar output.

## Information Sources
- `AGENTS.md` — current handoff context and known limitations
- `.worklogs/2026-03-22-141624-agents-md.md` — recent handoff writeup
- `.worklogs/2026-03-22-125808-required-recommended-terms.md` — recent check-addition context
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — JSON output contract
- `apps/prosesmasher/crates/app/core/src/patterns/*.rs` — pattern checks and matching heuristics
- `apps/prosesmasher/crates/app/core/src/structure/*.rs` — structure checks
- `apps/prosesmasher/crates/app/core/src/readability/*.rs` — readability checks
- `apps/prosesmasher/crates/app/core/src/terms/*.rs` — term checks
- `/Users/tartakovsky/Projects/websmasher/low-expectations/crates/low-expectations/src/suite.rs` — custom expectation recording
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- Span-level evidence is still missing. Sentence/paragraph/heading evidence is enough for a strong rewrite loop, but exact character offsets would make downstream UI and patching cleaner.
- Not every check benefits equally from evidence. Some thresholds (for example broad readability metrics) are still aggregate diagnostics by nature.
- Config presets are now the next bottleneck: the loop contract is much stronger, but adoption still depends on shipping useful default rule bundles.

## Key Files for Context
- `AGENTS.md` — overall project handoff and next-step framing
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — final rewrite-oriented JSON contract
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs` — serializer contract tests
- `/Users/tartakovsky/Projects/websmasher/low-expectations/crates/low-expectations/src/suite.rs` — structured custom evidence support
- `apps/prosesmasher/crates/app/core/src/patterns/mod.rs` — shared pattern evidence helpers
- `apps/prosesmasher/crates/app/core/src/patterns/negation_reframe.rs` — representative sentence-pair evidence producer
- `apps/prosesmasher/crates/app/core/src/structure/paragraph_length.rs` — representative paragraph-level evidence producer
- `apps/prosesmasher/crates/app/core/src/structure/sentence_case.rs` — representative heading-level evidence producer
- `apps/prosesmasher/crates/app/core/src/readability/flesch_kincaid.rs` — representative aggregate metric evidence producer
- `.worklogs/2026-03-22-141624-agents-md.md` — recent state summary before this rewrite-loop pass

## Next Steps / Continuation Plan
1. Define config preset goals before writing files: at minimum decide whether presets are audience-based, strictness-based, or content-type-based, since the JSON contract is now ready to support all three.
2. Add preset config files under the app workspace in a discoverable location, along with tests that load them successfully and assert a few key threshold/term choices.
3. Decide how presets enter the CLI UX: explicit `--config path`, preset name resolution, or future `prosesmasher init`.
4. Document the rewrite-loop contract with one concrete example JSON payload and one example “LLM rewrite prompt” that consumes it.
