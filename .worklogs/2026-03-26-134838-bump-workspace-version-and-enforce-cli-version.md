# Bump Workspace Version And Enforce CLI Version

**Date:** 2026-03-26 13:48
**Scope:** `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/CHANGELOG.md`, `apps/prosesmasher/**/Cargo.toml`, `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs`

## Summary
Bumped the entire `prosesmasher` workspace from `0.1.7` to `0.2.0` and added packaged smoke coverage for `prosesmasher --version`. The CLI already exposed clap's version flag; this change makes it an explicit release contract and keeps the workspace metadata aligned for future publishes.

## Context & Problem
The repo had grown materially since `0.1.7`: new `llm-slop` rules landed, the check families were restructured, and the test architecture was refactored around runtime/assertions crates and fixture sidecars. The user wanted versioning treated as a first-class release concern instead of an afterthought, and specifically wanted the CLI to expose the current library version through `--version`.

The immediate problems were:
- the entire workspace was still published as `0.1.7`
- internal path dependency pins still referenced `0.1.7`
- `clap` exposed `--version`, but nothing in tests enforced that behavior
- `CHANGELOG.md` did not yet reflect the new release scope

## Decisions Made

### Bump to `0.2.0`
- **Chose:** move the workspace from `0.1.7` to `0.2.0`.
- **Why:** this is more than a patch-level fix. The public surface has grown with a new rule family, more shipped checks, stronger fixture regressions, and a major internal structure split. Treating that as a patch would understate the release.
- **Alternatives considered:**
  - `0.1.8` — rejected because the accumulated feature surface since `0.1.7` is too large for a “tiny patch” story.
  - leaving version untouched until publish time — rejected because it guarantees drift between code, lockfile, and actual release intent.

### Keep clap's built-in version flag and enforce it with smoke tests
- **Chose:** keep the existing clap `version` wiring in the CLI and add a packaged smoke test for `prosesmasher --version`.
- **Why:** the behavior already existed; what was missing was a durable contract. A smoke test is enough to make it part of the release surface.
- **Alternatives considered:**
  - add a custom `version` subcommand — rejected because `--version` is the normal CLI shape and clap already provides it.
  - leave it untested — rejected because the user explicitly asked for it and packaging changes can regress wrapper behavior.

### Update every manifest and the lockfile together
- **Chose:** bump the workspace version, every internal pinned path dependency version, and the lockfile in one release change.
- **Why:** this workspace uses versioned internal path dependencies for publishable crates, so partial updates leave the package graph inconsistent.
- **Alternatives considered:**
  - only changing `[workspace.package]` — rejected because the internal crate manifests would still advertise the old version.
  - removing pinned internal versions entirely — rejected for now because that is a separate packaging policy change, not part of this task.

## Architectural Notes
The actual version source remains `apps/prosesmasher/Cargo.toml` under `[workspace.package]`, but the workspace still duplicates that number in internal path dependency manifests because these crates are intended to stay publishable. That duplication is annoying, but it is currently part of the release plumbing.

The CLI version contract is now enforced at the package layer in `packages/prosesmasher/tests/packaged_cli_smoke.rs`, which is the right boundary: it verifies the user-facing wrapper binary, not an internal library crate in isolation.

## Information Sources
- `apps/prosesmasher/Cargo.toml` — workspace version source.
- `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs` — package-level smoke coverage for user-facing CLI behavior.
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/args.rs` — clap version wiring already in place.
- `apps/prosesmasher/CHANGELOG.md` — release history and public-facing release notes.
- Recent worklogs describing the feature surface that justified the bump:
  - `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md`
  - `.worklogs/2026-03-26-124105-add-response-wrapper-rule.md`
  - `.worklogs/2026-03-26-130445-add-generic-signposting-rule.md`
  - `.worklogs/2026-03-26-132155-add-boilerplate-framing-rule.md`
  - `.worklogs/2026-03-26-134227-add-llm-vocabulary-rule.md`

## Open Questions / Future Considerations
- If version bumps are going to happen this frequently, the internal path dependency pinning may be worth simplifying later, but that should be handled as an explicit packaging decision.
- The release process still depends on humans deciding patch/minor/major. If that becomes repetitive, add a documented release policy note or helper script rather than guessing each time.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — workspace version authority and member graph.
- `apps/prosesmasher/CHANGELOG.md` — public release history.
- `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs` — enforced `--version` contract.
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/src/args.rs` — clap command metadata, including the version flag.
- `.worklogs/2026-03-26-134227-add-llm-vocabulary-rule.md` — immediate predecessor release-scope worklog.

## Next Steps / Continuation Plan
1. Keep bumping the workspace version as part of each release-worthy change, not as an afterthought after several unrelated commits.
2. When the next public-facing check lands, decide the bump level first, then update the workspace and manifests before or alongside the feature commit.
3. If internal version pinning keeps causing bulk-churn, evaluate whether some manifests can safely stop duplicating version fields without hurting publishability.
