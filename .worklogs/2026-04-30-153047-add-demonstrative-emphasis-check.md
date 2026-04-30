# Add demonstrative-emphasis check

## Summary

New cadence check that flags clusters of short demonstrative-subject emphatic sentences ("That is X.", "This is where Y.", "That difference matters.", "The hard part is the judgment call."). Density-based via `max_per_document` threshold (default 2) — single instances pass, clusters fail.

## Decisions

The check identifies six sub-shapes, all variants of the demonstrative-emphasis register:

| pattern_kind | Example |
|---|---|
| `demonstrative-copular` | "That is not subtle." |
| `demonstrative-relative` | "That is where the map earns its keep." |
| `demonstrative-perception` | "That looks normal." |
| `demonstrative-emphatic-verb` | "That difference matters." |
| `demonstrative-np-copular` | "That advice is not wrong." (with `-negation` and `-relative` sub-variants) |
| `definite-np-copular` | "The hard part is the judgment call." |

Sentence cap: 12 words. Below 3 words: skip. Lists and code blocks ignored, paragraphs and blockquotes counted.

Threshold default: `max_per_document = 2`. Hits up to 2 are tolerated; 3+ fails the check. Calibrated against the user's 7-example sequence (clearly slop) and against single-instance cases in legitimate writing (clearly fine).

Placement: cadence-patterns crate, alongside fragment-stacking and triple-repeat. Conceptually a register/cadence pattern, not an LLM-vocabulary one.

Alternatives rejected: per-instance flagging (would crush legitimate writing — every "That is interesting." would fire); sliding-window detection (over-engineered for this case — document-level density is the real signal).

## Audit

Ran on 202-file fixture corpus with default threshold. **19 documents fail** (9% of corpus, all AI-generated articles), 80 individual evidence items. Sample hits all read as clear AI slop:

- "That list is not random." (burnout-at-work.md)
- "Those are not cures." (burnout-at-work.md)
- "That distinction matters." (does-social-media-harm-attention-span.md)
- "This is where people usually overreach." (does-social-media-harm-attention-span.md)
- "That is not a character flaw." (how-chronic-stress-affects-the-body.md)
- "That gap matters." (why-children-have-tantrums.md)

Pattern-kind distribution: demonstrative-copular dominates (33), then definite-np-copular (15) and demonstrative-np-copular (13). Perception-verb and emphatic-verb sub-shapes fire rarely (3 each) — tight enough that they don't drift.

Existing FP-protection tests across the workspace all still pass (86 cadence tests, 212 fs adapter tests, etc.).

## Key files

- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_08_demonstrative_emphasis.rs` — check implementation, 6 classifier functions
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_08_demonstrative_emphasis_tests/synthetic.rs` — 8 tests (cluster, threshold, length, list/code, non-demonstrative, disabled)
- `apps/prosesmasher/crates/app/checks/cadence-patterns/assertions/src/heur_08_demonstrative_emphasis.rs` — assertions wiring
- `apps/prosesmasher/crates/domain/types/runtime/src/config.rs` — `demonstrative_emphasis: AccumulativeCheck` field on `HeuristicsConfig`
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs` — JSON DTO field + apply logic
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs` — count assertions bumped (45→46, 40→41, 29→30)

## Next steps

- Existing JSON config presets don't include the new check, so it runs at default settings everywhere. If the threshold turns out to need tuning per genre (e.g. tweets vs articles), add per-preset overrides.
- The check fixed the "demonstrative emphasis" gap but related families remain: stand-alone "the [N] is the [N]" emphatics outside a cluster ("The hard part is the judgment call." in isolation), pronoun-only emphatics ("It is not subtle."). Defer until corpus signal demands them.
