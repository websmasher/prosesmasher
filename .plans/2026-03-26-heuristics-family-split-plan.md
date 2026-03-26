# Heuristics Family Split Plan

## Goal

Split the current `app/checks/heuristics` family into smaller ownership-based families so:
- each family owns a coherent semantic space
- shared runtime helpers stay local to that family
- assertions stay local to that family
- future rule growth lands in obvious buckets instead of re-inflating one 15+ rule crate

This plan is for the full intended scope, including the new `llm-slop` family. It is not a future placeholder.

## Top-Level Decision

The current `heuristics` family is too broad. It mixes:
- punctuation/style signals
- cadence patterns
- rhetorical framing patterns
- persona/posture signals
- LLM-slop signatures and accumulative slop buckets

Those should not stay in one family crate.

## Target Families

### 1. `style-signals`

Owns surface-level style markers and light punctuation/title-style heuristics.

Rules:
- `heur_01_em_dashes`
- `heur_02_sentence_case`
- `heur_03_smart_quotes`
- `heur_04_exclamation_density`
- `heur_08_fake_timestamps`
- `heur_09_colon_dramatic`

Likely shared helpers:
- punctuation scanning
- heading/title-case helpers
- paragraph punctuation counters
- simple sentence/paragraph evidence collectors

### 2. `cadence-patterns`

Owns sentence-rhythm and repeated-structure patterns.

Rules:
- `heur_05_negation_reframe`
- `heur_06_fragment_stacking`
- `heur_07_triple_repeat`

Likely shared helpers:
- sentence windowing
- opener sequence detection
- fragment classification
- repeated-structure evidence collectors

### 3. `rhetorical-framing`

Owns section-position rhetorical patterns and canned framing moves.

Rules:
- `heur_10_llm_openers`
- `heur_11_affirmation_closers`
- `heur_12_summative_closer`
- `heur_13_false_question`

Likely shared helpers:
- section first/last sentence helpers
- section sentence collectors
- rhetorical phrase-family matching
- section-scoped evidence helpers

### 4. `persona-signals`

Owns “speaker posture” and persona/authority performance signals.

Rules:
- `heur_14_humble_bragger`
- `heur_15_jargon_faker`

Likely shared helpers:
- persona phrase-family matching
- tone/posture evidence collectors

### 5. `llm-slop`

This is an active planned family, not a future placeholder.

Initial rules to implement here:
- `llm-disclaimer`
- `response-wrapper`
- `empty-transition`
- `boilerplate-framing`
- `generic-signposting`
- `llm-vocabulary`
- `softening-language`
- `universalizing-claims`

Behavior split inside the family:
- immediate rules
  - `llm-disclaimer`
  - `response-wrapper`
  - `empty-transition`
- accumulative rules
  - `boilerplate-framing`
  - `generic-signposting`
  - `llm-vocabulary`
  - `softening-language`
  - `universalizing-claims`

Likely shared helpers:
- anchored phrase matching
- small token-template matching
- phrase-family matching
- accumulative match counting
- reusable slop-evidence extraction

## What Stays Out of These Families

Do not move these into the split-out heuristic families:
- generic `Check` trait / runner logic
- catalog assembly
- shared suite-result test helpers
- parser behavior
- lexical term pools
- readability metrics
- document-policy checks

Those remain in:
- `app/checks/core`
- `app/checks/catalog`
- `app/checks/test-support`
- parser adapter
- existing non-heuristic families

## Ownership Rule

A helper belongs in a family if:
1. at least two rules in that family use it, and
2. other families probably should not use it

If only one rule needs it:
- keep it in the rule file

If many unrelated families need it:
- move it to a more generic shared location

This rule is meant to prevent another vague `support.rs` blob.

## Proposed Workspace Tree

```text
crates/app/checks/
  style-signals/
    runtime/
    assertions/
  cadence-patterns/
    runtime/
    assertions/
  rhetorical-framing/
    runtime/
    assertions/
  persona-signals/
    runtime/
    assertions/
  llm-slop/
    runtime/
    assertions/
```

Each family keeps the same shape already established elsewhere:

```text
family/
  runtime/
    src/
      lib.rs
      rule_file.rs
      rule_file_tests/
        mod.rs
        synthetic.rs
  assertions/
    src/
      lib.rs
      rule_file.rs
```

## Rule Mapping from Current Files

### `style-signals`

