# Add Profession Modes Note

**Date:** 2026-03-23 21:27
**Scope:** `AGENTS.md`

## Summary
Added a future-direction note to the project handoff document for profession-specific product modes. The note captures the current strategic conclusion: domain-targeted offerings are more likely to combine `prosesmasher` guardrails with LLM prompting than to rely entirely on new hard-coded heuristics.

## Context & Problem
The project discussion shifted from generic "text quality" positioning toward marketable profession-specific modes such as legal, policy, or technical writing. The conclusion from that discussion was that multilingual, profession-specific quality enforcement is unlikely to come entirely from local heuristic expansion or off-the-shelf style engines, and is better represented as a hybrid direction: deterministic `prosesmasher` guardrails plus LLM-driven rewrite guidance. That decision needed to be captured in the repo's active handoff document so future work does not revert to the earlier "just add more heuristics" framing.

## Decisions Made

### Record profession-specific modes as a roadmap item
- **Chose:** Add a single explicit roadmap bullet under `## Next Steps` in `AGENTS.md`.
- **Why:** `AGENTS.md` is the project's live cold-start handoff, so future agents need to see this product direction in the same place as other pending improvements.
- **Alternatives considered:**
  - Add the note only to the README — rejected because this is internal planning context, not user-facing documentation.
  - Defer documenting it until implementation planning starts — rejected because the product decision has already been made and should not depend on memory.

### Describe the future mode as hybrid, not heuristic-only
- **Chose:** Phrase the item as `prosesmasher` guardrails plus LLM prompting for domain-specific rewrite guidance.
- **Why:** That reflects the current conclusion that profession-specific style is more plausibly delivered by constrained prompting than by endlessly expanding deterministic heuristics.
- **Alternatives considered:**
  - Describe it as new heuristics/presets only — rejected because that overstates what the current engine can realistically express.
  - Describe it as external style platforms such as Acrolinx/Vale — rejected because the discussion concluded those are not the primary direction.

## Architectural Notes
No code or CLI behavior changed. This is a planning/documentation update only, but it matters for future product architecture because it nudges profession-specific expansion toward a hybrid validation-plus-LLM model instead of more internal heuristic sprawl.

## Information Sources
- `AGENTS.md` — current handoff document and roadmap section
- Current conversation context about profession-specific positioning, Vale/Acrolinx/LanguageTool tradeoffs, and LLM-assisted rewrite guidance

## Open Questions / Future Considerations
- Whether profession-specific modes should be surfaced as separate products, presets, or a higher-level orchestration layer
- Whether any first profession target should be English-only initially or designed from the outset for multilingual prompting
- How tightly LLM rewrite prompts should be coupled to `prosesmasher` output schema

## Key Files for Context
- `AGENTS.md` — live project handoff, roadmap, architecture, and known limitations
- `.worklogs/2026-03-23-184140-release-0-1-6-final-cleanout.md` — latest release and repo-state checkpoint before this roadmap update
- `.worklogs/2026-03-23-183336-public-readme-and-corrective-hardening.md` — recent public-facing positioning work that informs how future product directions should be documented

## Next Steps / Continuation Plan
1. When planning profession-specific work, start by defining the product surface: preset, API mode, or LLM-assisted rewrite workflow.
2. If that work proceeds, update `README.md` only after the feature shape is concrete enough for user-facing documentation.
3. Keep `AGENTS.md` as the authoritative place for internal roadmap items that are still exploratory but strategically decided.
