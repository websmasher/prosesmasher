# Fix Dist Absolute Upload Paths

**Date:** 2026-03-24 21:15
**Scope:** `.github/workflows/dist.yml`

## Summary
This fixes the remaining `v0.1.7` release-artifact failure in the `Dist` workflow. `cargo-dist` emits absolute upload paths on CI runners, but the workflow was prepending `apps/prosesmasher/` unconditionally, which turned the archive paths invalid and caused GitHub Actions to upload only the manifest file.

## Context & Problem
The crates.io release and git tag for `0.1.7` were already live, but the GitHub release still only had `dist-manifest.json`. Investigation of the `build-local-artifacts` logs for the Linux, macOS, and Windows runners showed the same pattern:

- `dist print-upload-files-from-manifest` produced absolute archive paths such as `/home/runner/.../target/distrib/prosesmasher-x86_64-unknown-linux-gnu.tar.xz`
- the workflow transformed them into `apps/prosesmasher//home/runner/...`
- `actions/upload-artifact` therefore found only the manifest path, which was the only still-valid relative path

The earlier recursive-upload fixes in the host job were therefore downstream symptoms. The real loss happened during artifact upload from the build jobs.

## Decisions Made

### Normalize upload paths instead of blindly prefixing them
- **Chose:** add a small shell helper in both the local and global build jobs that leaves absolute paths unchanged, preserves already-prefixed repo-relative paths, and prefixes only true relative paths.
- **Why:** this matches the actual `cargo-dist` output contract across Linux, macOS, and Windows runners without assuming one path style.
- **Alternatives considered:**
  - Keep prefixing with `apps/prosesmasher/` and special-case Windows only — rejected because Linux and macOS logs showed absolute paths too.
  - Rebuild the upload list by walking `target/distrib` directly — rejected because `cargo-dist` already knows the correct release payload, and re-deriving it would be more fragile.

## Architectural Notes
The workflow still relies on `cargo-dist` as the source of truth for uploadable files. The fix only repairs path normalization at the GitHub Actions boundary. That keeps the packaging logic aligned with cargo-dist rather than duplicating artifact-selection rules in the workflow.

## Information Sources
- `.github/workflows/dist.yml`
- GitHub Actions logs from `Dist` run `23512242860`
- `.worklogs/2026-03-24-210604-fix-dist-release-file-recursion.md`
- `.worklogs/2026-03-24-205944-fix-dist-release-upload-glob.md`

## Open Questions / Future Considerations
- After rerunning `Dist`, confirm that the release now includes the expected archives, checksum files, and installer scripts.
- If the release still misses assets after this fix, inspect whether `dist host --steps=upload --steps=release` already manages the GitHub release lifecycle and whether the custom `gh release` step is redundant.

## Key Files for Context
- `.github/workflows/dist.yml` — reusable packaging and release workflow; this is where cargo-dist output paths are normalized before uploading artifacts
- `.github/workflows/manual-release.yml` — break-glass workflow used to publish `0.1.7`
- `.worklogs/2026-03-24-210604-fix-dist-release-file-recursion.md` — previous host-stage upload fix that turned out to be downstream of the real artifact-loss bug
- `.worklogs/2026-03-24-205944-fix-dist-release-upload-glob.md` — prior investigation into the host-stage GitHub release upload logic

## Next Steps / Continuation Plan
1. Commit this workflow fix and push `main`.
2. Re-run `Dist` manually for `tag=v0.1.7`.
3. Verify that each build job uploads real archives, not only `*-dist-manifest.json`.
4. Confirm the GitHub release `v0.1.7` contains the binary archives, checksum files, and installer scripts.