Current files to move:
- `app/checks/heuristics/runtime/src/heur_01_em_dashes.rs`
- `app/checks/heuristics/runtime/src/heur_02_sentence_case.rs`
- `app/checks/heuristics/runtime/src/heur_03_smart_quotes.rs`
- `app/checks/heuristics/runtime/src/heur_04_exclamation_density.rs`
- `app/checks/heuristics/runtime/src/heur_08_fake_timestamps.rs`
- `app/checks/heuristics/runtime/src/heur_09_colon_dramatic.rs`

And their matching assertions + sidecars.

### `cadence-patterns`

Current files to move:
- `app/checks/heuristics/runtime/src/heur_05_negation_reframe.rs`
- `app/checks/heuristics/runtime/src/heur_06_fragment_stacking.rs`
- `app/checks/heuristics/runtime/src/heur_07_triple_repeat.rs`

And their matching assertions + sidecars.

### `rhetorical-framing`

Current files to move:
- `app/checks/heuristics/runtime/src/heur_10_llm_openers.rs`
- `app/checks/heuristics/runtime/src/heur_11_affirmation_closers.rs`
- `app/checks/heuristics/runtime/src/heur_12_summative_closer.rs`
- `app/checks/heuristics/runtime/src/heur_13_false_question.rs`

And their matching assertions + sidecars.

### `persona-signals`

Current files to move:
- `app/checks/heuristics/runtime/src/heur_14_humble_bragger.rs`
- `app/checks/heuristics/runtime/src/heur_15_jargon_faker.rs`

And their matching assertions + sidecars.

### `llm-slop`

New files to create from scratch under the new family.

Suggested initial IDs:
- `slop_01_llm_disclaimer`
- `slop_02_response_wrapper`
- `slop_03_empty_transition`
- `slop_04_boilerplate_framing`
- `slop_05_generic_signposting`
- `slop_06_llm_vocabulary`
- `slop_07_softening_language`
- `slop_08_universalizing_claims`

## Support-File Strategy

The current `heuristics/runtime/src/support.rs` should not simply move whole into one new family.

Instead:
- move section-position helpers into `rhetorical-framing`
- move cadence helpers into `cadence-patterns`
- move punctuation/style helpers into `style-signals`
- move any genuinely single-rule helpers back into the rule file

The goal is to end with family-local support that actually matches ownership.

## Catalog Impact

`app/checks/catalog/runtime` should stop depending on one broad `heuristics` family and instead depend on:
- `style-signals`
- `cadence-patterns`
- `rhetorical-framing`
- `persona-signals`
- `llm-slop`

Grouping behavior can still expose these under a user-facing `quality.heuristics` umbrella if desired. Crate boundaries and user-facing check groups do not need to be the same concept.

## Migration Order

### Phase 1: Split current heuristics without adding new rules
1. Create new family crates:
   - `style-signals`
   - `cadence-patterns`
   - `rhetorical-framing`
   - `persona-signals`
2. Move existing runtime files, assertions, and sidecars into those crates.
3. Move only the helpers each family actually owns.
4. Update catalog/runtime to assemble from the new families.
5. Keep a temporary `heuristics` compatibility facade only if needed to reduce churn, then remove it.

### Phase 2: Add `llm-slop`
1. Create `llm-slop/runtime` and `llm-slop/assertions`.
2. Implement immediate rules first:
   - `llm-disclaimer`
   - `response-wrapper`
   - `empty-transition`
3. Implement accumulative rules next:
   - `boilerplate-framing`
   - `generic-signposting`
   - `llm-vocabulary`
   - `softening-language`
   - `universalizing-claims`
4. Add thresholds to config and DTOs only where the accumulative rules need them.
5. Register the new checks in catalog/runtime.

### Phase 3: Hardening
1. Ensure each new family’s sidecars only generate inputs/harnesses.
2. Ensure all public-behavior assertions live in that family’s assertions crate.
3. Run repeated adversarial test attacks after each family migration.

## Why This Split Is Worth It

- The ownership boundaries become obvious.
- Small families are easier to scan than a single 15+ rule crate.
- New slop-rule growth has a dedicated home immediately.
- Family-local support stops smearing semantics across unrelated rules.
- Assertions stay close to the rules they describe.

## Anti-Pattern to Avoid

Do not recreate a new mega-family by splitting crates but keeping one giant shared support file somewhere central.

If the split is real:
- helpers split too
- assertions split too
- tests split too

Otherwise the crates are fake.
