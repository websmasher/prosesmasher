# HTML Parser, Quality Reclassification, And Release Plumbing

**Date:** 2026-03-24 20:26
**Scope:** `apps/prosesmasher/crates/adapters/outbound/parser`, `apps/prosesmasher/crates/app/core`, `apps/prosesmasher/crates/adapters/outbound/fs`, `apps/prosesmasher/crates/adapters/inbound/cli`, release workflow/config, internal crate metadata, `AGENTS.md`

## Summary
This batch fixes the main false-negatives/false-positives reported by users and cleans up the release path around them. Raw HTML embedded in markdown now contributes visible text instead of being dropped, the em-dash heuristic now only flags closed em dashes, `sentence-case` is reclassified as a general quality heuristic while still targeting headings, and the release setup now publishes the crate graph while creating one public `prosesmasher` tag plus GitHub release artifacts for `cargo binstall`.

## Context & Problem
Several user-reported issues were stacked together in the worktree:

- aside-heavy markdown copied from Notion was effectively invisible to the parser because `Event::Html` and `Event::InlineHtml` were ignored
- the em-dash heuristic flagged both spaced and closed em dashes even though the “LLM slop” signal the user cared about was the closed form only
- `sentence-case` was categorized as document structure even though the failure is stylistic and should participate in general quality
- internal crates were being published as implementation dependencies, but crates.io pages did not explain that they were not user-facing install targets
- the repo advertised `cargo binstall` while the release path did not actually produce GitHub release binaries/installers
- the release workflow was pointed at a nonexistent `production` branch, so the configured automation could not trigger from the actual branch topology

The user asked to stop overfitting to one content bug, make the tool preserve text instead of dropping it, clean up the preset/check taxonomy, and then sort out the release state and ship the batch.

## Decisions Made

### Preserve visible HTML text inside markdown instead of dropping all raw HTML
- **Chose:** add a dedicated HTML fragment text extractor and feed markdown `Html` / `InlineHtml` events through it.
- **Why:** the real boundary is visible prose vs non-visible/non-content markup, not markdown-native text vs tagged text.
- **Alternatives considered:**
  - Keep ignoring raw HTML and treat the file format as unsupported — rejected because user-visible prose was being lost in common real input.
  - Switch to a universal multi-format parser first — rejected because the immediate bug was text loss inside an already-supported markdown parse.

### Narrow em-dash detection to closed em dashes only
- **Chose:** flag `word—word`, allow spaced forms like `word — word`.
- **Why:** the user explicitly distinguished closed em dashes as the undesirable “slop” form while using spaced em dashes intentionally.
- **Alternatives considered:**
  - Continue flagging all em dashes — rejected because it was too blunt and produced avoidable noise.
  - Remove the heuristic entirely — rejected because the closed form is still a useful signal in this product’s style policy.

### Reclassify `sentence-case` as quality, not document policy
- **Chose:** keep the implementation scoped to heading nodes for now, but move configuration and CLI grouping under quality heuristics.
- **Why:** the check depends on structure for targeting, but the failure itself is capitalization/style, not document shape.
- **Alternatives considered:**
  - Leave it under document policy — rejected because that taxonomy is misleading.
  - Expand it immediately to all prose — rejected because broad body-text sentence-case enforcement would be noisier and was not needed for this release.

### Keep the multi-crate workspace publish model, but make the public surface explicit
- **Chose:** retain full crate-graph publishing for crates.io compatibility, while updating internal crate descriptions to say they are workspace implementation crates and that end users should install `prosesmasher`.
- **Why:** the repo wants real crate boundaries and `cargo install`; with Cargo that means the internal crates remain published dependencies.
- **Alternatives considered:**
  - Collapse the runtime into one package — rejected because that would remove crate-level compilation/dependency boundaries.
  - Fake single-crate publishing with a stub package — rejected because it would intentionally break `cargo install`.

### Split crates.io publishing from binary distribution
- **Chose:** keep `release-plz` for the crates.io crate graph, disable workspace-wide git release/tag generation, emit only the public `prosesmasher` tag, and add a separate dist-driven GitHub artifact workflow for `cargo binstall`.
- **Why:** the crate graph still needs ordered publishing, but users should see one public release/tag/install surface.
- **Alternatives considered:**
  - Continue relying on crates.io source installs alone — rejected because the README already advertises `cargo binstall`.
  - Replace `release-plz` entirely with `cargo-dist` — rejected because `cargo-dist` does not solve multi-crate crates.io publication ordering.

