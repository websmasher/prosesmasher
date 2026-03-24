# Fix Dist Release File Recursion

**Date:** 2026-03-24 21:06
**Scope:** `.github/workflows/dist.yml`

## Summary
This corrects the release upload filter one more time after the previous dist rerun succeeded technically but only attached `dist-manifest.json` to the GitHub release. The real binary archives and installer files live in nested artifact paths, so the upload step now recurses through files while excluding manifest files explicitly.

## Context & Problem
After the upload-glob fix, the rerun `Dist` workflow completed and created the GitHub release for `v0.1.7`. However, inspection of the release showed only one uploaded asset:

- `dist-manifest.json`

That proved the previous file-selection logic was still too narrow:

- `find ../../artifacts -maxdepth 1 -type f`

The artifacts downloaded by the workflow are not all at the top level. The real archives and checksum files are nested under subdirectories, so the top-level-only search missed them and found only the top-level manifest file.

## Decisions Made

### Recurse through artifact files and exclude manifest files by name
- **Chose:** collect release files with recursive `find ../../artifacts -type f` while excluding `*-dist-manifest.json` and `dist-manifest.json`.
- **Why:** this matches the actual artifact layout while still avoiding the bookkeeping JSON manifests.
- **Alternatives considered:**
  - Keep relying on top-level artifact flattening — rejected because the observed release already proved the layout is nested in practice.
  - Parse the cargo-dist manifest to reconstruct the file list exactly — rejected for now because recursive file selection with manifest exclusions is sufficient and simpler to maintain.

## Architectural Notes
The release step now treats the artifact staging directory as a tree, not a flat folder. That aligns the custom `gh release` upload logic with how GitHub artifact downloads are actually materialized by `actions/download-artifact`.

## Information Sources
- GitHub release inspection for `v0.1.7`
- `.github/workflows/dist.yml`
- `.worklogs/2026-03-24-205944-fix-dist-release-upload-glob.md`

## Open Questions / Future Considerations
- After rerunning `Dist`, verify that the GitHub release includes the expected archives, checksums, and installer scripts, not just the release notes and manifest.
- If future cargo-dist changes the artifact layout again, parsing the generated manifest may become the safer long-term upload source.

## Key Files for Context
- `.github/workflows/dist.yml` — release upload logic
- `.worklogs/2026-03-24-205944-fix-dist-release-upload-glob.md` — prior upload-filter fix that removed directory entries but still missed nested files
- `https://github.com/websmasher/prosesmasher/releases/tag/v0.1.7` — live release to verify after rerun

## Next Steps / Continuation Plan
1. Commit this recursive file-selection fix and push `main`.
2. Re-run `Dist` for `tag=v0.1.7`.
3. Verify the release assets list now includes the actual archives, checksums, and installer scripts.
4. Confirm the worktree remains clean and the release surface is complete.
