# Rename Slogan Punchline To Contrastive Aphorism

**Date:** 2026-03-26 21:30
**Scope:** `apps/prosesmasher/CHANGELOG.md`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/presets/full-config-en.json`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`, `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/lib.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism_tests/mod.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism_tests/synthetic.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/lib.rs`, `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_10_contrastive_aphorism.rs`, workspace crate manifests under `apps/prosesmasher/**/Cargo.toml`

## Summary
Renamed the public `slogan-punchline` check to `contrastive-aphorism`, promoted the workspace to `0.3.0` because that rename changes the public CLI/config surface, and extended safe coverage for corrective negation and meta framing without widening into generic `not X` prose. The pass also verified that the six-model generated baseline corpus still compares cleanly after the rename and new matchers.

## Context & Problem
The prior name `slogan-punchline` was too fuzzy for the actual rhetorical family the rule owns. The user explicitly preferred `contrastive-aphorism`, which better describes the short coaching-style contrast lines we want to detect. At the same time, there were still uncaught lines in new snippets that looked sloppily contrastive or corrective, but previous attempts to widen generic negation had already produced false positives on legitimate prose such as factual or analytical `from X, not Y` sentences.

The work therefore needed to do three things together:
- rename the rule cleanly across runtime, assertions, config DTOs, presets, and public IDs
- keep corrective reframes (`X is not the problem. Y is.`) under `negation-reframe`
- keep short aphoristic contrasts (`Bring a pattern, not a vibe.`) under the renamed rule instead of turning `negation-reframe` into a catch-all for any short `not` sentence

## Decisions Made

### Rename The Rule As A Breaking Public Change
- **Chose:** Rename `slogan-punchline` to `contrastive-aphorism` everywhere the user can see it: public check ID, label, config key, preset JSON, domain config, runtime module names, and assertions module names.
- **Why:** The new name matches the actual ownership boundary. The rule detects short contrastive coaching aphorisms, not generic punchlines.
- **Alternatives considered:**
  - Keep `slogan-punchline` — rejected because it described the tone vaguely and did not tell future rule authors what belongs in the rule.
  - Fold the behavior into `negation-reframe` — rejected because the function is different even when both families use negation syntax.

### Separate Corrective Reframes From Contrastive Aphorisms
- **Chose:** Extend `negation-reframe` only for safe corrective forms such as `The biggest sign is not X. It is Y.` and `X is not the problem. Y is.`
- **Why:** These are true reframing structures with explicit relabeling. They fit the existing rule's semantic boundary.
- **Alternatives considered:**
  - Catch all short `not X / Y` lines in `negation-reframe` — rejected because that had already produced false positives on legitimate analytical prose.
  - Move all new negation lines to `contrastive-aphorism` — rejected because corrective reframing is already a coherent existing family with assertions and expectations.

### Keep Meta Framing Narrow And Additive
- **Chose:** Add `the useful move is` as a new `generic-signposting` pattern but leave it accumulative rather than making it an immediate failure.
- **Why:** Single uses of this frame are not enough to justify a failure, but repeated use contributes to the low-information explanatory voice we are targeting.
- **Alternatives considered:**
  - Make `the useful move is` immediate — rejected because that would make the rule too brittle on isolated legitimate uses.
  - Leave it uncaught — rejected because it is exactly the kind of empty procedural framing the user called out.

### Verify Against The Generated Regression Corpus Before Shipping
- **Chose:** Re-run the full workspace tests, CLI version output, and `generated_fixture_failures.py compare` across all six generated model buckets before committing.
- **Why:** The rename changes the public surface, and the matcher additions needed confirmation that they did not quietly expand into the 60-article baseline corpus.
- **Alternatives considered:**
  - Rely on targeted synthetic tests only — rejected because the generated corpus is the best existing practical false-positive backstop.
  - Skip the compare because the rename is mostly cosmetic — rejected because the compare is what proves the rename and added matchers did not change observed counts on the corpus.

## Architectural Notes
- `contrastive-aphorism` now owns short aphoristic contrasts like `Bring a pattern, not a vibe.` and `Watch for a pattern, not one bad week.` even though they use negation syntax. This keeps rule ownership based on rhetorical function, not superficial token shape.
- `negation-reframe` continues to own explicit corrective reframing, including repeated-subject `need`/`want` forms and the new `X is not the problem. Y is.` family.
- Because the rename changes public IDs and config keys, the workspace version was bumped to `0.3.0` instead of another patch release.
- The sidecar/assertions split remains intact: rule-specific assertions still live in the assertions crate, and synthetic scenario generation remains in sidecar test directories.

## Information Sources
- User guidance in-session about keeping `contrastive-aphorism` separate from `negation-reframe`
- Existing rule implementations and tests in:
  - `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`
  - `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`
- Previous worklog:
  - `.worklogs/2026-03-26-211647-extend-corrective-and-contrastive-negation-coverage.md`
- Verification commands run in this pass:
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
  - `cargo run --manifest-path apps/prosesmasher/Cargo.toml -q -p prosesmasher -- --version`

## Open Questions / Future Considerations
- The new snippet set still contains uncaught deictic/magnifier lines like `That is still real change.` and `That is how the pattern weakens.` Those probably belong in a future `empty-emphasis` subtype, but they were intentionally left out here.
- `... not just a discipline issue ...` remains intentionally uncaught because the current boundary for that pattern is still too loose.
- If more generated or real-world corpora start producing corrective negation in legitimate prose, `problem_reframe_corrective` is the first branch to re-audit.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs` — runtime matcher for the renamed rule and its new short contrast branches
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism_tests/synthetic.rs` — synthetic positives and negatives that pin the intended boundary
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_10_contrastive_aphorism.rs` — reusable assertion helpers for the public behavior of the rule
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — corrective negation rule with the new safe reframing branches
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — tests that distinguish safe reframes from normal prose
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — accumulative meta-framing rule with `the useful move is`
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — canonical runtime config shape including `contrastive_aphorism`
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — public JSON config mapping for `contrastiveAphorism`
- `apps/prosesmasher/CHANGELOG.md` — `0.3.0` release notes documenting the breaking rename
- `.worklogs/2026-03-26-211647-extend-corrective-and-contrastive-negation-coverage.md` — previous pass that introduced the first contrastive coverage and is the direct predecessor to this rename

## Next Steps / Continuation Plan
1. Review the remaining uncaught coaching snippets and decide whether `That is still real change.` / `That is how the pattern weakens.` belong in `empty-emphasis` or a narrower new rule.
2. Keep collecting real misses from generated and real-world corpora before broadening `generic-signposting` again; the next additions should come from repeated evidence, not intuition.
3. If future public-surface renames happen again, keep treating them as version-boundary changes and update fixture sidecars or config docs in the same pass rather than as follow-up cleanup.
