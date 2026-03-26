# Add Slop Taxonomy NLP Follow-Up Plan

**Date:** 2026-03-26 15:00
**Scope:** `.plans/2026-03-26-slop-taxonomy-nlp-followup-plan.md`

## Summary
Added a future-facing design note translating the paper “Measuring AI ‘Slop’ in Text” into a practical roadmap for `prosesmasher`. The note focuses on which ideas are actionable for deterministic checks, where lightweight NLP could help, and which directions should explicitly be avoided.

## Context & Problem
The user asked for a plan, not implementation, based on the paper at `https://arxiv.org/pdf/2509.19163`. The key question was not “what did the paper say?” but “what should materially influence `prosesmasher`?” and “is there a lightweight NLP path that could turn the paper’s dimensions into actual rules?” The plan needed to separate useful future rule ideas from abstract metrics that would not change detection outcomes.

## Decisions Made

### Focus The Plan On Actionable Signals
- **Chose:** Emphasize repetition, templatedness, density, and structural scaffolding as the useful outputs of the paper for this codebase.
- **Why:** Those are the dimensions most likely to become deterministic rules with evidence.
- **Alternatives considered:**
  - Centering the plan on coherence, tone, relevance, and factuality — rejected because the paper itself shows those are harder to measure automatically in a reliable lightweight way.

### Treat `nlprule` As A Candidate Spike, Not A Commitment
- **Chose:** Propose `nlprule` as the most plausible lightweight NLP dependency, but only in the context of specific future rules such as template repetition.
- **Why:** The user explicitly does not want a huge heavy NLP stack, and there is no value in adding NLP if it does not produce concrete interpretable rule outcomes.
- **Alternatives considered:**
  - Suggesting spaCy or Stanza — rejected because of Python/model weight and packaging overhead for a CLI.
  - Suggesting no NLP at all — rejected because some structurally useful future rules do plausibly benefit from POS/chunk information.

### Keep The Plan Aligned With Current Architecture
- **Chose:** Frame future additions as normal family/rule work inside the current runtime/assertions/sidecar structure.
- **Why:** The current architecture is now stable and the user wanted to know how the paper can influence detection without derailing that structure.
- **Alternatives considered:**
  - Planning a global slop score or classifier — rejected because it conflicts with the current evidence-first deterministic design.

## Architectural Notes
The plan intentionally preserves the current model:
- many interpretable checks
- organized into owned families
- each producing concrete evidence

The note explicitly recommends:
- adding internal `dimension` metadata for checks
- experimenting with one `nlprule`-powered rule first
- keeping fixture/evidence expectations as the regression floor

It explicitly rejects:
- LLM-as-judge
- opaque global scoring
- heavy Python NLP stacks in the main CLI

## Information Sources
- `https://arxiv.org/pdf/2509.19163` — primary source for the taxonomy and measurement limitations
- current llm-slop rule family in `apps/prosesmasher/crates/app/checks/llm-slop/`
- recent worklogs around the llm-slop expansion:
  - `.worklogs/2026-03-26-134227-add-llm-vocabulary-rule.md`
  - `.worklogs/2026-03-26-140309-add-softening-language-rule.md`
  - `.worklogs/2026-03-26-142727-add-universalizing-claims-rule.md`

## Open Questions / Future Considerations
- Whether `nlprule` is actually cheap enough in binary size and runtime for the packaged CLI still needs a spike, not a guess.
- The current fixture corpus is stronger for disclaimer/wrapper slop than for templatedness and broad-human framing, so future rule validation may need a different corpus.
- If dimension metadata is added later, we still need to decide whether it is internal-only or exposed in CLI/JSON output.

## Key Files for Context
- `.plans/2026-03-26-slop-taxonomy-nlp-followup-plan.md` — the actual future roadmap derived from the paper
- `.worklogs/2026-03-26-142727-add-universalizing-claims-rule.md` — latest llm-slop expansion context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/` — current live family structure that future work should preserve

## Next Steps / Continuation Plan
1. If we want to act on this plan, start with a small `nlprule` spike and benchmark startup cost, binary growth, and output quality.
2. If the spike looks viable, design one concrete `template-repetition` rule in the normal family architecture before touching any other NLP-backed ideas.
3. If the spike is not viable, fall back to non-NLP structural heuristics around templatedness and low-information padding.
