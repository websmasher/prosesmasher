# Public README And Corrective Hardening

**Date:** 2026-03-23 18:33
**Scope:** `README.md`, `apps/prosesmasher/packages/prosesmasher/README.md`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs`

## Summary
This pass does two things together: it prepares the repo for public GitHub visibility with a real root README and aligned crates.io README, and it hardens `negation-reframe` around reusable corrective-contrast templates instead of one-off phrase rules. The final docs were checked with `prosesmasher` itself, and the heuristic layer was verified against the local prose fixtures and the full test suite.

## Context & Problem
The repo was still private and had no root `README.md`, so making it public would have exposed a nearly blank front page. At the same time, the current `negation-reframe` work had moved beyond the last published release and needed cleanup before any public-facing push:

- README needed install, intended use, config model, command surface, output contract, and CI usage
- package README and repo README needed to stay aligned enough not to describe two different products
- recent corrective-pattern exploration had briefly overfit to a phrase-specific `begin the story` branch
- `tantrums-science.md` exposed a broader family of parallel corrective contrasts:
  - `doesn't begin X / it ends Y`
  - `I was not X / I was X`
  - `Less like X / More like Y`
  - `doesn't make X / but it makes Y`

The goal was a clean public face plus a less brittle heuristic implementation.

## Decisions Made

### Added a real root README instead of relying on the package README alone
- **Chose:** create `README.md` at repo root and keep it closely aligned with the published package README.
- **Why:** GitHub needs a root README; crates.io needs the package README. Letting them drift would create immediate documentation inconsistency.
- **Alternatives considered:**
  - Symlink or indirect include approach — rejected because GitHub/crates.io rendering is less predictable and the extra indirection is not worth it here.
  - Keep only the package README — rejected because the public repo landing page would stay weak.

### Documented the actual CLI surface from help output, not memory
- **Chose:** drive README command examples from the current `--help` output.
- **Why:** the command surface already changed across recent releases, so documentation needed to match the shipped CLI exactly.
- **Alternatives considered:**
  - Hand-written summary from memory — rejected because it would drift too easily.

### Replaced phrase-specific corrective matching with reusable parallel-contrast templates
- **Chose:** keep the narrow, high-signal corrective branches but frame them as structural templates:
  - `Less like X. More like Y.`
  - `I was not X. I was X.`
  - `doesn't make X. But it makes Y.`
  - bounded `doesn't begin/start X. It ends Y.`
- **Why:** this catches the real rhetoric family seen in `tantrums-science.md` without baking the detector into one literal phrase like `begin the story`.
- **Alternatives considered:**
  - Restore the earlier phrase-specific `story/buildup` branch — rejected because it was overfit.
  - Broaden to generic negation/contrast patterns — rejected because that would inflate false positives quickly.

### Kept lifecycle reversal bounded by noun-frame vs scheduling guards
- **Chose:** detect `doesn't begin/start X -> it ends Y` only when the negated clause is framing-like rather than time/preposition scheduling language.
- **Why:** this preserves the target hit while keeping obvious false positives such as `The meeting doesn't begin on time. It ends at five.` out of scope.
- **Alternatives considered:**
  - Match all `begin -> ends` pairs — rejected because it would immediately catch routine scheduling prose.

## Architectural Notes
The public README now presents the product as a deterministic policy engine with an opinionated heuristic layer, which matches the codebase much better than vague “quality checker” phrasing. On the detector side, `negation-reframe` remains one user-facing heuristic ID with multiple internal branches. That keeps the CLI stable while allowing the matcher to evolve around a coherent rhetorical family.

The branch ordering still matters:
- copular corrective patterns first
- then non-copular corrective templates

This keeps the evidence shapes predictable and avoids double-reporting the same pair through multiple internal branches.

## Information Sources
- CLI help output from the installed binary:
  - `prosesmasher --help`
  - `prosesmasher check --help`
  - `prosesmasher list-presets --help`
  - `prosesmasher dump-config --help`
- Local fixtures:
  - `fixtures/tantrums-science.md`
  - `fixtures/ignoring-tantrums.md`
- Existing docs and prior release notes:
  - `apps/prosesmasher/packages/prosesmasher/README.md`
  - `apps/prosesmasher/CHANGELOG.md`
  - `.worklogs/2026-03-23-174033-release-0-1-4-heuristic-tightening.md`
  - `.worklogs/2026-03-23-180152-release-0-1-5-corrective-expression-branch.md`

## Open Questions / Future Considerations
- These heuristic improvements are local at the moment; if they should ship publicly, cut another release after this push.
- The repo can be public with current docs, but package-level examples and GitHub Actions docs could still be expanded later.
- The local `fixtures/` directory remains untracked and intentionally out of git; it is still part of the validation workflow during development.

## Key Files for Context
- `README.md` — public GitHub landing page for the repo
- `apps/prosesmasher/packages/prosesmasher/README.md` — crates.io-facing package documentation
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe.rs` — corrective-rhetoric matcher family
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/negation_reframe_tests.rs` — regression coverage for the new parallel-contrast branches
- `.worklogs/2026-03-23-174033-release-0-1-4-heuristic-tightening.md` — prior heuristic release decisions
- `.worklogs/2026-03-23-180152-release-0-1-5-corrective-expression-branch.md` — previous corrective-branch release

## Next Steps / Continuation Plan
1. Commit this README and heuristic hardening pass, then push `main` to `origin`.
2. Flip `websmasher/prosesmasher` from private to public with GitHub CLI.
3. If these heuristic changes should be distributed on crates.io too, bump the crate graph and release the next patch version after the public repo push.
