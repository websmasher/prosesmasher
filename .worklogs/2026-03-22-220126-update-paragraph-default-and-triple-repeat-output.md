# Update Paragraph Default And Triple Repeat Output

**Date:** 2026-03-22 22:01
**Scope:** `AGENTS.md`, `apps/prosesmasher/crates/domain/types/src/config.rs`, `apps/prosesmasher/crates/domain/types/src/lib_tests.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/triple_repeat.rs`, `apps/prosesmasher/crates/app/core/src/quality/heuristics/triple_repeat_tests.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/src/config_loader_tests.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/tests/fixtures/sample-config.json`, `apps/prosesmasher/presets/full-config-en.json`

## Summary
Raised the default paragraph-length threshold from 4 to 6 sentences and normalized triple-repeat evidence to ordered `sentence_1`, `sentence_2`, `sentence_3` fields. Updated tests and shipped config examples so the runtime, examples, and handoff docs all describe the same defaults.

## Context & Problem
During real article runs, the user decided the default paragraph-length limit of 4 was too strict and asked to raise it to 6. In the same review, the current triple-repeat evidence shape was called out as confusing because it emitted `sentence`, `next_sentence`, and `third_sentence`, which is harder to scan and slightly out of order semantically.

The tool had already started serving as a rewrite-loop contract, so these defaults and evidence fields need to read cleanly. If the runtime default changes but the full-config example and handoff doc stay at 4, the tool becomes harder to trust.

## Decisions Made

### Raise paragraph-length default to 6
- **Chose:** Change `ParagraphLengthConfig::default()` to `max_sentences: 6`.
- **Why:** The user explicitly wants a looser default, and real article output showed 4 sentences was generating more pressure than intended.
- **Alternatives considered:**
  - Leave the default at 4 and rely on presets/config overrides — rejected because the request was specifically to change the default behavior.
  - Raise only the article preset and leave the global default alone — rejected because the default quality layer should stay shared across presets.

### Make triple-repeat evidence explicitly ordered
- **Chose:** Replace `sentence`, `next_sentence`, `third_sentence` with `sentence_1`, `sentence_2`, `sentence_3`.
- **Why:** The ordered names are clearer in JSON and match how a human reads the evidence.
- **Alternatives considered:**
  - Keep the current names and only reorder them in output formatting — rejected because the underlying evidence contract itself was awkward.
  - Emit an array instead of named fields — rejected for now because the current evidence style across checks is object-based and the ordered field names are sufficient.

### Keep examples and docs aligned with runtime defaults
- **Chose:** Update the full config example, sample config fixture, loader tests, and `AGENTS.md` snippet to show `maxSentences: 6`.
- **Why:** Once defaults are part of the product contract, stale examples become a source of false assumptions.
- **Alternatives considered:**
  - Update runtime code only — rejected because the repo already uses the full-config file and handoff doc as reference surfaces.

## Architectural Notes
This is a small but important contract cleanup: the paragraph-length threshold is a shared quality default, so changing it touches domain defaults and any example config intended to mirror defaults. The triple-repeat evidence change is localized to the heuristic implementation and its tests; the output layer did not need special handling because it already forwards evidence keys transparently.

## Information Sources
- `apps/prosesmasher/crates/domain/types/src/config.rs` — quality defaults
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/triple_repeat.rs` — evidence generation
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/triple_repeat_tests.rs` — regression coverage for evidence keys
- `apps/prosesmasher/presets/full-config-en.json` — canonical full example
- `AGENTS.md` — handoff config snippet
- Real validation run on `/Users/tartakovsky/Projects/steady-parent/apps/landing/content/blog/posts/en/adhd-sensory-asd-signs.mdx`

## Open Questions / Future Considerations
- `paragraph-length` is still counting MDX component blobs like `<BlogHowTo ...>` and `<BlogFAQ ...>` as paragraphs. That is now more visible because the threshold was relaxed to 6 and the remaining failures on real drafts are mostly markup-driven.
- If we keep refining the rewrite-loop contract, some other multi-sentence evidence shapes may want the same ordered naming treatment.

## Key Files for Context
- `apps/prosesmasher/crates/domain/types/src/config.rs` — source of default quality thresholds
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/triple_repeat.rs` — triple-repeat evidence contract
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/triple_repeat_tests.rs` — proof of ordered evidence keys
- `apps/prosesmasher/presets/full-config-en.json` — reference full config users can dump/edit
- `AGENTS.md` — current high-level product and config contract
- `.worklogs/2026-03-22-213522-clean-output-contract.md` — prior output cleanup context
- `.worklogs/2026-03-22-214230-tighten-word-repetition.md` — adjacent heuristics cleanup from the same review cycle

## Next Steps / Continuation Plan
1. Filter MDX/JSX component paragraphs out of `paragraph-length`, since they still produce low-value failures in real article runs.
2. Re-run the validator on the same `steady-parent` articles after that filter and verify that only genuinely prose-like paragraphs remain in `paragraph-length` evidence.
3. If more checks keep surfacing component noise, introduce a shared prose-block predicate in core rather than per-check ad hoc filters.