### Fix the release workflow to follow the actual repo branch
- **Chose:** retarget the release workflow trigger from `production` to `main`.
- **Why:** there is no `production` branch locally or on `origin`, so the previous workflow configuration could not fire.
- **Alternatives considered:**
  - Create and use a new `production` branch — rejected because the repo is currently operating on `main` and the user asked to release now.

## Architectural Notes
The parser remains markdown-first. The new HTML extraction does not attempt universal format detection; it only prevents visible HTML content inside markdown from disappearing. This keeps the current `DocumentParser` abstraction intact while solving the actual text-loss boundary.

The `sentence-case` move is architectural cleanup in the check taxonomy: structure is now used to locate targets, but not to classify the issue family. This lines up the config surface, CLI grouping, and output semantics better.

On release infrastructure, the app still has a real multi-crate hexagonal workspace. The release model is now explicitly dual:

- crates.io publication through `release-plz` for the full dependency graph
- GitHub binary distribution through `cargo-dist` for the public `prosesmasher` package only

This keeps Cargo-compatible source installs while making `cargo binstall` honest.

Trusted publishing is only partially wired from the repo side. The workflow now has `id-token: write`, but the actual switch away from `CARGO_REGISTRY_TOKEN` still depends on crates.io-side trusted publishing setup for every published workspace crate.

## Information Sources
- User-reported failure cases and policy decisions from the current session
- Parser implementation:
  - `apps/prosesmasher/crates/adapters/outbound/parser/src/markdown.rs`
  - `apps/prosesmasher/crates/adapters/outbound/parser/src/markdown_tests.rs`
- Quality/check taxonomy:
  - `apps/prosesmasher/crates/app/core/src/quality/heuristics/em_dashes.rs`
  - `apps/prosesmasher/crates/app/core/src/document_policy/sentence_case.rs`
  - `apps/prosesmasher/crates/domain/types/src/config.rs`
  - `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs`
- Release/distribution docs and behavior:
  - `release-plz` GitHub output docs
  - `cargo-binstall` README
  - local `dist` CLI help / generated reference workflow
  - Rust crates.io trusted publishing updates from 2025-07-11 and 2026-01-21
- Existing repo release state:
  - `.github/workflows/release.yml`
  - `apps/prosesmasher/release-plz.toml`
  - `apps/prosesmasher/CHANGELOG.md`
  - recent worklogs from 2026-03-23

## Open Questions / Future Considerations
- crates.io Trusted Publishing still needs to be configured on crates.io for every published workspace crate before the workflow can drop `CARGO_REGISTRY_TOKEN`
- `cargo-dist` is configured for the public package, but the actual GitHub release artifact path still needs one real CI run to prove the end-to-end upload works in GitHub Actions
- top-level multi-format document detection is still an architectural future problem; this batch only closes the markdown-with-embedded-HTML blind spot
- if `sentence-case` later expands beyond headings, it should become a separate broader capitalization policy rather than silently widening the current heading-targeted heuristic

## Key Files for Context
- `apps/prosesmasher/crates/adapters/outbound/parser/src/html_text.rs` — new HTML fragment text extraction boundary
- `apps/prosesmasher/crates/adapters/outbound/parser/src/markdown.rs` — markdown parser integration for raw HTML preservation
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/em_dashes.rs` — closed em-dash-only heuristic
- `apps/prosesmasher/crates/app/core/src/document_policy/sentence_case.rs` — implementation reused under the quality heuristic family
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_dto.rs` — config compatibility aliasing from old document-policy sentence case into the new quality location
- `.github/workflows/release.yml` — crates.io publishing flow and dist handoff
- `.github/workflows/dist.yml` — GitHub artifact build/release workflow for `prosesmasher`
- `apps/prosesmasher/dist-workspace.toml` — cargo-dist configuration
- `apps/prosesmasher/release-plz.toml` — per-package publish/tag/release strategy
- `.worklogs/2026-03-23-184140-release-0-1-6-final-cleanout.md` — last published repo-state checkpoint before this batch

## Next Steps / Continuation Plan
1. Commit this batch and push `main` so the release workflow can open a new release PR from the real default branch.
2. Watch the `Release` workflow run, confirm `release-plz` opens the expected PR, and merge that PR.
3. Let the second `Release` workflow run publish the workspace crates, emit the public `v…` tag, and invoke the reusable `dist` workflow.
4. After the release completes, verify on GitHub that only the public `prosesmasher` tag/release was created and that the release contains the expected dist assets for macOS, Linux, and Windows.
5. Separately in crates.io settings, enable Trusted Publishing for each published workspace crate and then remove `CARGO_REGISTRY_TOKEN` from the workflow.
