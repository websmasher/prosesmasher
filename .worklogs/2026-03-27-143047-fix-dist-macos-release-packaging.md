# Fix Dist MacOS Release Packaging

**Date:** 2026-03-27 14:30
**Scope:** `apps/prosesmasher/Cargo.toml`, `Cargo.toml`, `Cargo.lock`, `apps/prosesmasher/Cargo.lock`, workspace crate manifests under `apps/prosesmasher/**/Cargo.toml`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Fixed the failed `v0.3.5` GitHub release by disabling LTO in the `dist` profile and bumping the project to `0.3.6`. Verified the exact failing packaging path locally with a real `x86_64-apple-darwin` dist-profile build, then re-ran the full workspace test suite.

## Context & Problem
After setting up the new GitHub-release-based install surface, the user asked whether the public command actually worked:

```bash
cargo binstall --git https://github.com/websmasher/prosesmasher prosesmasher
```

It did not. The `v0.3.5` release workflow failed before publishing any release artifacts, so `cargo-binstall` had nothing to download.

Inspecting the failed GitHub Actions logs showed the concrete cause:
- `cargo-dist` was building the `x86_64-apple-darwin` artifact on an Apple Silicon runner
- the `dist` profile used `lto = "thin"`
- the linker hit an LLVM bitcode compatibility error (`Unknown attribute kind (102/103)`) while cross-linking the Intel macOS binary

So the public install path was fine in concept, but the packaging profile was preventing the release from ever becoming real.

## Decisions Made

### Disable LTO In The Dist Profile
- **Chose:** Change `[profile.dist]` in `apps/prosesmasher/Cargo.toml` from `lto = "thin"` to `lto = false`.
- **Why:** The failure was in link-time optimization during cross-target macOS packaging, not in ordinary debug/test builds. Disabling LTO in the dist profile removes the bitcode-cross-link path entirely and is the smallest targeted fix.
- **Alternatives considered:**
  - Keep thin LTO and try to pin a different linker/toolchain in CI — rejected because it adds CI-specific complexity around Apple LLVM compatibility for a packaging-only path.
  - Drop the `x86_64-apple-darwin` target from the release matrix — rejected because that would reduce the published platform support instead of fixing the packaging issue.
  - Move dist builds to separate native runners only — rejected because the existing cargo-dist matrix already intends to build this target and the simpler profile fix solved it locally.

### Ship The Packaging Fix As `0.3.6`
- **Chose:** Bump from `0.3.5` to `0.3.6` rather than trying to reuse the failed tag.
- **Why:** The failed tag already exists remotely and is tied to a broken release run. A clean patch release is easier to reason about and keeps the public version history honest.
- **Alternatives considered:**
  - Force-move the `v0.3.5` tag — rejected because rewriting a public tag after it has already triggered a workflow is a worse operational story than shipping a clear patch fix.
  - Leave version numbers unchanged and only patch CI — rejected because the user explicitly asked whether the public install worked, and the answer changed in a way that deserves a new release boundary.

### Validate The Actual Failure Mode Locally
- **Chose:** Install the missing `x86_64-apple-darwin` target locally and run:
  - `cargo build --manifest-path apps/prosesmasher/Cargo.toml -p prosesmasher --profile dist --target x86_64-apple-darwin`
- **Why:** The bug was specifically in the dist profile and the Intel macOS target. Just running ordinary workspace tests would not have proven the fix.
- **Alternatives considered:**
  - Trust the CI log and patch blind — rejected because a local reproduction/proof for the exact packaging path was available and much stronger.
  - Only run `cargo dist build --artifacts=lies` again — rejected because that fakes the artifact build and would not exercise the real linker path.

## Architectural Notes
- The public install model remains unchanged from `0.3.5`:
  - root shim package for `--git` installs
  - GitHub Releases as the binary host
  - tag-driven `cargo-dist` workflow
- The only behavioral change in this patch is the dist build profile. Runtime code and rule behavior are unaffected.
- This is intentionally a release-engineering patch, not another product feature release.

## Information Sources
- Failed workflow logs from:
  - GitHub Actions run `23650719475`
- Local verification commands:
  - `gh run view 23650719475 --log-failed`
  - `rustup target add x86_64-apple-darwin`
  - `cargo build --manifest-path apps/prosesmasher/Cargo.toml -p prosesmasher --profile dist --target x86_64-apple-darwin`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - `cargo run --manifest-path apps/prosesmasher/Cargo.toml -q -p prosesmasher -- --version`
  - `cargo run --manifest-path Cargo.toml -q -- --version`
- Prior release-surface worklog:
  - `.worklogs/2026-03-27-141640-setup-binstall-github-release-surface.md`

## Open Questions / Future Considerations
- If binary size or startup characteristics later make LTO desirable again, revisit it deliberately with a matrix-aware setup instead of re-enabling it blindly in `profile.dist`.
- `v0.3.5` remains as a failed release tag in GitHub Actions history. Operationally that is acceptable, but if release hygiene matters later we may want a short maintainer note documenting that `0.3.6` supersedes a broken packaging-only release.

## Key Files for Context
- `apps/prosesmasher/Cargo.toml` — `profile.dist` configuration and workspace version
- `apps/prosesmasher/CHANGELOG.md` — `0.3.6` packaging-fix release note
- `Cargo.toml` — root shim package version used by the public git/binstall entrypoint
- `.github/workflows/release.yml` — tag-driven release workflow introduced in the prior pass
- `.github/workflows/dist.yml` — reusable dist pipeline still used for all release artifacts
- `.worklogs/2026-03-27-141640-setup-binstall-github-release-surface.md` — prior worklog explaining the new GitHub-release install surface

## Next Steps / Continuation Plan
1. Commit and push the `0.3.6` packaging fix, then push tag `v0.3.6`.
2. Wait for the `Release` GitHub Actions workflow to finish successfully.
3. Run the exact public install command against the live release into a temporary root:
   - `cargo binstall --git https://github.com/websmasher/prosesmasher prosesmasher --root <tmpdir>`
4. Verify the installed binary with:
   - `<tmpdir>/bin/prosesmasher --version`
5. If the live install still fails after a successful release, inspect the release asset names versus the root `package.metadata.binstall` resolution patterns before changing anything else.
