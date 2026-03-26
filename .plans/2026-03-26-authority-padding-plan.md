# Authority Padding Plan

## Goal
Add a new family for vague prestige and research-authority scaffolding that signals evidence without providing concrete attribution or specific substance.

## Why This Family
This repeated across the article sweep and does not fit cleanly inside the current rules:
- `A growing body of scientific evidence ...`
- `Research consistently shows ...`
- `Decades of research ...`
- `backed by research ...`

These are not always bad on sight, but repeated use is a strong slop signal.

## Detection Principle
Catch vague evidence-posturing, not legitimate research reference.

A hit should require:
- authority cue
- vague or abstract claim
- missing concrete anchor such as study, source, dataset, named institution, or specific finding

## Positive Pattern Families

### 1. Growing-body framing
Examples:
- `A growing body of scientific evidence ...`
- `An increasing body of research ...`

### 2. Consistency framing
Examples:
- `Research consistently shows ...`
- `Research repeatedly finds ...`

### 3. Decades-of-research framing
Examples:
- `Decades of research suggest ...`
- `The research is clear ...`

### 4. Backed-by-research handoff
Examples:
- `The good news, backed by research, is ...`

## False-Positive Boundaries
Should not fire on:
- specific source attribution
- concrete reference to trial, survey, cohort, institution, or named paper
- precise evidence summary with numbers or bounded result

Bad false positive:
- `A 2024 randomized trial from X found ...`

## Matcher Strategy
- sentence-level combo matcher
- require:
  - authority cue
  - generic evidence verb
  - absence of concrete anchor tokens

This family should avoid trying to infer actual citation truth. It only detects vague authority padding.

## Likely Check Shape
- `accumulative`
- repeated vague evidence scaffolding should fail

## Real Fixture Anchors
- `fixtures/opus_4_6/why_couples_stop_communicating/article.md`
- `fixtures/opus_4_6/stress_and_physical_symptoms/article.md`
- `fixtures/opus_4_6/why_people_wake_up_tired/article.md`
- `fixtures/opus_4_6/adult_procrastination_causes_and_fixes/article.md`

## Initial Test Plan
- positive: `growing body of evidence`
- positive: `research consistently shows` with vague follow-up
- positive: `backed by research` reassurance sentence
- negative: concrete sourced finding
- negative: named study/institution reference
- negative: precise evidence sentence with numbers
