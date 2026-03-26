# Add Universalizing Claims Rule

**Date:** 2026-03-26 14:27
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/*`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added `universalizing-claims` as the next accumulative `llm-slop` rule, wired it through config/presets/catalog, and bumped the workspace to `0.2.3`. The rule is synthetic-first for now: it has reusable assertions and sidecar adversarial tests, but it did not produce any new real-fixture baselines, so the existing fixture expectations stayed unchanged.

## Context & Problem
After landing `softening-language`, the next planned `llm-slop` family gap was broad canned human-generalization framing like `everyone wants...` and `we all know...`. The user explicitly wanted this done additively: preserve prior expected failures unless a drop is intentional, and prefer a heuristic family matcher over a brittle phrase bag. The current real corpus did not have strong enough clean positives to justify sidecar baseline updates, so the rule needed to ship with strong synthetic coverage first.

## Decisions Made

### Universalizing Claims Is Sentence-Leading and Narrow
- **Chose:** Detect only sentence-leading collective-subject templates followed by desire/certainty verbs.
- **Why:** This keeps the rule on rhetorical broad-human claims instead of catching narrative uses of `everyone`.
- **Alternatives considered:**
  - Broad `everyone` / `most people` substring matching — rejected because it would catch literal narrative or factual sentences like `leaves everyone shaking`.
  - More ambitious semantic matching over generic human claims — rejected because it would be harder to keep deterministic and low-noise at this stage.

### Synthetic-First Instead Of Forcing Fixture Baselines
- **Chose:** Add reusable assertions and adversarial synthetic tests, but do not rewrite fixture sidecars because the current corpus did not produce clean, intentional hits for this rule.
- **Why:** The fixture sidecars are a regression floor. Forcing weak real-corpus matches into them would make the baseline noisy and harder to trust.
- **Alternatives considered:**
  - Inventing a real-fixture baseline from weak examples in `article1` or `why_do_we_dream` — rejected because those examples are borderline or clearly outside the intended boundary.
  - Postponing the rule until a better corpus existed — rejected because the rule shape was already clear enough to implement and harden synthetically now.

### Preserve Additive Fixture Behavior
- **Chose:** Verify the full fixture corpus after implementation and confirm there were no new sidecar deltas to apply.
- **Why:** The user explicitly required additive behavior and warned against blind fixture rewrites.
- **Alternatives considered:**
  - Regenerating expectation sidecars from fresh output — rejected because that risks silently dropping older expectations.

### Bump The Workspace To 0.2.3
- **Chose:** Bump all internal crate versions and the packaged CLI release to `0.2.3`.
- **Why:** The user wants every shipped build incremented semantically, and `--version` is part of the release contract now.
- **Alternatives considered:**
  - Leaving versions unchanged until a larger batch accumulates — rejected because it violates the current release discipline for this repo.

## Architectural Notes
`universalizing-claims` follows the same owned test architecture as the other `llm-slop` rules:
- runtime rule in `runtime/src/slop_07_universalizing_claims.rs`
- reusable public-behavior assertions in `assertions/src/slop_07_universalizing_claims.rs`
- sidecar synthetic harness in `runtime/src/slop_07_universalizing_claims_tests/`

The rule uses the existing `collect_sentence_evidence` / normalization helpers from `llm-slop/runtime/src/support.rs`, but keeps its subject/verb matching logic local to the rule. That preserves the family boundary without creating premature shared matcher infrastructure.

## Information Sources
- Existing `llm-slop` rules and their assertion/test shape:
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_03_generic_signposting.rs`
- Shared test helpers:
  - `apps/prosesmasher/crates/app/checks/test-support/src/builders.rs`
  - `apps/prosesmasher/crates/app/checks/test-support/src/result_helpers.rs`
- Current fixture corpus scan for candidate real hits:
  - `fixtures/article1.mdx`
  - `fixtures/why_do_we_dream.md`
  - `fixtures/medicaloutline/*.md`
- Prior worklogs:
  - `.worklogs/2026-03-26-140309-add-softening-language-rule.md`
  - `.worklogs/2026-03-26-141541-tighten-softening-language-boundary.md`

## Open Questions / Future Considerations
- The rule currently only catches strong sentence-leading collective claims. If future corpora show cleaner patterns like `for most people, ...` or broader generic-human framing, the matcher can grow from a better evidence base.
- The fixture corpus still does not have a dedicated regression runner that reads the per-file `*.expected.general-en.json` sidecars automatically. The sidecars exist, but the automated floor is still a follow-up task.
- The catalog group tests still have legacy function names like `collect_heuristics_returns_16` even though the actual asserted count is now higher. The behavior is right; the test names are stale.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_07_universalizing_claims.rs` — new rule implementation and matching boundary
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_07_universalizing_claims.rs` — reusable rule assertions used by sidecar tests
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_07_universalizing_claims_tests/synthetic.rs` — adversarial positives and negatives for the rule boundary
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical config surface; new `universalizing_claims` field lives here
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON DTO mapping for `universalizingClaims`
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs` — catalog counts updated for the additional heuristic rule
- `apps/prosesmasher/presets/full-config-en.json` — shipped preset default enabling the new rule
- `apps/prosesmasher/CHANGELOG.md` — `0.2.3` release note for this work
- `.worklogs/2026-03-26-140309-add-softening-language-rule.md` — prior llm-slop expansion context
- `.worklogs/2026-03-26-141541-tighten-softening-language-boundary.md` — additive-baseline constraint from the previous rule adjustment

## Next Steps / Continuation Plan
1. Add the automated fixture regression runner that reads `*.expected.general-en.json` sidecars and enforces minimum expected failures without doing blind rewrites.
2. Continue the `llm-slop` family expansion with the next accumulative or immediate rule that has the strongest corpus support, likely a tighter consultation-wrapper or another rhetorical-family rule rather than broadening `universalizing-claims` prematurely.
3. If a better real-world corpus for broad human-generalization framing is added later, rerun the new rule against it and only then add sidecar baseline expectations for `universalizing-claims`.
