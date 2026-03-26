# Add LLM Vocabulary Rule

**Date:** 2026-03-26 13:42
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/assertions/src/config_loader.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `fixtures/why_do_we_dream.expected.general-en.json`

## Summary
Added `llm-vocabulary` as the fifth `llm-slop` rule. The rule is accumulative, English-only, and flags repeated stock LLM-era diction while staying below threshold for isolated stray terms.

## Context & Problem
After landing `llm-disclaimer`, `response-wrapper`, `generic-signposting`, and `boilerplate-framing`, the remaining obvious gap in the new fixture corpus was repeated canned vocabulary like `delve`, `moreover`, `vital`, and `comprehensive`. The new Medical Outline corpus did not expose much of it, but `why_do_we_dream.md` did, and that fixture was already failing `prohibited-terms` partly because `delve` leaked through a different check. The goal here was to add a dedicated `llm-slop` rule for the diction cluster without turning it into a brittle one-off phrase blacklist or rewriting fixture baselines destructively.

## Decisions Made

### Make `llm-vocabulary` accumulative instead of immediate
- **Chose:** model the rule as `AccumulativeCheck` with `enabled` + `maxPerDocument`, defaulting to `1`.
- **Why:** one stray loaded word is not enough evidence; repeated stock diction is. This matches the earlier design decision that soft LLM slop should accumulate within its own category instead of firing on single use.
- **Alternatives considered:**
  - Immediate rule on any match — rejected because a single `comprehensive` or `vital` would be noisy.
  - Distinct-term-only scoring — rejected because repeated `delve` is bad even if it is the only term.

### Keep the matcher lexical and narrow for v1
- **Chose:** use a fixed lexicon of strongly LLM-coded terms and count every hit in prose and blockquotes, while ignoring code blocks and quoted segments.
- **Why:** this keeps the rule precise and cheap while still catching real fixture failures. Quoted discussion of bad AI words should not fail the rule, but live prose should.
- **Alternatives considered:**
  - Broad rhetorical heuristics — rejected because that belongs in other `llm-slop` rules, not vocabulary.
  - Morphological/stemming expansion — rejected for now because it adds noise before we have enough corpus pressure.

### Preserve fixture expectations additively
- **Chose:** update only `fixtures/why_do_we_dream.expected.general-en.json` and verify that no previously expected rule disappeared.
- **Why:** sidecars are a regression floor. New rules should only add coverage unless we explicitly decide to retire an old expectation.
- **Alternatives considered:**
  - Blindly regenerate all fixture sidecars — rejected because it can silently erase existing regression guarantees.

## Architectural Notes
`llm-vocabulary` follows the same structure as the other `llm-slop` rules:
- runtime rule in `slop_05_llm_vocabulary.rs`
- reusable public-output assertions in `assertions/src/slop_05_llm_vocabulary.rs`
- sidecar synthetic harness in `slop_05_llm_vocabulary_tests/`

The rule uses custom block traversal instead of the generic sentence-evidence helper because a single sentence can legitimately emit multiple vocabulary hits. That makes the evidence shape more faithful for regression sidecars and future fixture review.

## Information Sources
- Existing `llm-slop` rules and support helpers in:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs`
- Fixture corpus and baseline sidecars:
  - `fixtures/why_do_we_dream.md`
  - `fixtures/why_do_we_dream.expected.general-en.json`
  - `fixtures/medicaloutline/*.md`
- Prior worklogs:
  - `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md`
  - `.worklogs/2026-03-26-124105-add-response-wrapper-rule.md`
  - `.worklogs/2026-03-26-130445-add-generic-signposting-rule.md`
  - `.worklogs/2026-03-26-132155-add-boilerplate-framing-rule.md`
  - `.worklogs/2026-03-26-133343-tighten-boilerplate-framing-multi-hit.md`

## Open Questions / Future Considerations
- The current lexicon is intentionally narrow. Real-world corpus review may justify adding inflections or new stock terms, but only if they remain high-signal.
- `why_do_we_dream.md` is the only current real fixture hit. More corpora will probably surface better candidates than the Medical Outline set.
- The eventual `softening-language` rule may overlap in perceived “AI feel,” but it should remain separate because it is rhetorical accumulation, not vocabulary reuse.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_05_llm_vocabulary.rs` — new accumulative rule implementation and evidence collection.
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_05_llm_vocabulary.rs` — reusable assertions for all public-behavior tests of the rule.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_05_llm_vocabulary_tests/synthetic.rs` — synthetic positive/negative harness coverage.
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — new `llm_vocabulary` config wiring and defaults.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON config DTO support for `llmVocabulary`.
- `fixtures/why_do_we_dream.expected.general-en.json` — first real fixture baseline for this rule.
- `.worklogs/2026-03-26-133343-tighten-boilerplate-framing-multi-hit.md` — immediate predecessor in the `llm-slop` buildout.

## Next Steps / Continuation Plan
1. Implement `softening-language` as the next accumulative `llm-slop` rule, using the same sidecar/assertions pattern and keeping the matcher family-based rather than phrase-exact.
2. Re-run the full fixture corpus after each new rule and update sidecars additively, never dropping prior expected failures without an explicit decision.
3. After the next 2-3 `llm-slop` rules land, add a black-box regression harness that reads per-fixture `.expected.general-en.json` sidecars and enforces them automatically.
