# Implement Authority Padding And Expand Signposting Conclusion

**Date:** 2026-03-26 22:46
**Scope:** `apps/prosesmasher/crates/app/checks/llm-slop/*`, `apps/prosesmasher/crates/domain/types/runtime/src/config.rs`, `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_dto.rs`, preset assets, generated/explainer fixture baselines, `scripts/explainer_fixture_failures.py`, workspace `Cargo.toml` files, `apps/prosesmasher/CHANGELOG.md`

## Summary
Added the first of the three planned polished-article slop families, `authority-padding`, and expanded `generic-signposting` and `boilerplate-conclusion` with corpus-reviewed patterns. Verified the pass against the full workspace, all six generated-model baseline buckets, and the explainer corpus, then released the workspace as `0.3.2`.

## Context & Problem
After the earlier real-slop work, the next high-yield misses were no longer explicit chatbot leakage. The generated and explainer corpora were showing a more polished article voice with:
- vague research/evidence prestige glue
- empty answer/compression framing
- canned moral-summary or compression closers

The user wanted the first three planned families implemented, but with two hard constraints:
- less phrase-bag matching and more narrow rhetorical/function-based detection
- effectively zero false positives on prose that a good article might reasonably want to say

The practical loop for this pass was:
1. freeze current generated/explainer errors as baselines
2. add the rule family or matcher expansion
3. compare against the corpus
4. manually review every additive hit
5. only snapshot the reviewed additions

## Decisions Made

### Add `authority-padding` as an accumulative `llm-slop` rule
- **Chose:** A new family-owned rule for vague prestige/evidence scaffolding such as:
  - `The research is not mysterious here.`
  - `Researchers keep finding ...`
  - `The broader research backs ...`
  - `The evidence points ...`
- **Why:** These are not the same thing as `generic-signposting` or `boilerplate-conclusion`. They are a specific rhetorical move: invoking research/evidence as atmosphere rather than supplying concrete attribution.
- **Alternatives considered:**
  - Fold them into `generic-signposting` — rejected because that would blur “meta framing” with “prestige framing.”
  - Catch all `research/evidence` mentions — rejected because concrete sourced statements are normal and desirable.

### Explicitly leave concrete sourced authority statements out of scope
- **Chose:** Do not catch lines like `The 2023 review found ...`.
- **Why:** The user explicitly called that example normal enough, and the same judgment held in corpus review. The boundary for this rule is vague authority glue, not citations or study summaries.
- **Alternatives considered:**
  - Broaden to any sentence beginning with a study/review claim — rejected because that would punish legitimate evidence reporting.

### Expand `generic-signposting` with stronger empty compression/meta frames
- **Chose:** Add narrow strong-signpost families such as:
  - `the short answer is`
  - `the short version`
  - `the better conclusion is`
  - `the point is plain enough`
  - `that is the useful frame`
- **Why:** The explainers and generated `gpt_5_4_mini` corpus had repeated empty framing of this exact type, and the additive hits were consistently bad.
- **Alternatives considered:**
  - Leave the older narrower signposting rule alone — rejected because it was missing obvious polished article sludge already present in the fixture corpora.
  - Broaden into `the catch is`, `the reality is`, or `the mechanism is straightforward` in the same pass — rejected because those had materially higher false-positive risk.

### Expand `boilerplate-conclusion` only with reviewed compression/response closers
- **Chose:** Add:
  - `the practical response is plain`
  - `the basic rule is simple`
  - `the whole trick`
  - `the core fact`
  - `the rest is detail`
- **Why:** These were the cleanest remaining conclusion-family misses from the explainer review, and they clustered as moral-summary or compression closers rather than substantive endings.
- **Alternatives considered:**
  - Add `the bad news is / the good news is` — rejected because the user judged that pair normal enough.
  - Add broader “simple/straightforward” rule language — rejected because it quickly bleeds into acceptable instructional prose.

### Tighten `the basic rule is simple` to avoid technical explanatory prose
- **Chose:** Treat `the basic rule is simple` as a short standalone closer only, not as a general same-sentence phrase match.
- **Why:** The first draft incorrectly flagged `The basic rule is simple: parse the file first, then normalize the visible text.` That sentence is a legitimate concrete explanation, not slop.
- **Alternatives considered:**
  - Drop the phrase entirely — rejected because the explainer corpus had a real bad standalone hit.
  - Keep the raw contains-match — rejected because the false positive was immediate and obvious.

