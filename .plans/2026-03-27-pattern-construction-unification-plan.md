# Pattern Construction Unification Plan

## Goal

Refactor the ad hoc rhetorical matchers so they are built from constrained construction parts instead of hand-enumerated full strings or one-off tuple lists.

The target shape is:

- keep the unit of matching at the rhetorical construction level
- factor repeated pieces into constrained families
- avoid a generic grammar matcher or free cartesian products
- preserve high precision by only composing approved part families

This plan was triggered by `negation-reframe`, where the code had started to accrete many hardcoded branches for:

- repeated abstract frames
- repeated need/want corrective pairs
- human-subject corrective followups
- copular same-subject corrective pairs

The same code smell exists in a few other `llm-slop` rules, though not all of them need the same depth of refactor.

---

## Core Design

### Construction, not phrase bag

The reusable unit should be a **construction helper**, not a raw list of strings and not a generic grammar engine.

Good:

- same-subject copular contrast
- same-subject action contrast
- same-frame corrective pair
- first-person capability wrapper
- authority subject + credibility predicate frame

Bad:

- all pronouns × all negations × all verbs
- generic `not X / Y` matcher
- any repeated verb pair

The helper should accept constrained parts such as:

- `subject family`
- `auxiliary/copular family`
- `predicate family`
- `order` (`negated -> affirmative`, `affirmative -> negated`, or either)
- optional `word count` / `position` / `bridge token` constraints

### Suggested primitive vocabulary

These do not need to become public shared framework types immediately, but the runtime code should converge on these concepts:

- `SubjectFamily`
  - examples: `["i"]`, `["you"]`, `["we", "they", "you"]`, human plural nouns, human singular nouns
- `CopularFamily`
  - examples:
    - first-person singular: `am not -> am`
    - singular present: `is not -> is`, `isn't -> is`
    - plural present: `are not -> are`, `aren't -> are`
- `ActionNegationFamily`
  - examples: `do not`, `don't`, `does not`, `doesn't`
- `PredicateFamily`
  - examples:
    - `looking for`
    - `need`
    - `want to turn`
    - abstract frame prefixes like `the goal is`
    - credibility tails like `the evidence is`
- `ContrastOrder`
  - `NegatedThenAffirmative`
  - `AffirmativeThenNegated`
  - `Either`

### Matcher rules

Each construction helper should:

- compose only from approved families
- enforce same-subject or same-frame reuse explicitly
- support both orders where the rhetoric is equivalent
- remain narrow enough that technical/explanatory prose is not swallowed by accident

---

## Phase 1: Negation-Reframe Refactor

### Why this rule first

`heur_05_negation_reframe.rs` is now the biggest concentration of construction logic that is still partly handwritten. It already has several families that want to collapse into a shared construction model.

### What to unify

#### 1. Same-subject copular contrast

Current examples:

- `they are not looking for X -> they are looking for Y`
- future reverse: `they are looking for X -> they are not looking for Y`
- future same family with singular subjects:
  - `it is not X -> it is Y`
  - `he is not X -> he is Y`
  - `i am not X -> i am Y`

Refactor into:

- `same_subject_copular_contrast(subjects, copular_family, predicate_family, order)`

This helper should replace hand-written tuples like:

- `("they are not looking for ", "they are looking for ")`

#### 2. Same-subject action contrast

Current examples:

- `kids do not take pleasure ... -> they keep poking ...`
- `calm adults do not erase ... -> they shorten ...`
- `people do not procrastinate because ... -> they procrastinate because ...`

Refactor into:

- `same_subject_action_contrast(subject_family, negation_family, predicate_family, followup_family, order)`

Important:

- do **not** make `predicate_family` unconstrained
- harvest actual slop predicate families from fixtures before adding them

#### 3. Same-frame corrective pair

Current examples:

- `the goal is not X -> the goal is Y`
- `your job is not X -> your job is Y`
- `the best result is not X -> it is Y`
- `the useful alternatives are not X -> they are Y`

Refactor into:

- `same_frame_copular_contrast(frame_family, followup_family, order)`

This should absorb:

- `ABSTRACT_FRAME_NEGATIONS`
- `ABSTRACT_FRAME_AFFIRMATIVES`
- the current special-case “it is / they are” followup behavior

#### 4. Need / want / problem subfamilies

These are already conceptually separate and should stay separate, but their internals can use the same part vocabulary:

- `need` family
- `want-to-turn` family
- `is-not-the-problem` family

Refactor target:

- use shared subject/case/aux helpers
- keep family-specific semantic constraints

### What not to force into the abstraction

These should stay as specialized constructions, not over-generalized:

- infinitive contrast: `not to X -> to Y`
- lifecycle reversal: `doesn't begin X -> it ends Y`
- explicit make contrast: `doesn't make X -> but it makes Y`
- less/more-like pair

They are already construction-level and do not obviously benefit from part-factorization yet.

