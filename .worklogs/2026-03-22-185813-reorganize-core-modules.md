# Reorganize core modules around semantic layers

**Date:** 2026-03-22 18:58
**Scope:** `AGENTS.md`, `apps/prosesmasher/crates/app/core/src/lib.rs`, `apps/prosesmasher/crates/app/core/src/quality/**`, `apps/prosesmasher/crates/app/core/src/document_policy/**`, `apps/prosesmasher/crates/app/core/src/runner_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/{args,checks}.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/{args_tests,checks_tests}.rs`

## Summary
Reorganized the core crate so its module layout matches the semantic model we settled on: `quality` and `document_policy` instead of the historical `terms` / `patterns` / `structure` / `readability` split. Also updated CLI grouping to the new semantic groups and kept the runtime behavior unchanged.

## Context & Problem
After the canonical config cleanup, the API boundary was correct but the code layout still taught the old taxonomy:
- root modules were `terms`, `patterns`, `structure`, `readability`
- CLI `--group` still used those historical names
- the handoff doc still described the old folder layout

That mismatch mattered because the user explicitly wanted the library organized around prose quality vs document policy, not around implementation-era buckets. Leaving the old folders in place would keep the wrong mental model alive for future work and future sessions.

## Decisions Made

### Reorganize the core crate around `quality` and `document_policy`
- **Chose:** Move check files into:
  - `quality/lexical`
  - `quality/heuristics`
  - `quality/flow`
  - `quality/readability`
  - `document_policy`
- **Why:** This matches the semantic split we already committed to in config and docs. The codebase should reinforce that model instead of fighting it.
- **Alternatives considered:**
  - Keep the old folder names and only change docs — rejected because the code layout itself would remain misleading.
  - Flatten everything into one directory — rejected because it would lose useful structure and make the crate harder to navigate.

### Keep readability as a quality subfamily
- **Chose:** Preserve `readability` as a nested module under `quality` rather than collapsing it into `heuristics` or `flow`.
- **Why:** The readability formulas are still a distinct implementation family even though they are part of core prose quality. Nesting them under `quality` preserves both truths.
- **Alternatives considered:**
  - Merge readability into `flow` — rejected because formula-based readability and paragraph/word-flow checks are different enough to deserve separate navigation.
  - Merge readability into `heuristics` — rejected because formula checks are not heuristics in the same sense as the rhetorical anti-slop detectors.

### Update CLI groups to semantic names
- **Chose:** Replace `terms`, `patterns`, and `structure` CLI groups with:
  - `quality`
  - `document-policy`
  - `lexical`
  - `heuristics`
  - `flow`
  - `readability`
- **Why:** The CLI should expose the current product model, not the old implementation folders. The subgroup names still preserve useful filtering granularity.
- **Alternatives considered:**
  - Keep old CLI groups as aliases — rejected because the user explicitly asked to stop carrying backward-compatibility surface area.
  - Expose only `quality` and `document-policy` — rejected because losing subgroups would make targeted runs less useful.

## Architectural Notes
This refactor is intentionally organizational rather than behavioral:
- check IDs did not change
- config semantics did not change
- runtime behavior did not change

What changed is the navigational skeleton of the crate:
- `quality::lexical` now owns the term/lexicon checks
- `quality::heuristics` owns the rhetorical/style detectors
- `quality::flow` owns paragraph length and word repetition
- `quality::readability` owns the formula checks
- `document_policy` owns the opt-in markdown shape checks

The one functional edge in the move was `word_repetition`, which previously depended on the old `terms` helper module. That helper call was rewired to `quality::lexical::resolve_string_override_list`.

## Information Sources
- `.worklogs/2026-03-22-183839-drop-config-compatibility.md` — canonical-only config boundary immediately before this refactor
- `AGENTS.md` — the intended semantic split and stale folder description
- `apps/prosesmasher/crates/app/core/src/lib.rs` — old root module exports
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs` — old CLI group taxonomy
- `apps/prosesmasher/crates/app/core/src/runner_tests.rs` — direct imports that needed path updates
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- The CLI now uses the new group names only. If that feels too strict for internal ergonomics, we can revisit aliases later, but that would be an explicit product choice rather than accidental legacy support.
- The check labels/messages still use historical wording in a few places, but that is cosmetic and does not affect the semantic grouping.
- The crate names and adapter boundaries are now cleaner than the file names in some tests; if we do more cleanup, test naming consistency is a reasonable next small pass.

## Key Files for Context
- `apps/prosesmasher/crates/app/core/src/lib.rs` — root semantic module exports
- `apps/prosesmasher/crates/app/core/src/quality/mod.rs` — quality aggregation layer
- `apps/prosesmasher/crates/app/core/src/document_policy/mod.rs` — document-policy aggregation layer
- `apps/prosesmasher/crates/app/core/src/quality/flow/word_repetition.rs` — representative moved check with helper path update
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs` — new CLI group mapping
- `AGENTS.md` — updated handoff doc reflecting the new module layout
- `.worklogs/2026-03-22-183839-drop-config-compatibility.md` — prior canonical-only boundary cleanup

## Next Steps / Continuation Plan
1. Audit the shipped presets against the final schema now that both config and module layout have settled. Focus on whether the preset content is actually desirable, not just structurally valid.
2. Review whether any check messages or JSON output labels should be regrouped to reflect `quality` vs `documentPolicy` more explicitly.
3. If we keep evolving the CLI, consider exposing the new group names in help/examples more prominently and maybe documenting them in a dedicated README section instead of only `AGENTS.md`.
