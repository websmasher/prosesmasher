# Bare-NP verb-mirror to "it" reframe matcher

## Summary

Added `np_action_verb_to_it_corrective` matcher for the slop pattern `[Bare NP] does not [V] X. It [V]s Y.` with subject-verb agreement conjugation. User-surfaced example: "Google does not always rank the companies you compete against on pricing pages. It ranks whoever owns the questions buyers are asking before they look at a pricing page."

## Decisions

This pattern's structural ambiguity with legitimate technical writing is real and previously caused me to flag it as out-of-scope. The slop and legit shapes are identical:

- Slop: "Google does not rank X. It ranks Y."
- Legit: "The function does not return null. It returns the first match."

The distinguishing signal I landed on: **bare-noun subjects vs determiner-led NPs.** Slop tends to use proper nouns or abstract nouns without determiners ("Google", "Communication", "Shame"). Technical descriptions of object behavior use definite NPs ("the function", "the parser", "the cache"). Restricting the matcher to subjects without determiners (`the/a/an`) catches the slop and lets technical writing pass.

This keeps the existing FP guards intact: `technical_entity_not_problem_does_not_trigger_human_followup` and `non_agentive_np_copular_to_pronoun_does_not_trigger` both pass.

The matcher also handles subject-verb agreement conjugation in S2: `is_third_person_singular_of` accepts the bare verb, `+s`, `+es`, and the `y → ies` rule, covering the common third-person-singular forms.

Optional adverb skip in S1 (`always`, `necessarily`, `usually`, `really`, `actually`, `just`) so phrases like "does not always rank" still match the verb on the next token.

`[V] to [INFINITIVE]` skip retained from the existing pronoun matcher.

## Audit

Corpus run on the 202-file fixture set: **3 new hits, all clear AI slop**:

- "Beating procrastination does not require becoming a new person with a color-coded soul. It requires lowering friction, managing emotion, and making the next action easy to start."
- "Communication does not stay strong by accident. It stays strong when partners keep choosing it, especially after it starts to fray."
- "Shame does not make you more consistent. It makes the task feel heavier, which makes avoidance more tempting."

All workspace tests pass (94 cadence-pattern tests, 3 new). FP-guard tests still pass: technical "the parser does not need X" and "the function does not return null" cases correctly excluded by the determiner rule.

## Key files

- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs` — new matcher, helpers (`strip_optional_adverb`, `is_third_person_singular_of`)
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs` — 3 new tests (proper-noun positive, bare-abstract-noun positive, determiner-led negative)

## Next steps

- The determiner rule is the load-bearing precision gate. If false positives surface in real-world writing on bare-noun subjects, the next move is a curated stop-list of common bare-noun heads (e.g. "data", "code", "users") rather than relaxing the rule.
