# Generic Signposting Expansion Plan

## Goal
Broaden `generic-signposting` to catch article-template transitions that announce importance or structure without adding substance.

## Why This Family
The corpus repeatedly missed:
- `Here is the problem.`
- `The numbers tell a stark story.`
- `The progression tends to follow a pattern.`
- `Notably, ...`

These are classic “pay attention now” signals and structure announcements.

## Detection Principle
This family should catch low-information signposts, not every transition or every discourse marker.

A hit should require:
- a signposting cue
- with little propositional content

## Positive Pattern Families

### 1. Problem announcers
Examples:
- `Here is the problem.`
- `This is where the problem begins.`

### 2. Importance announcers
Examples:
- `The numbers tell a stark story.`
- `This matters more than it seems.`

### 3. Structure announcers
Examples:
- `The progression tends to follow a pattern.`
- `The pattern usually looks like this.`

### 4. Emphasis cue words
Examples:
- `Notably, ...`
- `Importantly, ...`

Scope:
- only when the rest of the sentence stays generic
- not when followed by concrete detail immediately

## False-Positive Boundaries
Should not fire on:
- useful transitions with specific content
- narrow signposts in technical writing where the next clause does real work
- strong argumentative contrast sentences

Bad false positive:
- `Importantly, the trial excluded patients with renal failure.` because that sentence is specific

## Matcher Strategy
- sentence-level scoring
- combine:
  - signpost opener
  - generic noun or vague predicate
  - low specificity

This family should remain accumulative.

## Likely Check Shape
- `accumulative`
- current `maxPerDocument` style thresholding remains appropriate

## Real Fixture Anchors
- `fixtures/opus_4_6/how_burnout_develops_at_work/article.md`
- `fixtures/opus_4_6/why_friendships_fade/article.md`
- `fixtures/opus_4_6/why_people_lose_motivation_after_big_goals/article.md`

## Initial Test Plan
- positive: empty `Here is the problem`
- positive: `Notably` + generic claim
- positive: `numbers tell a stark story`
- negative: `Importantly` + concrete specific clause
- negative: `Here is the problem` inside quote/meta discussion
- negative: useful technical progression sentence with specifics
