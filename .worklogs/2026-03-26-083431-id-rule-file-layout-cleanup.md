# Align Rule Files To ID-Based Layout

**Date:** 2026-03-26 08:34
**Scope:** `apps/prosesmasher/crates/app/checks/{heuristics,lexical,flow,readability,document-policy}/{runtime,assertions}`, `apps/prosesmasher/crates/app/checks/catalog/runtime`

## Summary
Renamed the concrete `app/checks` rule files, sidecar test folders, and sibling assertions files to ID-based stems such as `heur_01_em_dashes` and `doc_01_word_count`. Kept the semantic Rust module names stable through `#[path = "..."]` wiring, removed stale empty structural leftovers, and revalidated the full workspace.

## Context & Problem
After the earlier runtime/assertions migration, the codebase had family crates and sidecar tests, but the on-disk layout still did not look like the GuardRails-style rule tree the user expected. The concrete problems called out were:

- `packages/prosesmasher/assertions` looked suspicious and over-ceremonial.
- `crates/app/checks/catalog/runtime/src/catalog_tests` was still hanging around despite having already been superseded by `lib_tests`.
- the rule families in `app/checks/*/runtime` did not visually present as one rule per ID-shaped file, even though the logic was already one rule per file.

The requirement here was not to redesign the test architecture again, but to make the current family crates look like the intended structure while preserving behavior and not breaking all semantic imports that already referred to modules like `em_dashes`, `word_count`, or `paragraph_length`.

## Decisions Made

### Rename files and sidecars to ID-based stems but keep semantic module names
- **Chose:** rename the backing files/directories to stems like `heur_01_em_dashes.rs` and `heur_01_em_dashes_tests/`, while keeping module names such as `em_dashes` via `#[path = "..."]`.
- **Why:** this gives the tree the desired “rule by ID” shape without forcing wide import churn across tests and assertions that already use semantic module names.
- **Alternatives considered:**
  - Rename both filenames and public module names to the ID stems — rejected because it would create a lot of low-value churn in imports and make normal Rust call sites less readable.
  - Leave the semantic filenames as-is and only argue that they were “already one rule per file” — rejected because it did not satisfy the structural concern the user raised.

### Use the public/prose check ordering as the numbering source
- **Chose:** number files according to the public check lists already used in project docs and handoff material:
  - heuristics: em-dashes through jargon-faker
  - lexical: prohibited-terms through recommended-terms
  - flow: paragraph-length, word-repetition
  - readability: flesch-kincaid, gunning-fog, coleman-liau, avg-sentence-length
  - document-policy: word-count, heading-hierarchy, heading-counts, bold-density, code-fences
- **Why:** the numbering should match the product-facing order people already read, not an accidental internal `all_checks()` order that had drifted in a few crates.
- **Alternatives considered:**
  - Number by current `all_checks()` order — rejected because readability and document-policy already had an order mismatch with the public check lists.
  - Number alphabetically — rejected because it would not match either the public docs or the logical grouping/order already used elsewhere.

### Reorder `all_checks()` where it disagreed with the new numbering
- **Chose:** update readability and document-policy `all_checks()` vectors to match the new numbering order.
- **Why:** if filenames claim `doc_01_word_count` and `read_01_flesch_kincaid`, the runtime collection order should not silently disagree.
- **Alternatives considered:**
  - Keep the old runtime order and let numbering diverge — rejected because that would create a second source of truth immediately.

### Delete stale empty leftovers instead of preserving migration debris
- **Chose:** remove the empty `catalog_tests/` and stray empty `document-policy/runtime/src/sentence_case_tests/` directories.
- **Why:** they served no purpose and directly undermined the claim that the tree now has one clear structural shape.
- **Alternatives considered:**
  - Leave them because they did not affect compilation — rejected because this work was specifically about making the structure honest and legible.

