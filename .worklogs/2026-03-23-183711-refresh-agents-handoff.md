# Refresh Agents Handoff

**Date:** 2026-03-23 18:37
**Scope:** `AGENTS.md`

## Summary
This updates `AGENTS.md` to match the current state of the repo and package. It removes stale future-work items, adds the shipped `fragment-stacking` heuristic to the handoff doc, and fixes two em dashes so the handoff file passes `prosesmasher` itself.

## Context & Problem
The repo handoff document had drifted behind the actual product:

- it still described `prosesmasher` as not yet published
- it still listed `prosesmasher init` as a future step even though that direction was intentionally rejected earlier
- it did not mention the now-shipped `fragment-stacking` heuristic
- it contained literal em dashes, which caused the file to fail the validator when checked under `general-en`

Since the repo is now public, the handoff file should not quietly lag behind the shipped state.

## Decisions Made

### Updated only the clearly stale sections
- **Chose:** patch current status, heuristic count/listing, and next-step bullets instead of rewriting the whole handoff.
- **Why:** the document is long and mostly accurate; only a few lines were provably stale.
- **Alternatives considered:**
  - Full handoff rewrite — rejected because it would create unnecessary churn.
  - Leave it untouched — rejected because the stale lines were now misleading.

### Made the handoff doc pass the tool itself
- **Chose:** remove the two literal em dashes instead of exempting the file.
- **Why:** the repo instructions should pass the repo’s own default linting rules.
- **Alternatives considered:**
  - Ignore the failure as documentation-only noise — rejected because the user explicitly wanted the docs run through the tool.

## Architectural Notes
No runtime behavior changed here. This is a handoff accuracy pass only. The main value is that future sessions can trust `AGENTS.md` for the high-level product state without tripping over already-resolved historical notes.

## Information Sources
- `AGENTS.md`
- `prosesmasher check --list-checks --format json`
- current repo/package state:
  - public GitHub repo
  - published crates.io package
- local validator runs on:
  - `README.md`
  - `apps/prosesmasher/packages/prosesmasher/README.md`
  - `AGENTS.md`

## Open Questions / Future Considerations
- There is an unrelated tracked deletion of `CLAUDE.md` in the worktree that was intentionally not included in this change.
- Future handoff edits should continue to keep the doc aligned with the current release state, especially check counts and shipped commands.

## Key Files for Context
- `AGENTS.md` — repo handoff and architecture summary
- `.worklogs/2026-03-23-183336-public-readme-and-corrective-hardening.md` — prior public-facing documentation pass
- `README.md` — current public GitHub-facing product description
- `apps/prosesmasher/packages/prosesmasher/README.md` — current crates.io-facing package description

## Next Steps / Continuation Plan
1. Commit only `AGENTS.md` and this worklog, leaving unrelated worktree changes untouched.
2. Push the handoff refresh to `main`.
3. If additional handoff drift appears later, update only the stale sections instead of reworking the whole document.
