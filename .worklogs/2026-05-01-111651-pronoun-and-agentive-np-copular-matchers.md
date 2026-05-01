# Pronoun verb-mirror and agentive NP copular -> pronoun reframe

## Summary

Two more negation-reframe matchers for slop variants the existing checks missed:

1. **Pronoun verb-mirror corrective** — "We do not [V] X. We [V] Y." with first/second/third-person pronouns and arbitrary action verbs. Excludes `[V] to [INFINITIVE]` to preserve the existing FP guard against "You do not want to click. You want to save." instructional patterns.

2. **Agentive NP copular -> pronoun reframe** — "The [agentive-headed NP] are/is not X. They/It are/is Y." Uses the same first-4-tokens NP-head scan as the existing agentive matcher so subjects like "the buyers searching these terms" (where the agentive head isn't the last token) match cleanly. Word-count cap is 28 each side, enough for longer reframes.

## Decisions

Both matchers placed at the end of `non_copular_corrective_evidence` so existing curated matchers still win when both could match.

**Pronoun matcher tightness:** the `to`-infinitive exclusion is the load-bearing FP guard. The existing `repeated_want_transform_corrective` only catches the specific `you do not want to X. you want to turn Y into Z.` shape and intentionally leaves bare `you do not want to X. you want to Y.` alone. My matcher would have over-fired on the existing test `repeated_want_without_transform_passes` without the `to`-skip.

**Agentive matcher subject anchor:** the agentive NP-head scan is what distinguishes this from a generic "NP is not X. It is Y." matcher (which would crush legitimate technical writing like "The function is not deprecated. It is part of the public API."). The agentive head requirement (`user`, `customer`, `buyer`, etc.) keeps it tight.

**Why not catch reversed-order limitation patterns** ("X says A. It does not say B."): structurally identical to legitimate scope-limit writing in technical docs (`The function returns the first match. It does not return all matches.`). Out of scope.

## Audit

Ran on the 202-file fixture corpus. **9 new hits, 0 false positives.**

Pronoun verb-mirror (6 hits):
- "You do not need a perfect routine. You need a repeatable one."
- "You do not preserve friendships by hoping harder. You preserve them by making them easier to continue."
- "You do not need a perfect mindset. You need strategies that make starting easier..."
- "You do not need perfect rules. You need a small set of consistent boundaries..."
- "You do not need a dramatic reset. You need a few moments that are real enough..."
- "They do not fail because people are careless. They fail because the language gets fuzzy..."

Agentive NP copular (3 hits):
- "A student studying with a feed open is not just 'using social media.' They are splitting attention..."
- "The people most likely to feel attention problems are not mysterious. They are the ones who combine several risk factors..."
- "The best people I know are not the ones who can make everything sound sophisticated. They are the ones who can strip a mess down to its core..."

All are clearly AI slop in psychology/advisory writing. All workspace tests still pass (91 cadence-pattern tests, 5 new).

Added 5 unit tests:
- `pronoun_verb_mirror_we_detected`
- `pronoun_verb_mirror_they_detected`
- `pronoun_verb_mirror_excludes_to_infinitive` (FP guard for `[V] to [INFINITIVE]` pattern)
- `agentive_np_copular_to_pronoun_reframe_detected`
- `non_agentive_np_copular_to_pronoun_does_not_trigger` (FP guard for non-agentive technical entities)

## Key files

- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — two new matcher functions, one new constant table (`PRONOUN_VERB_MIRROR_SUBJECTS`), one helper (`is_plausible_action_verb`)
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — 5 new tests

## Next steps

- These two matchers close the highlighted gaps from the user's third batch of slop examples. Reversed-order scope-limit patterns ("X says A. It does not say B.") remain explicitly out of scope due to FP risk in technical docs.