---

## Phase 2: Other Rules That Should Adopt The Same Approach

### A. `contrastive-aphorism`

File:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`

Why:

- currently mixes token-slice shapes and one-off phrase families
- several constructions are really the same pattern family with different lexical parts

Candidates to unify:

- imperative contrast:
  - `bring a pattern, not a vibe`
- human-plural + abstract contrast:
  - `kids get kind in reps, not revelations`
- “like X, not Y” constructions

Refactor target:

- `imperative_contrast_construction(...)`
- `subject_gets_x_not_y_construction(...)`
- `like_problem_not_problem_construction(...)`

Do **not** over-abstract the curriculum pair yet; it is already clean.

### B. `response-wrapper`

File:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`

Why:

- currently a phrase-family bag split across capability / advice / diagnosis branches
- wants a construction model for:
  - first-person subject
  - capability/limitation auxiliary
  - action family
  - object family

Refactor target:

- `first_person_capability_wrapper(subjects, capability_family, action_family, object_family)`
- `first_person_limitation_wrapper(subjects, limitation_family, action_family, object_family)`

Keep the family boundaries:

- information wrapper
- advice limitation
- diagnosis limitation

### C. `authority-padding`

File:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding.rs`

Why:

- right now it is mostly `starts_with_any(...)`
- the rule has already grown into several repeated “authority subject + credibility predicate” families

Refactor target:

- `authority_subject_frame(subject_family, predicate_family)`

Examples:

- `the evidence is ...`
- `the research ...`
- prestige frame variants

This is a good candidate for part-factorization, but lower priority than negation and response-wrapper.

### D. `generic-signposting`

File:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs`

Why:

- less structurally broken than negation
- but several arrays are really one meta-frame family:
  - question frames
  - answer frames
  - sequence frames
  - frame-signposts

Refactor target:

- `meta_frame_construction(head_family, tail_family)`

Examples:

- `the useful question is`
- `the practical answer is`
- `a simple sequence works well`

This should stay narrow. The existing phrase arrays are acceptable until there is more real pressure.

### E. `lesson-framing`

File:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_13_lesson_framing.rs`

Why:

- very small right now, so this is not urgent
- but it is another example of “head frame + approved qualifier”

Refactor target:

- `lesson_summary_construction(...)`
- `fix_wrapper_construction(...)`

Only do this after the higher-yield rules above.

---

## Rules That Should Mostly Stay As They Are

These are already closer to the right level or are primarily lexical/token-family checks:

- `empty-emphasis`
- `softening-language`
- `universalizing-claims`
- `boilerplate-framing`
- `boilerplate-conclusion`
- `llm-disclaimer`
- `smart-quotes`, `em-dashes`, `fake-timestamps`, other direct style-signals

They may benefit from small helper extraction, but they do not need the same construction-builder treatment.

---

## Execution Order

### 1. Finish negation-reframe unification

Read first:

- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs`
- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe_tests/synthetic.rs`

Do:

1. Introduce shared construction helpers for same-subject contrast
2. Add reverse-order support where rhetoric is equivalent
3. Expand subject/copular families cleanly (`i / he / she / it / we / they / you`, with the right auxiliaries)
4. Migrate existing ad hoc branches onto those helpers
5. Keep predicate families constrained to real fixture-backed examples

Verify:

- cadence runtime tests
- compare scripts for generated / explainers / social corpora

### 2. Refactor `contrastive-aphorism`

Read:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_10_contrastive_aphorism.rs`
- corresponding sidecar tests and assertions

Do:

1. Identify repeated token-slice construction families
2. Replace one-off shape matchers with construction helpers
3. Keep the curriculum pair separate unless a clearer shared shape emerges

### 3. Refactor `response-wrapper`

Read:

- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_02_response_wrapper.rs`

Do:

1. Convert capability/limitation phrase bags into subject/aux/action/object families
2. Preserve current false-positive boundaries
3. Add reverse or variant order only if real fixtures justify it

### 4. Refactor `authority-padding`

Do only after the above are stable.

### 5. Refactor `generic-signposting` and `lesson-framing`

Lower priority cleanup after the high-pressure rules are converged.

---

## Review Discipline

For every refactor:

1. Add synthetic positives for the new construction variants
2. Add synthetic negatives for near-miss legitimate prose
3. Run:
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - `python scripts/generated_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
   - `python scripts/explainer_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
   - `python scripts/social_fixture_failures.py compare --binary apps/prosesmasher/target/debug/prosesmasher`
4. Review every additive hit before snapshot refresh
5. Only then update baselines

---

## Success Criteria

The refactor is successful when:

- rhetorical rules are expressed as constrained construction helpers rather than manual tuple sprawl
- reverse-order equivalents are supported where the rhetoric is genuinely symmetric
- predicate families are harvested from real slop, not generalized blindly
- existing corpora gain reviewed good catches without broad false-positive drift
