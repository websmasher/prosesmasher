# Disambiguate Fixture Evidence Sidecars

**Date:** 2026-03-26 12:20
**Scope:** `fixtures/*.expected.general-en.json`, `fixtures/medicaloutline/*.expected.general-en.json`

## Summary
Replaced the ambiguous `evidenceContains` arrays in the fixture sidecars with field-aware expectation keys. Sidecars now say which output field each required substring belongs to, and use `observedContains` only for failures that do not emit structured evidence objects.

## Context & Problem
After adding `llm-disclaimer`, the per-fixture sidecars exposed an ambiguity: a two-item `evidenceContains` array could mean either one match plus its sentence, or two separate matches. The user pointed this out on `what-is-end-stage-bipolar.expected.general-en.json`, where the old shape did not make it obvious that the file had exactly one disclaimer hit with two different asserted fields.

## Decisions Made

### Replace `evidenceContains` with field-aware keys
- **Chose:** Convert each rule entry to one of:
  - `evidenceContainsByField`
  - `observedContains`
- **Why:** This makes the expectation contract explicit and removes the repeat-vs-field ambiguity entirely.
- **Alternatives considered:**
  - Keep `evidenceContains` and rely on comments or convention — rejected because the JSON remains ambiguous to both humans and tooling.
  - Store the entire raw failure payload — rejected because that would turn the sidecars into brittle snapshots instead of compact regression contracts.

### Preserve the existing substrings instead of regenerating a new baseline
- **Chose:** Reassign the already checked-in substrings to concrete fields using the actual current CLI output for each fixture.
- **Why:** The baseline intent was already good; the problem was only the shape. Reusing the same snippets keeps churn low and preserves review history.
- **Alternatives considered:**
  - Regenerate all sidecars from scratch — rejected because it would create unnecessary content churn and make the shape migration harder to review.

### Use `observedContains` only when structured evidence does not exist
- **Chose:** Keep `simplicity` as `observedContains`, while mapping evidence-rich rules like `triple-repeat`, `smart-quotes`, `word-repetition`, and `llm-disclaimer` into `evidenceContainsByField`.
- **Why:** This mirrors the actual result schema instead of inventing fake evidence fields for checks that do not emit them.
- **Alternatives considered:**
  - Force everything into one `evidenceContainsByField` bucket — rejected because some checks only expose meaningful strings through `observed`.

## Architectural Notes
The new sidecar contract is now:
- `rule`
- `evidenceContainsByField` for structured evidence
- `observedContains` for fallback string expectations

Examples:
- `llm-disclaimer` uses:
  - `matched_text`
  - `sentence`
- `triple-repeat` uses:
  - `matched_text`
  - `sentence_1`
  - `sentence_2`
  - `sentence_3`
- `word-repetition` uses:
  - `word`
  - `paragraph_text`
- `simplicity` uses:
  - `observedContains`

This is a data-shape migration only. The regression harness still needs to be built later to consume the new keys.

## Information Sources
- `fixtures/medicaloutline/what-is-end-stage-bipolar.expected.general-en.json` — ambiguity case that prompted the change
- Current CLI JSON output from `apps/prosesmasher/target/debug/prosesmasher check <fixture> --preset general-en --format json`
- `.worklogs/2026-03-26-115433-add-fixture-expected-failures.md` — original sidecar-baseline design
- `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md` — immediate preceding rule addition that surfaced the ambiguity clearly

## Open Questions / Future Considerations
- The future black-box fixture regression harness needs to read `evidenceContainsByField` and `observedContains` instead of the old single array.
- If a future check emits nested evidence or non-string values worth pinning, the sidecar schema may need one more extension, but the current flat field mapping is enough for all present fixtures.

## Key Files for Context
- `fixtures/medicaloutline/what-is-end-stage-bipolar.expected.general-en.json` — clear example of the new field-aware structure
- `fixtures/why_do_we_dream.expected.general-en.json` — example spanning evidence-backed and observed-only rules
- `fixtures/article1.expected.general-en.json` — example of multi-sentence evidence fields for `triple-repeat`
- `.worklogs/2026-03-26-115433-add-fixture-expected-failures.md` — original sidecar-format rationale
- `.worklogs/2026-03-26-121258-add-llm-disclaimer-rule.md` — latest corpus update before this schema cleanup

## Next Steps / Continuation Plan
1. Update the planned black-box regression harness to read `evidenceContainsByField` and `observedContains`.
2. When new fixtures are added, write sidecars directly in the field-aware format instead of using the old ambiguous array shape.
3. Keep using the live CLI output as the source of truth when assigning substrings to fields so the sidecar schema stays aligned with the real result payloads.
