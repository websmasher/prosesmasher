# Switch presets to structure-only families

**Date:** 2026-03-22 19:52
**Scope:** `apps/prosesmasher/presets/*.json`, `apps/prosesmasher/presets/README.md`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`

## Summary
Replaced the remaining article-tier/docs/landing-page preset set with a smaller preset family that differs only by `documentPolicy`: `general`, `article`, `substack`, `email`, and `tweet`. Real presets now keep shared quality defaults everywhere and encode only structural differences.

## Context & Problem
The earlier preset cleanup still assumed that article-length and a couple of genre-specific quality deltas were the right shipped distinctions. The user clarified a stricter product rule:
- full quality checks should run everywhere
- presets should differ purely by structure: word count, heading policy, and similar document-shape constraints

That invalidated the previous preset design in two ways:
- article short/medium/long were mostly just word-count variants
- docs and landing-page were really content-model/component concerns rather than stable prose-preset families for this library

## Decisions Made

### Make real presets structure-only
- **Chose:** Strip real presets back to shared `quality: {}` and vary only `documentPolicy`.
- **Why:** This matches the product boundary the user described. The library owns prose-quality defaults; presets define the structural envelope.
- **Alternatives considered:**
  - Keep mild quality threshold deltas per preset — rejected because the user explicitly wanted shared full checks everywhere.
  - Keep lexical/style overrides for specific preset families — rejected because that reintroduces fuzzy “style personality” presets instead of structural ones.

### Replace the old preset family with real use-case families
- **Chose:** Ship:
  - `general-en.json`
  - `article-en.json`
  - `substack-en.json`
  - `email-en.json`
  - `tweet-en.json`
- **Why:** Those are real usage modes, and their meaningful differences are structural.
- **Alternatives considered:**
  - Keep article short/medium/long tiers — rejected because the only real distinction was mostly word count.
  - Keep `docs-en` and `landing-page-en` — rejected because those are better modeled elsewhere: docs as a separate editorial/content domain if needed later, landing page as component/page validation rather than one prose-body preset.

### Keep heading policy only where it makes sense
- **Chose:** Give `article` and `substack` heading rules; leave `email` and `tweet` with word-count-only policy.
- **Why:** Email and tweet are still single prose bodies here, but they are not heading-driven markdown documents.
- **Alternatives considered:**
  - Add heading policy to all presets for consistency — rejected because it would be structurally artificial for email and tweet.
  - Remove heading policy from substack — rejected because it still behaves like a document-level prose artifact more than a snippet.

## Architectural Notes
This change sharpens the product contract:
- quality is shared and library-owned
- preset differences are structural only

That lines up with both the current `prosesmasher` architecture and the earlier research:
- `validator-rust`’s `text.rs` is a generic primitive layer
- `prose.rs` is a narrow structure layer
- `writing-style-rules.md` in the generator pipeline defines shared quality expectations rather than per-channel lexical personalities

So the shipped preset surface is now much closer to the actual intended use: same prose-quality validation engine, different structural envelopes.

## Information Sources
- `apps/prosesmasher/crates/domain/types/src/config.rs` — confirms shared built-in English quality defaults
- `apps/prosesmasher/presets/README.md` — shipped preset surface
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs` — preset loader coverage
- `/Users/tartakovsky/Projects/steady-parent/packages/content-constraints/domains/text/text-quality.json` — shared prose-quality baseline
- `/Users/tartakovsky/Projects/steady-parent/apps/validator-rust/crates/app/src/check/text.rs` — generic text primitive layer
- `/Users/tartakovsky/Projects/steady-parent/apps/validator-rust/crates/app/src/check/prose.rs` — structure-focused prose layer
- `/Users/tartakovsky/Projects/steady-parent/packages/generator/pipeline/7_rewrite_articles/input/writing-style-rules.md` — shared writing-quality rules for rewrite step
- Verification runs:
  - `cargo test -q -p prosesmasher-adapters-outbound-fs`
  - `cargo test -q`
  - `cargo clippy -q --all-targets --all-features`

## Open Questions / Future Considerations
- `substack` vs `article` is still mainly a structural distinction. If those converge in practice, one of them may eventually be unnecessary.
- `email` and `tweet` currently use word-count policy because the library does not yet have char-count-based prose presets.
- If field-level copy validation is added later, `email` and `tweet` may eventually move to a more precise short-text mode.

## Key Files for Context
- `apps/prosesmasher/presets/article-en.json` — standard document-article preset
- `apps/prosesmasher/presets/substack-en.json` — longer/looser document preset
- `apps/prosesmasher/presets/email-en.json` — short body preset
- `apps/prosesmasher/presets/tweet-en.json` — very short body preset
- `apps/prosesmasher/presets/general-en.json` — defaults-only preset
- `apps/prosesmasher/presets/README.md` — explains the shipped preset philosophy
- `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs` — preset coverage and expected distinctions
- `.worklogs/2026-03-22-192945-rationalize-presets.md` — previous preset rationalization pass
- `.worklogs/2026-03-22-194123-split-preset-examples.md` — example-vs-real preset split

## Next Steps / Continuation Plan
1. Decide whether `--preset <name>` should be added now that the shipped preset family is stable and small.
2. If we add `--preset`, ensure it only selects real presets and not files under `presets/examples/`.
3. Revisit whether `substack` needs a more explicit structural distinction from `article`, or whether one unified long-form document preset would be cleaner after real-world usage.
