# Boilerplate Conclusion Expansion Plan

## Goal
Catch polished article-template endings that sound finished and authoritative but add little concrete value.

## Why This Family
This was the clearest repeated miss across the `opus_4_6` article sweep. High-quality human articles can end cleanly, but the misses here were not just “a conclusion exists.” They were formulaic reassurance, motivational uplift, moral-summary cadence, and generic “key insight” wrap-ups.

## Detection Principle
Do not flag isolated phrases.

Flag a sentence or closing span only when it combines:
- a conclusion-position cue
- a generic summary or reassurance move
- abstract or universalized language

This should remain mostly structural and combo-based, not literal blacklist matching.

## Positive Pattern Families

### 1. Reassurance close
Examples:
- `The good news is ...`
- `X is not something you have to accept as normal.`

Scope:
- Only when used near the end of the article or section
- Only when the sentence is doing reassurance or release, not making a precise factual claim

### 2. Key-insight close
Examples:
- `The most important insight ...`
- `The deepest reason ...`
- `The single most important idea ...`

Scope:
- Closing or paragraph-final summary claims
- Should combine importance language plus abstract framing

### 3. Motivational uplift close
Examples:
- `X is not a luxury.`
- `With the right adjustments, it can.`
- `You will.`

Scope:
- Short emphatic conclusion sentences
- Often paired with a moralized preceding sentence

### 4. Research-flourish close
Examples:
- `The research is clear ...`
- `Decades of research suggest ...`

Scope:
- Only when used as broad concluding authority wrap-up
- Not when concrete sources or findings are actually cited

## False-Positive Boundaries
Should not fire on:
- precise, evidence-backed conclusions
- concise factual end summaries
- domain-specific endings with concrete next steps
- ordinary `in conclusion`-style closers if they are specific and not generic uplift

Bad false positive:
- a strong conclusion in a serious analysis article that states a concrete thesis with specifics

## Matcher Strategy
- sentence-local templates
- end-of-section or end-of-document position weighting
- combination of:
  - importance cue
  - reassurance or universal frame
  - abstract nouning / uplift

Not:
- raw exact-phrase matching alone

## Likely Check Shape
- `accumulative`
- warning/error by repeated boilerplate close signals across a document

Possibly later:
- one immediate subrule for extremely canned enders if the corpus justifies it

## Real Fixture Anchors
- `fixtures/opus_4_6/adult_procrastination_causes_and_fixes/article.md`
- `fixtures/opus_4_6/why_couples_stop_communicating/article.md`
- `fixtures/opus_4_6/why_people_lose_motivation_after_big_goals/article.md`
- `fixtures/opus_4_6/why_people_struggle_to_build_habits/article.md`
- `fixtures/opus_4_6/why_people_wake_up_tired/article.md`

## Initial Test Plan
- positive: reassuring generic ending
- positive: `most important insight` style closer
- positive: abstract moral-summary ending
- negative: concrete factual conclusion
- negative: direct recommendation with specifics
- negative: non-final sentence using similar wording but doing real argumentative work
