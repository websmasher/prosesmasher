# Fix Sentence-Case CLI Release Boundary

**Date:** 2026-03-26 17:45
**Scope:** `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/output.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/output_tests/*`, `apps/prosesmasher/packages/prosesmasher/*`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`, workspace `Cargo.toml` version references

## Summary
Restored the packaged `prosesmasher` wrapper crate so the workspace can run `cargo run -p prosesmasher` again, then fixed the public CLI output boundary so `sentence-case` emits the stable check ID `sentence-case` instead of leaking per-heading internal expectation column names into JSON. Released the repaired CLI as `0.2.4` and verified the full workspace plus packaged smoke tests.

## Context & Problem
The packaged CLI workspace was broken because the tracked `apps/prosesmasher/packages/prosesmasher/*` files had been deleted locally, which made `cargo run --manifest-path apps/prosesmasher/Cargo.toml -p prosesmasher` fail with a missing workspace member error. At the same time, real corpus runs exposed a public-output bug in the `sentence-case` check: the runtime intentionally records one expectation per heading using internal columns like `sentence-case-<heading text>`, but the CLI formatter was publishing those raw internal column names as public JSON `id` values. Another agent needed a usable released CLI immediately, so the fix needed to restore the package wrapper and stabilize the public output contract without destabilizing the check internals.

## Decisions Made

### Restore the packaged wrapper crate instead of working around it
- **Chose:** Restore the deleted `apps/prosesmasher/packages/prosesmasher` package files and assertions helper crate exactly into the workspace.
- **Why:** The user asked to release a usable CLI for another agent. Using the already-built binary was only a local workaround; the actual developer workflow requires `cargo run -p prosesmasher` and the packaged smoke tests to work again.
- **Alternatives considered:**
  - Keep using `target/debug/prosesmasher` — rejected because it does not fix the broken workspace or help another agent running from source.
  - Remove the package from the workspace — rejected because it would rip out the published wrapper package and packaged smoke coverage that already exists.

### Fix sentence-case at the CLI output boundary, not inside the rule runtime
- **Chose:** Add a small public-ID normalization helper in `output.rs` so any internal `sentence-case-*` expectation column is emitted as the stable public ID `sentence-case`.
- **Why:** The runtime check still benefits from per-heading expectations and evidence. The bug was not the internal storage shape; it was the adapter leaking internal columns through the public output surface.
- **Alternatives considered:**
  - Rewrite the sentence-case rule to aggregate all headings into one expectation — rejected because it is a larger behavioral refactor with more test fallout than needed for this release.
  - Leave the dynamic IDs alone and document them — rejected because the public JSON contract should not encode heading text into check IDs.

### Add a direct regression test for the leaked-ID boundary
- **Chose:** Add `output_tests/sentence_case_public_id.rs` in the CLI runtime crate.
- **Why:** The failure mode was adapter-specific. The existing style-signals rule tests only validate the internal expectation result shape; they would not catch future re-leaks through the JSON formatter.
- **Alternatives considered:**
  - Rely on existing sentence-case rule tests — rejected because they intentionally inspect internal `sentence-case-*` keys and do not cover the public CLI contract.

### Release as `0.2.4`
- **Chose:** Bump the workspace and internal crate dependency versions from `0.2.3` to `0.2.4`, update the changelog, and verify `--version`.
- **Why:** The user asked to release the new CLI. The wrapper restoration and public output contract fix both affect the shipped CLI behavior and packaging flow.
- **Alternatives considered:**
  - Leave the version unchanged — rejected because that would make the repaired CLI indistinguishable from the broken `0.2.3` source state.

## Architectural Notes
The key architectural choice here is preserving the split between internal rule instrumentation and public adapter output. `SentenceCaseCheck` still emits one expectation per heading, which keeps evidence localized and assertion helpers simple. The CLI adapter now owns the translation from internal expectation columns to public check IDs. That is the correct hex boundary: runtime checks may use fine-grained internal identifiers, but adapters must publish stable consumer-facing IDs.

The packaged wrapper crate remains the correct place for `cargo run -p prosesmasher`, release metadata, and smoke tests that exercise the published binary shape. Restoring it keeps the package/release path aligned with previous work instead of inventing a second execution path.

Running `cargo fmt --all` after the fix also reformatted some already-existing Rust files outside the exact sentence-case path. Those are formatting-only changes, not new behavioral work, but they are included in this release commit because the workspace policy is to keep the tree formatted and green.

## Information Sources
- `apps/prosesmasher/crates/app/checks/style-signals/runtime/src/heur_02_sentence_case.rs` — confirmed the runtime intentionally creates per-heading internal columns.
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/output.rs` — identified the raw internal column leak into public JSON IDs.
- `apps/prosesmasher/crates/app/checks/style-signals/assertions/src/heur_02_sentence_case.rs` — confirmed existing rule assertions depend on internal `sentence-case-*` IDs and therefore should not be rewritten for this release.
- `apps/prosesmasher/packages/prosesmasher/*` at `HEAD` — used as the source of truth for restoring the missing package wrapper files.
- `.worklogs/2026-03-26-134838-bump-workspace-version-and-enforce-cli-version.md` — prior release/versioning pattern.
- `.worklogs/2026-03-26-142727-add-universalizing-claims-rule.md` — reminder to bump only `prosesmasher*` package versions and keep release semantics consistent.

## Open Questions / Future Considerations
- The public JSON now emits a stable `sentence-case` ID, but if multiple headings fail there can still be multiple failure entries with the same public ID. That is acceptable for now because evidence stays distinct, but a future aggregation pass may want a single grouped failure object.
- The new AI-generated fixture corpora are still untracked and separate from this release work. They should be reviewed and integrated intentionally in a later task.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/output.rs` — public JSON/text output shaping and public-ID normalization.
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/output_tests/sentence_case_public_id.rs` — regression test that guards the sentence-case public contract.
- `apps/prosesmasher/crates/app/checks/style-signals/runtime/src/heur_02_sentence_case.rs` — internal per-heading expectation design for sentence-case.
- `apps/prosesmasher/packages/prosesmasher/Cargo.toml` — packaged CLI wrapper crate metadata and dependency surface.
- `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs` — packaged binary smoke coverage, including `--version`.
- `apps/prosesmasher/CHANGELOG.md` — `0.2.4` release note.
- `.worklogs/2026-03-26-134838-bump-workspace-version-and-enforce-cli-version.md` — prior versioning/release baseline.
- `.worklogs/2026-03-26-142727-add-universalizing-claims-rule.md` — most recent pre-release version bump context.

## Next Steps / Continuation Plan
1. If sentence-case consumers need one failure per check instead of one failure per heading, design a deliberate aggregation layer in the CLI adapter and update the JSON contract tests first.
2. Turn the per-fixture `.expected.general-en.json` sidecars into an automated regression runner so the growing fixture corpora become a stable release gate.
3. Review the untracked model-generated fixture directories under `fixtures/` and decide which should be committed as long-term corpora for the next `llm-slop` rule iterations.
