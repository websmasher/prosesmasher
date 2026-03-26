# Universalizing Claims Strengthening Plan

## Goal
Strengthen `universalizing-claims` so it catches broader human-generalization templates without grabbing legitimate bounded statements.

## Why This Family
The existing rule is too narrow. Real misses included:
- `Most people think ...`
- `Every friendship ... is either growing or fading`
- `one of the most universal human experiences`
- broad unsourced mass-population claims

## Detection Principle
Catch strong generalization about people, behavior, or lived experience when the statement is framed as near-universal truth rather than bounded observation.

## Positive Pattern Families

### 1. Majority-mind framing
Examples:
- `Most people think ...`
- `Most of us assume ...`

### 2. Universal experience framing
Examples:
- `one of the most universal human experiences`
- `everyone knows what it feels like`

### 3. Universal binary claims
Examples:
- `Every friendship ... is either growing or fading`
- `There is no stasis`

### 4. Unsourced mass-population framing
Examples:
- `Millions of adults ...`
- similarly broad population claims used rhetorically rather than evidentially

## False-Positive Boundaries
Should not fire on:
- bounded statistical statements with a cited source
- carefully hedged distribution claims
- narrow contextual claims like `most patients in this trial`
- legitimate literal uses of `everyone` that are not universalizing thesis moves

Bad false positive:
- `Most patients in the cohort improved by week six.`

## Matcher Strategy
- sentence-level templates
- require:
  - collective subject or mass-population phrase
  - plus broad cognition/desire/state claim, or strong binary existential claim

This should stay semantic enough to avoid exact-phrase blacklisting.

## Likely Check Shape
- `accumulative`
- repeated universalizing moves across a document should trigger

## Real Fixture Anchors
- `fixtures/opus_4_6/how_burnout_develops_at_work/article.md`
- `fixtures/opus_4_6/why_friendships_fade/article.md`
- `fixtures/opus_4_6/why_people_lose_motivation_after_big_goals/article.md`
- `fixtures/opus_4_6/why_people_wake_up_tired/article.md`

## Initial Test Plan
- positive: `most people think`
- positive: universal human experience sentence
- positive: `every X is either Y or Z`
- negative: sourced or bounded population claim
- negative: literal non-generalizing `everyone`
- negative: meta discussion of universalizing language
