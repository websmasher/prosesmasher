# Add Fixture Expected Failures

**Date:** 2026-03-26 11:54
**Scope:** `fixtures/article1.mdx`, `fixtures/chatgpt_answer1.md`, `fixtures/why_do_we_dream.md`, `fixtures/*.expected.general-en.json`, `fixtures/medicaloutline/*.expected.general-en.json`

## Summary
Added per-fixture sidecar expectation files for the current `general-en` behavior and brought the three previously local root fixtures into version control so the baseline corpus is complete. Each sidecar records only the failing rule id plus short evidence substrings, which is enough to assert “we still catch the same stuff” without snapshotting the entire CLI output.

## Context & Problem
The project already had a growing set of slop-heavy fixtures, especially the new Medical Outline corpus, but there was no checked-in baseline for what the current app is expected to catch. The user wanted a regression structure that survives app changes and makes it easy to rerun the corpus and verify we still catch at least the same failures as before.

## Decisions Made

### Use per-fixture sidecar files instead of folder manifests
- **Chose:** Store expectations as files like `article1.expected.general-en.json` beside the source fixture.
- **Why:** The user explicitly preferred sidecar files over per-article folders. The sidecar shape is cleaner, easier to scan, and keeps ownership local to each fixture.
- **Alternatives considered:**
  - One manifest for the whole corpus — rejected because it centralizes unrelated expectations and becomes merge-heavy.
  - Per-article `_expected/` folders — rejected because they add directory clutter for a single JSON artifact.

### Store only `rule` and `evidenceContains`
- **Chose:** Each sidecar contains `requiredFailures`, and each failure stores only:
  - `rule`
  - `evidenceContains`
- **Why:** This is the minimum useful regression contract. It avoids brittle full-output snapshots while still pinning both the rule and the actual text evidence we expect to keep seeing.
- **Alternatives considered:**
  - Full raw JSON snapshots — rejected because they are too brittle and noisy.
  - Rule ids only — rejected because that would not prove we are catching the same concrete content.

### Use short stable evidence snippets, not full paragraphs
- **Chose:** Trim long evidence strings to readable prefixes and cap the number of stored substrings per rule.
- **Why:** Full paragraphs and MDX component blobs made the first pass unreadable. Short stable snippets are enough for `contains`-style matching.
- **Alternatives considered:**
  - Preserve complete evidence strings — rejected because the files became bulky and hard to review.

### Fall back to `observed` when a rule has no structured evidence
- **Chose:** If a failure does not emit evidence objects, derive `evidenceContains` from `observed`.
- **Why:** Some current checks, like `simplicity`, report stringy `observed` values instead of an `evidence` array. The sidecar contract still needs actual text in those cases.
- **Alternatives considered:**
  - Leave `evidenceContains` empty — rejected because it weakens the regression contract too much.

### Check in the three root fixtures alongside their expectations
- **Chose:** Add `article1.mdx`, `chatgpt_answer1.md`, and `why_do_we_dream.md` to git with their new sidecars.
- **Why:** Their expectation files would be useless in repository history without the source fixture content.
- **Alternatives considered:**
  - Commit only the sidecars — rejected because CI and future local reruns would not have the files they refer to.

## Architectural Notes
This establishes the fixture-side contract, not yet the enforcement harness. The intended next layer is a black-box regression test that:
1. runs `prosesmasher` on each fixture,
2. loads the neighboring `.expected.general-en.json`,
3. asserts each required rule still fails,
4. asserts each listed substring still appears in that rule’s evidence/output.

The current baseline under `general-en` is intentionally literal:
- `article1` currently fails `paragraph-length` and `triple-repeat`
- `chatgpt_answer1` currently fails `word-repetition`
- `why_do_we_dream` currently fails six checks
- nine of the ten Medical Outline articles currently fail only `smart-quotes`
- `is-yolanda-hadid-still-ill` currently passes completely

That narrow Medical Outline baseline is not ideal semantically; it mainly reflects that `llm-slop` rules have not been implemented yet.

## Information Sources
- Live CLI runs via `apps/prosesmasher/target/debug/prosesmasher check <fixture> --preset general-en --format json`
- `fixtures/medicaloutline/*.md` — real-world AI-written source corpus added in `.worklogs/2026-03-26-113930-add-medicaloutline-ai-fixtures.md`
- `fixtures/article1.mdx`, `fixtures/chatgpt_answer1.md`, `fixtures/why_do_we_dream.md` — local root fixtures used as part of the baseline corpus

## Open Questions / Future Considerations
- The sidecars exist, but there is not yet a committed automated regression harness that consumes them.
- When `llm-slop` rules land, most Medical Outline expectations should become much richer than `smart-quotes`.
- We may want `forbiddenFailures` later for fixtures that are meant to stay clean on specific checks.

## Key Files for Context
- `fixtures/article1.expected.general-en.json` — example sidecar for a multi-failure fixture
- `fixtures/why_do_we_dream.expected.general-en.json` — example sidecar covering several different rule families
- `fixtures/medicaloutline/what-are-the-10-worst-cancers.expected.general-en.json` — example sidecar for a real-world AI article
- `.worklogs/2026-03-26-113930-add-medicaloutline-ai-fixtures.md` — provenance for the Medical Outline corpus
- `.worklogs/2026-03-26-112504-harden-triple-repeat-synthetic-tests.md` — adjacent fixture/test-hardening work

## Next Steps / Continuation Plan
1. Add a black-box regression test that scans fixture files, finds matching `.expected.general-en.json` sidecars, runs the CLI, and enforces `rule` + `evidenceContains`.
2. Keep the enforcement at package/CLI level so the full stack is exercised, not just individual rules.
3. Once `llm-slop` checks exist, rerun the Medical Outline corpus and update those sidecars from the richer baseline.
