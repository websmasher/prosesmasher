# Fix Dist Release Upload Glob

**Date:** 2026-03-24 20:59
**Scope:** `.github/workflows/dist.yml`

## Summary
This fixes the final `v0.1.7` dist failure in the host job. The workflow had already built every artifact successfully, but the GitHub release upload step tried to pass a directory to `gh release create` because the glob `../../artifacts/*` matched the residual `target/` directory.

## Context & Problem
After the bootstrap-order fix, the rerun `Dist` workflow (`23511826599`) successfully completed:

- all four local artifact builds
- the global artifact build
- `dist host --steps=upload --steps=release`

The only remaining failure was the custom GitHub release step, which ended with:

- `Post "https://uploads.github.com/.../assets?...": read ../../artifacts/target: is a directory`

The workflow intentionally downloads all artifacts into `../../artifacts`, and even after the cleanup step removes `*-dist-manifest.json`, the directory still contains a nested `target/` directory. The shell glob `../../artifacts/*` therefore expands to both files and that directory, and `gh release create/upload` refuses the directory entry.

## Decisions Made

### Upload only files from the artifact staging directory
- **Chose:** gather release payloads with `find ../../artifacts -maxdepth 1 -type f | sort` and pass only those files to `gh release create/upload`.
- **Why:** this is the narrowest fix that matches the actual contract of the step: upload built release files, not arbitrary directory entries.
- **Alternatives considered:**
  - Add another cleanup rule for the `target/` directory — rejected because filtering to files is safer and does not depend on knowing every directory name cargo-dist might use.
  - Replace the custom GitHub release step entirely with a different uploader — rejected because the current step already has the correct release-title/notes logic; only the file selection was wrong.

## Architectural Notes
The important change is that the workflow now treats the artifact staging area as a mixed directory and explicitly filters it before calling `gh`. That is a more stable contract than assuming every entry under `../../artifacts` is uploadable.

## Information Sources
- Failed `host` job log from `Dist` run `23511826599`
- `.github/workflows/dist.yml`
- `.worklogs/2026-03-24-205530-fix-dist-bootstrap-order.md`

## Open Questions / Future Considerations
- After rerunning `Dist`, verify that the GitHub release page shows all expected archives, checksums, and installer scripts.
- The release currently exists as a git tag on GitHub; this fix should convert that into the full binary release surface without republishing crates.

## Key Files for Context
- `.github/workflows/dist.yml` — artifact build and GitHub release creation logic
- `.worklogs/2026-03-24-205530-fix-dist-bootstrap-order.md` — previous dist workflow fix that got the matrix builds running
- `.github/workflows/manual-release.yml` — workflow that published `v0.1.7` and created the release tag

## Next Steps / Continuation Plan
1. Commit this upload-filter fix and push `main`.
2. Re-run `Dist` with `tag=v0.1.7`.
3. Confirm the host job completes and the GitHub release appears with all uploaded artifacts.
4. Verify the public install surfaces (`gh release`, `cargo search`) match the released `0.1.7` state.