### Add a dedicated explainer baseline workflow
- **Chose:** Add `scripts/explainer_fixture_failures.py` mirroring the existing generated-fixture baseline flow.
- **Why:** The new explainers are flat `*.md` files rather than `*/article.md` folders, so the generated script was the wrong shape. We needed the same snapshot/compare discipline for these new fixtures.
- **Alternatives considered:**
  - Fold explainer handling into the generated script — rejected for now because the fixture layouts differ enough that a dedicated script was simpler and clearer.

## Architectural Notes
- `authority-padding` lives in `llm-slop` with the full required shape:
  - runtime rule file
  - assertions module
  - sidecar test folder
- Config wiring followed the same owned path as the rest of the checks:
  - domain config
  - FS DTO mapping
  - preset JSON
  - catalog counts
- `generic-signposting` now has a clearer split between:
  - weaker accumulative signposts
  - strong single-hit meta/compression frames
- `boilerplate-conclusion` remains a closing-family rule, but it now owns a narrow compression-close subset rather than only authority/insight closers.
- The public surface changed additively, so this was released as `0.3.2` rather than another unversioned internal change.

## Information Sources
- User guidance in-session, especially:
  - `The 2023 review found ...` is acceptable and should stay out of `authority-padding`
  - `The bad news is ... The good news is ...` is normal enough and should stay out of `boilerplate-conclusion`
- Corpus review over:
  - `fixtures/gpt_5_4_mini/*`
  - `fixtures/explainers/gpt_5_4_mini/*.md`
- Existing generated baseline tool:
  - `scripts/generated_fixture_failures.py`
- New explainer baseline tool:
  - `scripts/explainer_fixture_failures.py`
- Prior worklogs:
  - `.worklogs/2026-03-26-214418-catch-weak-meta-framing-and-empty-emphasis.md`
  - `.worklogs/2026-03-26-213044-rename-slogan-punchline-to-contrastive-aphorism.md`
  - `.worklogs/2026-03-26-211647-extend-corrective-and-contrastive-negation-coverage.md`

## Open Questions / Future Considerations
- `authority-padding` still leaves some adjacent families for later, especially:
  - `the practical move is ...`
  - `the practical answer is ...`
  - some `the catch is ...` lines
  Those looked promising in review, but not safe enough to bundle into this pass.
- `generic-signposting` likely still has a future batch around `the useful conclusion is simple` and close siblings, but those need the same compare-and-review loop first.
- `boilerplate-conclusion` may grow again, but only if each new closing pattern survives explicit review against both explainers and generated corpora.

## Key Files for Context
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding.rs` — new authority-padding matcher and evidence logic
- `apps/prosesmasher/crates/app/checks/llm-slop/assertions/src/slop_12_authority_padding.rs` — reusable assertions for the new rule
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_12_authority_padding_tests/synthetic.rs` — synthetic positives and false-positive guardrails
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_03_generic_signposting.rs` — expanded meta/compression signpost logic
- `apps/prosesmasher/crates/app/checks/llm-slop/runtime/src/slop_08_boilerplate_conclusion.rs` — expanded conclusion/compression closer logic with the tightened `basic rule is simple` boundary
- `scripts/explainer_fixture_failures.py` — explainer snapshot/compare workflow
- `scripts/generated_fixture_failures.py` — generated article snapshot/compare workflow used as the main regression gate
- `fixtures/explainers/gpt_5_4_mini/*.baseline.general-en.json` — reviewed explainer baselines for this pass
- `fixtures/gpt_5_4_mini/*.baseline.general-en.json` — reviewed generated-model baselines after the additive `boilerplate-conclusion` hit
- `.worklogs/2026-03-26-214418-catch-weak-meta-framing-and-empty-emphasis.md` — direct predecessor release and corpus-review posture

## Next Steps / Continuation Plan
1. Start the next polished-article pass from the reviewed misses already identified in this session:
   - `the practical move is ...`
   - `the practical answer is ...`
   - `the useful conclusion is simple`
   Read the reviewed examples in `fixtures/explainers/gpt_5_4_mini` and `fixtures/gpt_5_4_mini` before changing any matcher.
2. Use the same loop again:
   - compare
   - review every additive hit
   - snapshot only the approved diffs
   Do not bulk-resnapshot unexplained changes.
3. If a future family starts looking cross-cutting rather than cleanly owned by `generic-signposting`, `authority-padding`, or `boilerplate-conclusion`, stop and split it explicitly instead of stretching one of these three rules into a grab bag.
