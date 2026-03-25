# LLM Slop Detection Plan

## Goal

Add a new layer of prose-quality checks that catches LLM-style boilerplate, canned assistant language, and overused diction without overfitting to literal seed phrases and without inventing a second scoring system on top of the existing checker model.

This plan does **not** replace the current checks. It adds new checks in the same architecture and uses the same overall result model.

## Core Design Decisions

### 1. Every check is either `immediate` or `accumulative`

That is the only execution distinction that matters.

- **Immediate**: one violating match is enough to flag the check.
- **Accumulative**: weak matches are collected inside that check, and only that check's thresholds determine whether the result is `warning` or `error`.

We do **not** need more behavioral kinds than that.

### 2. No cross-check accumulation

Warnings do not add up across unrelated checks.

This is wrong:
- 1 weak signpost match
- 1 weak LLM-vocabulary match
- 1 weak softening-language match
- therefore error

This is correct:
- each check accumulates only its own matches
- thresholds are per check
- a document can have several weak warnings across different checks without any of them becoming an error

### 3. Do not assign severity phrase-by-phrase

That approach is arbitrary and will rot quickly.

If one family needs "bad on sight" behavior and another needs cumulative behavior, they should be separate checks.

Examples:
- `as an ai language model` should not live in the same check as `delve`
- `without further ado` should not live in the same check as `might be`

### 4. Seed phrases are starting material, not the final product

The user-provided examples are not meant to become a dumb blacklist. For each example, we need to derive the broader rhetorical family and build a deterministic matcher for that family.

### 5. Keep the first version simple

For accumulative checks, do not build weighted scores, spread analysis, distinct-term scoring, or cross-category density logic.

For v1:
- collect matches
- compare raw count to thresholds
- report the matched phrases/snippets array

That is enough.

## Result Levels

Checks may produce:
- `critical`
- `error`
- `warning`

Recommended behavior:
- `critical` for hard LLM leakage or explicit assistant disclaimers
- `error` for immediate canned garbage on sight
- `warning` for weak accumulative families below their error threshold

For accumulative checks:
- `count < warning_threshold` => clean
- `warning_threshold <= count < error_threshold` => warning
- `count >= error_threshold` => error

## Existing Architecture Compatibility

These new checks should live alongside the current heuristic checks. They do not require a new parallel check engine.

Existing checks such as:
- `em-dashes`
- `smart-quotes`
- `false-question`
- `word-count`
- `heading-hierarchy`
- `readability`

remain conceptually the same. Most of them are effectively `immediate`.

The new work adds additional checks with the same overall lifecycle:
- parse document
- run check
- emit result

## New Check Families

### Immediate checks

These are "one hit is enough" checks.

#### `llm-disclaimer`

Purpose:
- Catch explicit model leakage and assistant-style inability disclaimers.

Level on match:
- `critical`

Seed examples:
- `as an ai language model`
- `as of my last knowledge update`
- `i do not have access to`
- `i don't have access to`
- `regenerate response`

Matcher strategy:
- exact phrase matching plus a few obvious close variants

Why this is immediate:
- these are not normal stylistic slips; they are direct evidence of assistant leakage

#### `response-wrapper`

Purpose:
- Catch canned assistant preambles and wrapper phrases

Level on match:
- `error`

Seed examples:
- `based on the information provided`
- `certainly, here's`
- `certainly, here are`
- `i'm sorry, but`

Matcher strategy:
- anchored phrase starts and contains matchers

Why this is immediate:
- these are stock assistant wrappers, not merely weak writing

#### `empty-transition`

Purpose:
- Catch stock transition phrases that are poor on sight

Level on match:
- `error`

Seed examples:
- `without further ado`
- `ah, yes`

Matcher strategy:
- exact phrase or anchored start

Why this is immediate:
- these are not useful content patterns and do not need repeated use to become a problem

### Accumulative checks

These are "one hit may be acceptable, repetition is not" checks.

#### `boilerplate-framing`

Purpose:
- Catch canned setup framing, curiosity bait, and empowerment boilerplate

Seed examples:
- `if you have ever wondered`
- `have you ever wondered why`
- `now, this might make you wonder`
- `you have the power to`

Suggested thresholds:
- warning: `2`
- error: `3`

Matcher strategy:
- anchored phrase starts
- small token-template families where safe

General family description:
- second-person bait/opening frames that do rhetorical setup work without adding substance

#### `generic-signposting`

Purpose:
- Catch repetitive stock "here is the point" scaffolding

Seed examples:
- `here's the kicker`
- `here's the part most people miss`
- `it's important to note`
- `it's important to remember`
- `important to consider`
- `the most important thing is`

Suggested thresholds:
- warning: `2`
- error: `4`

Matcher strategy:
- anchored phrase starts
- short canned signpost lexicon

General family description:
- over-explicit narrative signposts that make prose sound templated

#### `llm-vocabulary`

Purpose:
- Catch overused LLM-era diction that becomes conspicuous through repetition

Seed examples:
- `delve`
- `tapestry`
- `vibrant`
- `landscape`
- `realm`
- `embark`
- `excels`
- `vital`
- `comprehensive`
- `intricate`
- `pivotal`
- `moreover`

Suggested thresholds:
- warning: `3`
- error: `5`

Matcher strategy:
- simple lexicon hits on normalized words

General family description:
- overrepresented diction that is not inherently invalid but reads templated in clusters

Important note:
- a single `delve` should not warn
- a cluster of these words should

