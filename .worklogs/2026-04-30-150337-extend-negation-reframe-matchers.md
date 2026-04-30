# Extend negation-reframe with NP-subject matchers

## Summary

Added three new matchers to the negation-reframe check to catch the "X is not Y. X is Z." slop family with NP subjects, modal copulars, and agentive action verbs. Existing specific matchers (goal/point/aim/etc.) still take priority; the new matchers fire as fallbacks.

## Decisions

**Pattern A — `repeated_subject_copular_corrective`**: literal NP subject mirror with copula preserved. "The decision is not X. The decision is Y." Catches subjects beyond the curated `ABSTRACT_FRAME_NEGATIONS` list (decision, problem, fix, child, etc.) and reframes longer than the existing 20-word cap (uses 28).

**Pattern B — `np_modal_negation_to_pronoun_reframe`**: definite-NP + modal copular negation followed by pronoun + same modal. "The page should not be X. It should help Y." Modal preservation across both sentences is the precision anchor.

**Pattern C — `agentive_action_verb_corrective`**: agentive NP + action-verb negation + `they` + same-verb reframe. "A searcher does not want X. They want Y." Three stacked constraints (agentive head noun, intent-verb whitelist, `they`-pronoun reframe) keep it from firing on legit technical writing like "The parser does not need X. It needs Y."

Placement: all three appended to the end of `non_copular_corrective_evidence` so existing labels (`goal is not x -> goal is y`, etc.) win when both could match. This preserved the existing `repeated_*_frame_detected` test labels.

Alternatives rejected: extending `COPULAR_NEGATION_STARTS` to include NP forms — would have collided with the existing labeled-sentence path and required relaxing the 8-word affirmative-relabel cap, which has high FP risk.

## Audit

Ran against the full 202-file fixture corpus. 12 new hits, all from Pattern A, all clearly AI slop ("The fix is not to X. The fix is to Y.", "The child is not being X. The child is Y."). Pattern B and C fired 0 times across the corpus — tight enough not to over-fire on existing fixtures.

All existing FP-protection tests pass. Added two new FP guards:
- `subject_mirror_with_pronoun_in_b_does_not_trigger` — Pattern A requires literal subject repetition, not pronoun substitution
- `non_agentive_technical_entity_does_not_trigger` — Pattern C blocks "the parser does not need X. It needs Y." via agentive-noun gate

## Key files

- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — three new matcher functions, three constant tables (`SUBJECT_DETERMINERS`, `AGENTIVE_NOUNS`, `AGENTIVE_INTENT_VERBS`), wired into `non_copular_corrective_evidence`
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — 9 new tests (5 positive, 4 FP guards)

## Next steps

- Pattern B and C have 0 corpus hits; revisit if user reports specific gaps in agentive/modal slop that aren't being caught.
- Original gomining audit doc still has 0 hits — those patterns ("None of these problems require X. They require Y.", inline modal contrastives, need-to-imperative reframes) are out of scope here and were judged high-FP-risk in earlier analysis.
