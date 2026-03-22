# Rationalize shipped presets

**Date:** 2026-03-22 19:29
**Scope:** `apps/prosesmasher/presets/*.json`, `apps/prosesmasher/presets/README.md`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`

## Summary
Reduced the shipped preset taxonomy to a smaller set of real reusable prose families and rewrote the preset JSON to rely on library defaults instead of restating anti-slop lists. The final set is `general`, three article length tiers, `docs`, and `landing-page`.

## Context & Problem
The old preset set was structurally valid but semantically weak:
- several presets repeated near-identical prohibited-term and simplicity lists
- distinctions like `blog-strict`, `essay`, and `technical-article` were mostly tiny threshold nudges rather than real reusable policy families
- the presets were carrying too much of the library’s built-in quality defaults instead of just expressing the delta

The user wanted us to stop guessing and ground the design in existing content constraints from `steady-parent`. That review showed two important things:
- most real reuse is about prose family and document shape, not many subtle editorial personas
- the library itself already owns the English anti-slop defaults, so presets should mostly define shape and selective threshold changes

## Decisions Made

### Collapse fake-distinct presets into real content families
- **Chose:** Replace:
  - `blog-strict-en.json`
  - `technical-article-en.json`
  - `essay-en.json`
  with:
  - `article-short-en.json`
  - `article-medium-en.json`
  - `article-long-en.json`
- **Why:** Length tiers and heading density are real reusable article distinctions. The removed presets mostly duplicated the same lexical rules with cosmetic threshold differences.
- **Alternatives considered:**
  - Keep the old names and just simplify contents — rejected because the names still encode fake product distinctions.
  - Ship only one `article-en` preset — rejected because article length tiers were explicitly part of the user’s intended use cases and are a meaningful policy difference.

### Make `general-en` a baseline quality preset with no document policy
- **Chose:** Strip `general-en.json` down to empty `quality` and `documentPolicy` blocks so it simply means “use library defaults.”
- **Why:** A general-purpose preset should not impose arbitrary word-count or heading policy. It should be the safe starting point for prose quality alone.
- **Alternatives considered:**
  - Keep a medium-length article policy in `general-en` — rejected because that silently treats “general prose” as “article body.”
  - Remove `general-en` entirely — rejected because a baseline “defaults only” preset is still useful as a starter artifact.

### Keep presets focused on shape and threshold deltas
- **Chose:** Remove repeated prohibited-term, simplicity-pair, and repetition-ignore lists from the shipped presets unless a preset really needed a delta from the library default.
- **Why:** Those defaults already live in `default_quality_for_locale(Locale::En)`. Repeating them in every preset makes the shipped surface noisy and brittle.
- **Alternatives considered:**
  - Keep explicit lexical lists for transparency — rejected because the lists are library defaults now, not preset-specific editorial policy.
  - Move more preset-specific terms into `landing-page` or `docs` — rejected because that would start re-encoding domain/editorial policy rather than shipping reusable starting points.

### Keep docs and landing-page as the two real non-article specials
- **Chose:** Retain `docs-en` and `landing-page-en`, but simplify them to genuine deltas:
  - docs: tighter paragraphing, zero exclamations, looser readability, shape policy
  - landing-page: shorter paragraphs, stricter readability, short word-count range
- **Why:** Those are the two current families that are meaningfully different from the article tiers.
- **Alternatives considered:**
  - Merge docs into article presets — rejected because reference/docs prose really does have a different tolerance profile.
  - Rename landing-page to microcopy — rejected because the current file is still document-oriented, not field-oriented snippet validation.

## Architectural Notes
This change aligns the shipped presets with the library boundary:
- library defaults own the reusable English quality baseline
- presets mostly express `documentPolicy`
- only a few threshold deltas remain under `quality.heuristics`

That is consistent with the earlier architecture decisions:
- `quality` is the library’s main value
- `documentPolicy` is optional overlay policy
- field-level exact/regex/microcopy rules remain out of these markdown presets

The preset README was updated to explain that relationship explicitly so future edits don’t drift back toward restating defaults in every file.

## Information Sources
- `apps/prosesmasher/crates/domain/types/src/config.rs` — confirms current built-in English defaults for prohibited terms, simplicity pairs, and word repetition exclusions
- `apps/prosesmasher/presets/*.json` — previous shipped preset surface
- `/Users/tartakovsky/Projects/steady-parent/packages/content-constraints/domains/text/text-quality.json` — shared prose-quality source lists
- `/Users/tartakovsky/Projects/steady-parent/packages/content-constraints/domains/articles.json` — real article-body shape distinctions
- `/Users/tartakovsky/Projects/steady-parent/packages/content-constraints/domains/text/title-desc.json` — evidence that short-field validation is a separate family
- `/Users/tartakovsky/Projects/steady-parent/apps/validator-rust/crates/app/src/check/text.rs` — generic text primitive layer
- `/Users/tartakovsky/Projects/steady-parent/apps/validator-rust/crates/app/src/check/prose.rs` — narrow prose/document-policy layer
- Explorer findings from this session on `content-constraints` and `validator-rust`
- Verification runs:
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- If `prosesmasher` later adds field-level validation presets, those should live in a separate family from these markdown-document presets.
- `landing-page-en` may later want to split into “page section” vs “hero/microcopy” if the library gains a more fragment-oriented mode.
- If the built-in English anti-slop defaults expand materially, we may want a documented versioning note for how shipped presets inherit them.

## Key Files for Context
- `apps/prosesmasher/presets/general-en.json` — baseline defaults-only preset
- `apps/prosesmasher/presets/article-short-en.json` — short article tier
- `apps/prosesmasher/presets/article-medium-en.json` — medium article tier
- `apps/prosesmasher/presets/article-long-en.json` — long article tier
- `apps/prosesmasher/presets/docs-en.json` — docs/reference preset
- `apps/prosesmasher/presets/landing-page-en.json` — short marketing prose preset
- `apps/prosesmasher/presets/README.md` — rationale for the shipped preset surface
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs` — loader coverage for the new preset family
- `.worklogs/2026-03-22-183839-drop-config-compatibility.md` — canonical-only config boundary
- `.worklogs/2026-03-22-185813-reorganize-core-modules.md` — post-boundary module organization

## Next Steps / Continuation Plan
1. Decide whether presets should remain file-only artifacts or whether the CLI should gain `--preset <name>` support now that the set has stabilized.
2. If `--preset` is added, read and update:
   - `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`
   - `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs`
   - `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader.rs`
3. If no CLI preset support is added yet, at least expose the preset family list prominently in user-facing docs beyond `AGENTS.md`.