#### `softening-language`

Purpose:
- Catch repeated low-commitment hedging and watered-down claims

Seed examples:
- `typically`
- `more often than not`
- `might be`
- `don't always`
- `may`
- `can also`

Suggested thresholds:
- warning: `3`
- error: `6`

Matcher strategy:
- lexicon hits and a few small fixed phrases

General family description:
- weak, noncommittal phrasing that is fine occasionally but becomes sludge when stacked

Important note:
- this family must stay accumulative
- many of these expressions are valid English individually

#### `universalizing-claims`

Purpose:
- Catch canned, overbroad generalizations about what "everyone" or "most people" want or do

Seed examples:
- `everyone wants to`
- `for most people`
- `we all`

Suggested thresholds:
- warning: `2`
- error: `4`

Matcher strategy:
- anchored phrase starts
- small token templates

General family description:
- broad claim-openers that flatten nuance and often read as canned synthesis

## Non-Overfitting Rules

These rules are mandatory. They are the difference between a useful system and a dumb phrase blacklist.

### 1. Every seed phrase must be mapped to a broader family

We do not ship a phrase because it was mentioned once in chat. We ship a family because we understand why the phrase belongs to it.

Examples:
- `if you have ever wondered`
  Family: curiosity-bait framing
- `here's the kicker`
  Family: canned signposting
- `delve`
  Family: overused LLM diction

### 2. Exact matching is only for true signatures

Use exact matching where broader generalization would be noisy:
- `as an ai language model`
- `as of my last knowledge update`
- `without further ado`

Do not treat broad ordinary language the same way:
- `might be`
- `may`
- `for most people`

Those only belong in accumulative checks.

### 3. If one instance can be acceptable, it must not be an immediate check

This is the key quality gate.

Bad immediate candidates:
- `might be`
- `typically`
- `delve`

Good immediate candidates:
- `as an ai language model`
- `regenerate response`
- `without further ado`

### 4. If a family needs radically different escalation behavior, split it

Do not shove unlike things into one bucket and then rescue the design with per-phrase severity.

Correct:
- `llm-disclaimer`
- `response-wrapper`
- `llm-vocabulary`

Wrong:
- one mega-check containing `as an ai language model`, `without further ado`, and `delve`

## Matcher Strategy

Keep matcher types small and deterministic:

- exact phrase
- anchored start phrase
- contains phrase
- lexicon hit
- small token-template matcher

Avoid:
- giant regex soup
- probabilistic language models
- weighted scoring systems

## Output Shape

For the new checks, the output only needs:
- check id
- level
- count
- matches

Example:

```json
{
  "id": "llm-vocabulary",
  "level": "warning",
  "count": 4,
  "matches": ["delve", "realm", "delve", "moreover"]
}
```

The consumer on the other side can decide how ugly that cluster looks. The checker does not need to over-explain it.

## Relationship To Existing Checks

Before implementing anything, audit overlap against existing heuristics:

- `llm-openers`
- `false-question`
- `fragment-stacking`
- `negation-reframe`
- `hedge-stacking`

Questions to answer during implementation:
- Should `llm-openers` be extended into `boilerplate-framing`, or kept separate?
- Should `hedge-stacking` stay separate from `softening-language`, or should they share matcher helpers?
- Are some "best part? it's this." style examples already covered sufficiently by existing cadence checks?

The default bias should be:
- reuse existing checks where coverage is already close
- add new checks only for genuinely new families

## Implementation Sequence

1. Write down the final first-batch checks and mark each one `immediate` or `accumulative`.
2. For each check, define:
   - purpose
   - family description
   - seed examples
   - matcher type
   - thresholds if accumulative
3. Audit overlap with current heuristic checks and decide whether each family is:
   - new
   - merged into an existing check
   - replacing an existing check
4. Add config support:
   - immediate checks: `enabled`
   - accumulative checks: `enabled`, `warningThreshold`, `errorThreshold`
5. Implement reusable matcher helpers for:
   - exact phrases
   - anchored phrase starts
   - lexicon hits
   - small token templates
6. Implement immediate checks first:
   - `llm-disclaimer`
   - `response-wrapper`
   - `empty-transition`
7. Implement accumulative checks next:
   - `boilerplate-framing`
   - `generic-signposting`
   - `llm-vocabulary`
   - `softening-language`
   - `universalizing-claims`
8. Update output so accumulative checks emit `count` and `matches`.
9. Add positive and negative tests for every family.
10. Tune thresholds only after running on real documents.

## Test Plan

For each new check:

### Positive tests
- literal seed phrases should match
- obvious same-family variants should match

### Negative tests
- nearby legitimate prose should not match
- isolated ordinary-language examples should stay clean where intended

### Threshold tests
- just below warning threshold
- warning threshold hit
- error threshold hit

### Isolation tests
- warnings from different accumulative checks do not combine

### Regression tests
- quoted/meta-discussion text should be audited later as a false-positive risk

## Open Questions

- Whether quoted or explicitly discussed bad phrases should be suppressed in v1 or deferred
- Whether `llm-openers` should remain separate or fold into `boilerplate-framing`
- The exact public config field names for the new checks

## First Practical Principle

When evaluating a new sloppy phrase report, ask:

1. Is this bad on sight?
2. If not, is it only bad in repetition?
3. What broader family does it belong to?
4. Is that family already covered by an existing check?

If those questions are answered consistently, the checker will stay coherent instead of devolving into a random phrase blacklist.