### Keep the wrapper package assertions crate
- **Chose:** leave `packages/prosesmasher/assertions` in place.
- **Why:** even though it is not a rule family, it is the enforced location for reusable package-smoke assertions used by the packaged wrapper harness. The current cleanup was about `app/checks` rule layout, not about creating another exception to the already-applied package test pattern.
- **Alternatives considered:**
  - Delete the package assertions crate as over-ceremony — rejected because the package smoke tests would then be the one surviving exception to the same shared-assertions pattern being enforced elsewhere.

## Architectural Notes
- The file tree now communicates the rule ownership boundary more clearly:
  - one concrete rule file
  - one matching sidecar test directory
  - one matching assertions file
- Semantic module names remain stable (`em_dashes`, `word_count`, etc.), so the rename affects structure and discoverability far more than public API.
- `support.rs` remains in the family crates. That is still the main place where family-level ownership can drift, but this change intentionally did not try to split those helpers while doing a broad rename sweep.

## Information Sources
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md` — target structural shape being mirrored
- `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md` — original family-crate split
- `.worklogs/2026-03-25-223349-complete-rs-test-runtime-assertions-split.md` — repo-wide runtime/assertions completion pass
- `.worklogs/2026-03-26-003511-five-round-test-attack-hardening.md` — post-migration hardening baseline before this cleanup
- Current family lib/runtime/assertions files under:
  - `apps/prosesmasher/crates/app/checks/heuristics/**`
  - `apps/prosesmasher/crates/app/checks/lexical/**`
  - `apps/prosesmasher/crates/app/checks/flow/**`
  - `apps/prosesmasher/crates/app/checks/readability/**`
  - `apps/prosesmasher/crates/app/checks/document-policy/**`
- Verification commands:
  - `cargo fmt --manifest-path apps/prosesmasher/Cargo.toml --all`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - structural `find` / `rg` scans against `app/checks/**`

## Open Questions / Future Considerations
- `support.rs` in `heuristics` and `lexical` is still the main ownership-pressure point. If rule-local semantics keep accumulating there, the next cleanup should split those helpers downward by rule cluster instead of letting the family helper layer grow.
- `packages/prosesmasher/assertions` still looks heavier than the rule-family crates because it serves package smoke helpers rather than rule semantics. That is acceptable under the current enforced pattern, but it remains a consciously accepted bit of ceremony.
- `domain/types` still uses broader `lib_tests` coverage rather than one exported type per sidecar. This change did not try to make non-rule crates mimic the rule-family tree more than they already do.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/heuristics/runtime/src/lib.rs` — path-wired semantic module exports backed by ID-based rule files
- `apps/prosesmasher/crates/app/checks/heuristics/assertions/src/lib.rs` — matching assertions layout using the same semantic-module-to-ID-file split
- `apps/prosesmasher/crates/app/checks/lexical/runtime/src/lib.rs` — example where the old filename (`hedge_words`) was structurally corrected to `lex_02_hedge_stacking`
- `apps/prosesmasher/crates/app/checks/document-policy/runtime/src/lib.rs` — reordered `all_checks()` to match the new numbering
- `apps/prosesmasher/crates/app/checks/readability/runtime/src/lib.rs` — reordered readability `all_checks()` to match the new numbering
- `apps/prosesmasher/packages/prosesmasher/assertions/src/packaged_cli_smoke.rs` — package-level shared smoke assertions intentionally kept in place
- `.worklogs/2026-03-26-003511-five-round-test-attack-hardening.md` — the last full hardening baseline before this structural rename pass

## Next Steps / Continuation Plan
1. If the next architecture pass targets ownership rather than naming, start with `heuristics/runtime/src/support.rs` and `lexical/runtime/src/support.rs`; identify helpers that are effectively owned by one rule or one small rule cluster and move them downward.
2. If the GuardRails family enforcer is run against this repo, use the `app/checks/**` crates as the first verification target because they now have the clearest rule-by-ID layout.
3. If similar ID-based naming is desired outside `app/checks`, decide explicitly whether non-rule crates like `domain/types` and adapter runtimes should copy this style or keep their broader component-level naming.
