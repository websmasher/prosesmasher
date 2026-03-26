# Plan Next LLM-Slop Families

**Date:** 2026-03-26 18:10
**Scope:** `.plans/2026-03-26-boilerplate-conclusion-expansion-plan.md`, `.plans/2026-03-26-response-wrapper-expansion-plan.md`, `.plans/2026-03-26-generic-signposting-expansion-plan.md`, `.plans/2026-03-26-universalizing-claims-strengthening-plan.md`, `.plans/2026-03-26-authority-padding-plan.md`

## Summary
Added five scoped planning notes for the next `llm-slop` expansion areas. Each note is intentionally constrained around combo-based heuristic detection, explicit false-positive boundaries, and real fixture anchors from the recent `opus_4_6` adversarial review sweep.

## Context & Problem
The first `llm-slop` family pass catches explicit disclaimer/wrapper language and some rhetorical scaffolding, but the richer generated-article corpus exposed a different class of misses: polished article-template slop. The user wanted plans for each target family with careful scope boundaries, especially balancing two constraints:

- move away from brittle exact-phrase matching
- preserve near-zero false positives for phrases that a high-quality human article might legitimately use

The right planning artifact here is not a single vague roadmap. It is one note per family so each rule boundary can be debated independently before implementation.

## Decisions Made

### Split the next work into five separate family notes
- **Chose:** Write one plan note each for:
  - `boilerplate-conclusion`
  - `response-wrapper` expansion
  - `generic-signposting` expansion
  - `universalizing-claims` strengthening
  - `authority-padding`
- **Why:** These families overlap in “article-template slop” feel, but their false-positive boundaries are different enough that one merged note would blur the matcher rules.
- **Alternatives considered:**
  - One umbrella plan for all next `llm-slop` work — rejected because it would collapse the family-specific boundary decisions into mush.
  - Start implementing directly from the sub-agent review results — rejected because the user explicitly asked for scoped planning first.

### Optimize each plan for combo-based heuristics
- **Chose:** Frame every plan around combinations of signals rather than exact phrases.
- **Why:** The user’s requirement was clear: the less exact-phrase-driven, the better. The corpus also showed that the problem is article-template function, not just repeated literal strings.
- **Alternatives considered:**
  - Phrase lists as the primary design — rejected because they are fast to ship but too brittle and too noisy.
  - Very abstract NLP-only future ideas — rejected because the immediate next rules still need deterministic, testable matcher boundaries.

### Make false-positive boundaries explicit in every plan
- **Chose:** Every note includes a “Should not fire on” section and at least one bad false-positive example.
- **Why:** These families all target phrasing that can appear in legitimate writing. Without explicit anti-goals, implementation drift would be too easy.
- **Alternatives considered:**
  - Leave false-positive discussion implicit in examples — rejected because that encourages overbroad matcher creep.

## Architectural Notes
These plan notes continue the current design discipline:
- rule family per semantic ownership boundary
- combo-based deterministic heuristics
- evidence-first implementation
- accumulative vs immediate behavior decided at the check level, not per phrase

The notes also deliberately anchor each family to real fixture files from the recent `opus_4_6` review so implementation can be driven by repeated live misses instead of ideation alone.

## Information Sources
- `fixtures/opus_4_6/*/article.md` — adversarial corpus that exposed the current misses.
- Prior `llm-slop` plans:
  - `.plans/2026-03-25-llm-slop-detection-plan.md`
  - `.plans/2026-03-26-slop-taxonomy-nlp-followup-plan.md`
- Recent sub-agent review outputs over the `opus_4_6` article set, especially repeated findings around reassurance pivots, signposting filler, authority padding, and boilerplate conclusions.

## Open Questions / Future Considerations
- `template-sequencing` looked real in the corpus but did not make this cut because it still needs stronger cross-article evidence.
- `boilerplate-conclusion` may later split into reassurance endings vs motivational moral endings, but that boundary is not worth formalizing before implementation data exists.
- `authority-padding` is the most likely family to produce false positives if implemented too eagerly; it should stay tightly combo-scoped.

## Key Files for Context
- `.plans/2026-03-26-boilerplate-conclusion-expansion-plan.md` — conclusion-family scope and anti-goals.
- `.plans/2026-03-26-response-wrapper-expansion-plan.md` — article-mode reassurance wrapper expansion.
- `.plans/2026-03-26-generic-signposting-expansion-plan.md` — low-information transition/signpost expansion.
- `.plans/2026-03-26-universalizing-claims-strengthening-plan.md` — broader human-generalization scope.
- `.plans/2026-03-26-authority-padding-plan.md` — vague prestige/evidence scaffolding family.
- `.plans/2026-03-25-llm-slop-detection-plan.md` — original `llm-slop` architecture baseline.
- `.worklogs/2026-03-26-174550-fix-sentence-case-cli-release.md` — most recent release context; useful if implementation starts from the freshly released CLI state.

## Next Steps / Continuation Plan
1. Pick the first implementation target from these five. The strongest next candidate is `boilerplate-conclusion`, because it recurred across many reviewed articles and has a clear human-vs-template boundary.
2. For the chosen family, extract 3-5 repeated live positive snippets from `fixtures/opus_4_6` and pair them with 3-5 human-legit negative near-misses before touching runtime code.
3. Implement the new or expanded rule inside `apps/prosesmasher/crates/app/checks/llm-slop/` with:
   - runtime rule file
   - assertions file
   - sidecar synthetic tests
   - additive fixture sidecar updates only if the new hits are real and no old expected failures disappear.
