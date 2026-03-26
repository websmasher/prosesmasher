# Tighten Softening Language Boundary

**Date:** 2026-03-26 14:15
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language.rs`, `fixtures/why_do_we_dream.expected.general-en.json`, `apps/prosesmasher/Cargo.toml`, `apps/prosesmasher/Cargo.lock`, `apps/prosesmasher/**/Cargo.toml`, `apps/prosesmasher/CHANGELOG.md`

## Summary
Tightened `softening-language` so a weaker `may + often` sentence no longer trips the rule, while keeping the strong Medical Outline hits intact. Released that boundary adjustment as `0.2.2`.

## Context & Problem
After adding `softening-language`, the rule caught four real fixtures. Three Medical Outline hits looked clearly right. The fourth, in `why_do_we_dream.md`, was borderline: the sentence was soft and vague, but not in the same obviously canned medical-disclaimer register as the corpus the rule was meant to target.

The user asked to tighten it “just a little bit,” which meant:
- preserve most of the true positives
- drop the weakest false-positive-ish hit
- avoid redesigning the rule from scratch

## Decisions Made

### Remove `often` from the qualifier branch
- **Chose:** stop treating `often` as one of the strong qualifier tokens for `softening-language`.
- **Why:** `may + often` was the specific combination creating the weakest hit in `why_do_we_dream`. In this rule, `often` was doing too much work compared with stronger qualifiers like `commonly`, `typically`, or `potentially`.
- **Alternatives considered:**
  - Raise the whole rule threshold — rejected because it would weaken the stronger Medical Outline detections too.
  - Add a special-case exclusion for `why_do_we_dream`-style narration — rejected because that would be corpus-specific and ugly.
  - Remove the entire modal+qualifier branch — rejected because it would throw away legitimate hits like `may + potentially`.

### Intentionally drop the `why_do_we_dream` sidecar expectation
- **Chose:** remove `softening-language` from `fixtures/why_do_we_dream.expected.general-en.json`.
- **Why:** after tightening, the file no longer fails this rule. That is the desired outcome, not an accidental regression.
- **Alternatives considered:**
  - Keep the sidecar and force the rule to still hit — rejected because the whole point of the change was to stop treating that sentence as strong enough evidence.

### Bump the workspace to `0.2.2`
- **Chose:** ship the boundary adjustment as `0.2.2`.
- **Why:** the user asked for semantic version bumps on each build/release step. This is a backward-compatible patch-level tuning change after `0.2.1`.
- **Alternatives considered:**
  - Leave the version at `0.2.1` — rejected because it violates the release/versioning rule now in force.

### Repair the lockfile surgically
- **Chose:** restore `Cargo.lock` from the previous good state and only rewrite the `prosesmasher*` package entries to `0.2.2`.
- **Why:** a naive global version-string replacement had also mutated unrelated third-party `0.2.x` packages such as `libm` and `windows-link`, which broke resolution against the currently available registry index.
- **Alternatives considered:**
  - Let `cargo update` regenerate the lockfile — rejected because the index constraints here made that path unstable and slower than a controlled repair.
  - Hand-edit only the failing third-party packages — rejected because it would leave the lockfile history harder to reason about than simply restoring the last known-good base.

## Architectural Notes
This is a good example of why per-fixture sidecars are useful: they let us distinguish a deliberate boundary tightening from an accidental regression. The rule changed, one expected failure disappeared on purpose, and the other stronger real-world hits stayed.

The lockfile incident also exposed a release-process footgun: global textual version replacement is safe for manifests, but not for `Cargo.lock`. Future version bumps should continue using selective lockfile rewriting for `prosesmasher*` entries only.

## Information Sources
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language.rs` — matcher logic and qualifier list.
- `fixtures/medicaloutline/what-cancers-cannot-be-cured.md`
- `fixtures/medicaloutline/what-food-is-not-good-for-eczema.md`
- `fixtures/medicaloutline/what-food-should-psoriasis-patients-avoid.md`
- `fixtures/why_do_we_dream.md`
- `fixtures/why_do_we_dream.expected.general-en.json`
- `.worklogs/2026-03-26-140309-add-softening-language-rule.md`

## Open Questions / Future Considerations
- `softening-language` may still deserve one or two more careful boundary passes after more non-medical corpora are added.
- The release/versioning workflow should probably get a tiny helper script to avoid repeating manual workspace+lockfile updates.
- Catalog test names still lag the asserted counts and should be cleaned up separately.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_06_softening_language.rs` — the tightened matcher.
- `fixtures/why_do_we_dream.expected.general-en.json` — intentional dropped baseline.
- `apps/prosesmasher/Cargo.toml` — workspace version authority, now `0.2.2`.
- `apps/prosesmasher/Cargo.lock` — selectively repaired lockfile with only `prosesmasher*` entries bumped.
- `apps/prosesmasher/CHANGELOG.md` — `0.2.2` release note.
- `.worklogs/2026-03-26-140309-add-softening-language-rule.md` — original rule introduction and rationale.

## Next Steps / Continuation Plan
1. Continue with `universalizing-claims` as the next `llm-slop` rule.
2. Add an automated sidecar regression harness so intentional expectation drops are explicit in test output rather than manual review only.
3. If another boundary tweak is needed, verify it against both Medical Outline and non-medical fixtures before recording new sidecars.
