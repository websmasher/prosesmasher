# Add Softening Language Rule

**Date:** 2026-03-26 14:03
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/presets/full-config-en.json`, `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs`, `fixtures/*.expected.general-en.json`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/**/Cargo.toml`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added `softening-language` as the sixth `llm-slop` rule, recorded its first real fixture hits additively, and bumped the workspace from `0.2.0` to `0.2.1`. The rule is accumulative, English-only, and targets repeated stacked low-commitment phrasing instead of single stray hedge words.

## Context & Problem
After `llm-disclaimer`, `response-wrapper`, `generic-signposting`, `boilerplate-framing`, and `llm-vocabulary`, the next obvious gap in the Medical Outline corpus was repeated low-commitment language: modal-heavy caveats, variability disclaimers, and tentative-reporting frames. The user explicitly asked for heuristics over exact phrase lists, and also asked that every release-worthy build bump the semantic version and keep `--version` truthful.

The constraints were:
- avoid a noisy “any `may` fails” rule
- keep the rule accumulative inside `llm-slop`
- preserve all prior fixture expectations when recording new hits
- ship the feature as a versioned release step, not an unversioned drift commit

## Decisions Made

### Make `softening-language` sentence-level and accumulative
- **Chose:** count only sentences that stack at least two softening signals, then fail the document when those stacked sentences repeat past `maxPerDocument`.
- **Why:** single hedges are normal prose; repeated hedge-heavy sentences are the slop signal. This keeps the rule aligned with the earlier immediate-vs-accumulative design.
- **Alternatives considered:**
  - Fail on any hedge term like `may` or `might` — rejected because it would explode with false positives.
  - Score every hedge word separately — rejected because that turns the rule into a noisy token counter rather than a rhetorical detector.

### Use signal families instead of raw phrase bags
- **Chose:** model the matcher around signal families:
  - modals (`may`, `might`, `could`)
  - qualifiers (`commonly`, `often`, `potentially`, `generally`, etc.)
  - variability phrases (`from person to person`, `may not necessarily`, `in some individuals`, etc.)
  - tentative-reporting phrases (`generally considered`, `research suggests`, `more research is needed`)
  - quantifier+noun pairs (`some people`, `certain foods`, `some studies`, etc.)
- **Why:** this is general enough to catch family variants while staying much tighter than a free-floating fuzzy heuristic.
- **Alternatives considered:**
  - exact sentence templates only — rejected because that would miss obvious nearby variants in the corpus.
  - broad semantic NLP-style classification — rejected because the tool is intentionally deterministic and local.

### Record only the first real fixture hits that looked worth freezing
- **Chose:** add `softening-language` only to the four sidecars that actually tripped:
  - `what-cancers-cannot-be-cured`
  - `what-food-is-not-good-for-eczema`
  - `what-food-should-psoriasis-patients-avoid`
  - `why_do_we_dream`
- **Why:** these are the real current positives. Recording non-hits or speculative future hits would make the sidecars misleading.
- **Alternatives considered:**
  - regenerate all sidecars wholesale — rejected because the sidecars are a regression floor, not a disposable snapshot.
  - force the rule into more Medical Outline articles before shipping — rejected because the current boundary already looks acceptably precise.

### Bump the workspace to `0.2.1`
- **Chose:** release this change as `0.2.1`.
- **Why:** the user asked for semantic version bumps on each build/release step. This is a backward-compatible new rule and fixture expansion after `0.2.0`, so a patch increment is the cleanest fit.
- **Alternatives considered:**
  - leave the workspace at `0.2.0` — rejected because it violates the new versioning requirement.
  - jump to `0.3.0` — rejected because this is additive, not a new compatibility boundary.

## Architectural Notes
`softening-language` follows the same `llm-slop` family structure as the previous rules:
- runtime rule in `slop_06_softening_language.rs`
- reusable assertion surface in `assertions/src/slop_06_softening_language.rs`
- sidecar synthetic harness in `slop_06_softening_language_tests/`

The rule emits one evidence item per matched sentence, not per hedge word. That keeps the accumulated count semantically meaningful and makes fixture sidecars much easier to review.

The release bump touched every publishable crate manifest again because the workspace still uses explicit internal path dependency versions for crates.io/publish coherence.

## Information Sources
- Existing `llm-slop` rules and support:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_01_llm_disclaimer.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_04_boilerplate_framing.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_05_llm_vocabulary.rs`
- Real corpus files:
  - `fixtures/medicaloutline/what-cancers-cannot-be-cured.md`
  - `fixtures/medicaloutline/what-food-is-not-good-for-eczema.md`
  - `fixtures/medicaloutline/what-food-should-psoriasis-patients-avoid.md`
  - `fixtures/why_do_we_dream.md`
- Release/version context:
  - `apps/prosesmasher/Cargo.toml`
  - `apps/prosesmasher/CHANGELOG.md`
  - `.worklogs/2026-03-26-134838-bump-workspace-version-and-enforce-cli-version.md`

## Open Questions / Future Considerations
- `softening-language` is intentionally narrow right now. More corpora may justify broader quantifier targets or additional tentative-reporting phrases, but only if they remain low-noise.
- The catalog test function names still contain stale old count suffixes (`collect_all_returns_32`, etc.) even though the asserted counts were updated; those names are now misleading cleanup debt.
- If fixture sidecars become numerous, the next structural step should be an automated black-box regression harness that reads the per-file `.expected.general-en.json` contracts.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language.rs` — rule implementation and signal-family matcher.
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_06_softening_language.rs` — reusable public-output assertions for this rule.
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language_tests/synthetic.rs` — synthetic adversarial coverage and boundary cases.
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — new `softening_language` config wiring and defaults.
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON config loader support for `softeningLanguage`.
- `fixtures/medicaloutline/what-food-is-not-good-for-eczema.expected.general-en.json` — representative real-world sidecar hit for this rule.
- `fixtures/why_do_we_dream.expected.general-en.json` — non-Medical Outline corpus hit proving the rule is not tied to one source.
- `.worklogs/2026-03-26-134227-add-llm-vocabulary-rule.md` — immediate predecessor in the `llm-slop` sequence.
- `.worklogs/2026-03-26-134838-bump-workspace-version-and-enforce-cli-version.md` — prior versioning decision that this work continues.

## Next Steps / Continuation Plan
1. Implement `universalizing-claims` next in `llm-slop`, using the same accumulative sentence-level approach and sidecar/assertions structure.
2. Add a black-box regression test that reads `fixtures/*.expected.general-en.json` and enforces the per-file rule/evidence contracts automatically.
3. Clean up stale catalog test names so the function names match the new asserted counts after the `llm-slop` expansion.
