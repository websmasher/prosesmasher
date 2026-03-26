# Response Wrapper Expansion Plan

## Goal
Expand `response-wrapper` from explicit assistant-service phrasing into article-mode reassurance wrappers without swallowing normal responsible prose.

## Why This Family
The current rule catches explicit helper/refusal language well, but misses softer article wrappers like:
- `If this sounds familiar, you are not alone.`
- `The good news is ...`
- `The encouraging news ...`

These are still wrapper moves, just less obviously chatbot-like.

## Detection Principle
This family should catch assistant-like or self-help-coach-like reassurance scaffolding, not every comforting sentence.

A hit should require:
- reassurance framing
- plus a generic article-guidance or explanation handoff

## Positive Pattern Families

### 1. Familiarity reassurance
Examples:
- `If this sounds familiar, you are not alone.`
- `If this scenario feels familiar, you are far from alone.`

Scope:
- second-person recognition plus reassurance
- especially near intros and pivots

### 2. Good-news pivot
Examples:
- `The good news is ...`
- `The encouraging news is ...`

Scope:
- only when used as article reassurance bridge
- not every literal `good news` phrase

### 3. Reassurance-to-guidance handoff
Examples:
- reassurance sentence followed by generalized explanation or advice
- `... and decades of research can explain why this happens and what to do about it`

Scope:
- sentence pair or same-sentence combo
- reassurance + help/explain/fix language

## False-Positive Boundaries
Should not fire on:
- ordinary empathetic prose in memoir or interviews
- precise clinical reassurance with concrete specifics
- plain `good news` in a factual context
- stand-alone `consult a professional` safety advice

Bad false positive:
- a human-written health article saying one reassuring sentence without article-template follow-up

## Matcher Strategy
- combo-based
- sentence-local plus optional neighboring sentence context
- require at least two signals:
  - reassurance cue
  - help/explain/fix/guide cue
  - generic audience address

## Likely Check Shape
- keep as `immediate` for strong wrappers
- possibly add a weak accumulative companion later if softer wrappers appear in clusters

## Real Fixture Anchors
- `fixtures/opus_4_6/adult_procrastination_causes_and_fixes/article.md`
- `fixtures/opus_4_6/why_couples_stop_communicating/article.md`
- `fixtures/opus_4_6/why_people_wake_up_tired/article.md`
- `fixtures/opus_4_6/social_anxiety_in_daily_life/article.md`

## Initial Test Plan
- positive: familiarity reassurance opener
- positive: `good news is` + guidance combo
- positive: reassurance + explain-what-to-do handoff
- negative: literal good news in news-like context
- negative: one empathetic sentence without wrapper/guidance combo
- negative: quoted discussion of the phrase
