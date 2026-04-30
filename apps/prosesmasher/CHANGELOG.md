# Changelog

## 0.3.14

- Restructured the repo-root `prosesmasher` package to be a publishable crates.io stub with no real dependencies, so `cargo binstall prosesmasher` works without the `--git` flag. The stub holds only the binstall metadata that points at the GitHub release artifacts; the real CLI continues to be built and shipped from `apps/prosesmasher/packages/prosesmasher` via cargo-dist.
- The stub binary itself prints an explicit error and exits with status 1 if a user runs `cargo install prosesmasher` from crates.io and then tries to invoke the resulting binary, with instructions to use `cargo binstall prosesmasher` (precompiled) or `cargo install --git ...` (source).

## 0.3.13

- Added new `demonstrative-emphasis` cadence check that flags clusters of short demonstrative-subject emphatic sentences such as `That is X.`, `This is where Y.`, `That difference matters.`, `The hard part is the judgment call.` Density-based via `max_per_document` (default 2) so single instances pass and clusters fail. Catches six classifier sub-shapes: demonstrative-copular, demonstrative-relative, demonstrative-perception, demonstrative-emphatic-verb, demonstrative-np-copular (with `-negation` and `-relative` sub-variants), and definite-np-copular.
- Extended `negation-reframe` with three new matchers for NP-subject corrective patterns: literal NP subject mirror with copula preserved (`The decision is not X. The decision is Y.`); NP + modal copular negation followed by pronoun + same modal (`The page should not be X. It should help Y.`); and agentive NP + action-verb negation with pronoun reframe sharing the same verb (`A searcher does not want X. They want Y.`).
- All new matchers placed after existing curated ones so prior labels still win when both could match.

## 0.3.12

- Disabled `colon-dramatic` by default in the canonical heuristics config and the shipped full English preset because the current matcher is still too noisy on normal article/link-intro prose.
- Kept the rule in the catalog for explicit opt-in use, but changed the public default surface so `dump-config --full-config` now emits `"colonDramatic": { "enabled": false }`.

## 0.3.11

- Expanded `generic-signposting` with construction-based abstract evaluation frames such as `The point is to ...`, `What matters is ...`, `What helps is ...`, `The bigger win is ...`, and `The result worth caring about ...`, while leaving concrete procedural prose like `The point is to parse the file ...` and concrete advice like `What helps is reducing caffeine ...` out of scope.
- Expanded `universalizing-claims` with a strong group-behavior opener family like `Most parents keep reaching ...` when it is used as a broad human-generalization frame rather than a concrete statement.
- Expanded `empty-emphasis` with a narrow empty moral relabel branch for deictic virtue lines like `That is discipline.` and `This is discipline.`
- Refreshed only the reviewed explainer, Instagram, and generated `gpt_5_4_mini` baselines for the additive `generic-signposting` hits from this pass while keeping the broader corpus compares clean.

## 0.3.10

- Expanded `negation-reframe` with additional narrow corrective families:
  - `The answer is not X. It is Y.`
  - `That does not make X okay. It does explain Y.`
  - `X teaches Y. It does not teach regulation/self-control/restraint/repair.`
- Expanded `empty-emphasis` with the short dismissive meta line `What helps is not brilliant.`
- Refreshed only the reviewed generated/social baselines for the new approved negation hits while explicitly leaving higher-risk inline single-sentence `..., not ...` contrasts out of scope.

## 0.3.9

- Refactored `response-wrapper` and `authority-padding` into construction-plus-parts matchers built from constrained subject, auxiliary, action, and predicate families instead of phrase-bag conditionals.
- Added synthetic guardrails for `medical expertise` response wrappers and `researchers keep finding ...` authority-padding frames while preserving the existing real-corpus baselines.

## 0.3.8

- Refactored `contrastive-aphorism` toward construction-plus-parts helpers instead of handwritten token-shape branches.
- Added reviewed advisory contrast coverage for short coaching lines such as `I would give one anchor, not a buffet.` and `I would expect repetition, not elegance.`
- Kept the change set regression-safe across the generated, explainer, and social fixture corpora.

## 0.3.7

- expand negation-reframe coverage for repeated corrective coaching frames and same-subject copular/action contrasts
- add a construction-unification plan for refactoring rhetorical matchers into constrained part families

## 0.3.6

- Fixed the GitHub release packaging path for `cargo-dist` by disabling LTO in the dist profile, avoiding the Apple cross-link LLVM bitcode failure that broke the `v0.3.5` binary release.
- Kept the new public install surface unchanged:
  - `cargo binstall --git https://github.com/websmasher/prosesmasher prosesmasher`
  - `cargo install --git https://github.com/websmasher/prosesmasher prosesmasher`

## 0.3.5

- Added a binary-only GitHub release path for `cargo-binstall` installs via the repository root shim package.
- Switched release automation to tag-driven `cargo-dist` publishing on GitHub releases instead of `release-plz`-driven crates.io publishing.
- Documented the supported install paths as:
  - `cargo binstall --git https://github.com/websmasher/prosesmasher prosesmasher`
  - `cargo install --git https://github.com/websmasher/prosesmasher prosesmasher`

## 0.3.4

- Added `lesson-framing` for empty short-form coaching wrappers such as `The biggest lesson was simple.`, `The practical lesson for me was simple:`, and vague evaluative `The fix is boring/plain/not heroic.` lines while leaving concrete technical fixes like `The fix is to initialize the parser...` out of scope.
- Added `observer-guidance` for short reader-observation scaffolds such as `You see it everywhere:`, `You can watch it happen in real time.`, `If this hits home, ...`, and the reviewed `That is where ...` abstract bridge lines from the LinkedIn/Instagram corpora.
- Added `scripts/social_fixture_failures.py` plus checked-in LinkedIn/Twitter/Instagram baseline sidecars so short-form corpora now follow the same snapshot/compare workflow as the generated article and explainer fixture sets.
- Reviewed the additive hits against all three corpus gates and kept the generated-article and explainer baselines unchanged while snapshotting only the approved social additions.

## 0.3.3

- Expanded `generic-signposting` with another narrow polished-article meta-framing batch:
  - `the practical move is ...`
  - `the practical answer is ...`
  - `the practical version is ...`
  - `the useful conclusion is simple`
- Reviewed the additive hits against both the generated `gpt_5_4_mini` corpus and the explainer corpus, then refreshed only those approved baseline sidecars.
- Kept the boundary narrow enough to leave ordinary concrete uses like `Reducing temptation is another practical move ...` out of scope.

## 0.3.2

- Added `authority-padding` for vague research/evidence prestige framing such as `The research is not mysterious here.`, `Researchers keep finding ...`, and `The broader research backs ...` while explicitly leaving concrete sourced statements like `The 2023 review found ...` out of scope.
- Expanded `generic-signposting` with more empty compression/meta frames such as `The short answer is ...`, `The short version ...`, `The point is plain enough ...`, and `That is the useful frame.` after reviewing additive hits in both generated and explainer corpora.
- Expanded `boilerplate-conclusion` with narrow `practical response is plain`, `the basic rule is simple`, `the whole trick`, `the core fact`, and `the rest is detail` closers, while keeping concrete technical `The basic rule is simple: parse the file first ...` prose out of scope.
- Added an explainer-fixture baseline workflow in `scripts/explainer_fixture_failures.py` and refreshed the reviewed `gpt_5_4_mini` generated/explainer sidecars for the additive hits from this pass.

## 0.3.1

- Tightened `generic-signposting` so empty guidance frames like `The useful move is ...` and `The useful question is ...` fail even when they appear alone, while ordinary note/consultation signposts remain accumulative.
- Expanded `empty-emphasis` with deictic significance lines like `That is still real change.` and `That is how the pattern weakens.` without widening into longer explanatory prose.
- Refreshed the generated `gpt_5_4_mini` fixture baselines after reviewing the one new additive `generic-signposting` hit and keeping the rest of the six-model corpus stable.

## 0.3.0

- Breaking: renamed the `slogan-punchline` check to `contrastive-aphorism`, including the public check ID, label, and config key (`contrastiveAphorism`).
- Expanded `negation-reframe` with two additional safe corrective branches:
  - `The biggest sign is not X. It is Y.`
  - `X is not the problem. Y is.`
- Expanded `contrastive-aphorism` with short coaching contrasts such as `Watch for a pattern, not one bad week.` and `You handle it like a nervous-system problem, not a manners problem.`
- Expanded `generic-signposting` with `the useful move is ...` as another narrow meta-framing family while keeping it accumulative.

- Also in this release:
  - quantified human `do not need -> they need` pairs such as `Most new moms do not need ... They need ...`
  - a transformation-specific `do not want -> want to turn ... into ...` branch
- Verified all of the new branches against the six generated-model regression buckets and kept all of those compares clean.

## 0.2.8

- Expanded `negation-reframe` with a repeated noun-subject corrective branch so lines like `The child does not need ... The child needs ...` are caught without broadening into generic contrast prose.
- Expanded `generic-signposting` with narrow meta-framing patterns such as `The answer is simple.`, `The useful question is ...`, and `A simple sequence works well:` while keeping the generated article regression corpus stable.
- Expanded `empty-emphasis` with deictic filler lines like `That one change helped a lot.` and `This is telling you something.` while leaving concrete-subject explanations out of scope.

## 0.2.7

- Added `slogan-punchline` for short sloganized lines like `The rehearsal is the part that sticks.`, `It sounds small, and it changes everything.`, and paired `X is enough for this. X is the curriculum.` constructions.
- Added `blame-reframe` for narrow blame-to-growth coaching contrasts like `comes from development, not malice` and `as skill-building instead of shame`.
- Kept both new rules deliberately narrow and verified that they did not add any regressions across the 60 generated article fixture baselines.

## 0.2.6

- Added `empty-emphasis` for short deictic filler lines such as `That last part matters.` while keeping explanatory sentences and quoted discussion out of scope.
- Expanded `negation-reframe` with safe corrective families like `The goal is not ... The goal is ...` and `You do not need ... You need ...`, then explicitly rejected a broader `comes from x, not y` branch after false-positive review.
- Fixed the generated-fixture baseline tool to track real observed hit counts instead of one failure per rule and refreshed the generated article baselines to that stricter contract.

## 0.2.5

- Added `boilerplate-conclusion` and a generated-fixture baseline compare workflow for model-written article corpora.

## 0.2.4

- Restored the packaged `prosesmasher` wrapper crate files so `cargo run -p prosesmasher` and packaged smoke tests work again after the accidental workspace breakage.
- Normalized `sentence-case` JSON output back to the stable public check ID `sentence-case` instead of leaking heading text through internal expectation columns.
- Added a CLI output regression test for the `sentence-case` public ID contract and kept `--version` release coverage intact.

## 0.2.3

- Added `universalizing-claims` as the next accumulative `llm-slop` rule for repeated broad-human framing such as `everyone wants...`, `we all want...`, and `most people know...`.
- Kept the new rule synthetic-first for now: the existing fixture sidecar baselines stayed intact and no prior expected failures disappeared.
- Extended the shipped config surface and presets with `universalizingClaims` while keeping the full workspace and packaged CLI green.

## 0.2.2

- Tightened `softening-language` so a weaker `may + often` sentence no longer fails on its own.
- Kept the stronger Medical Outline softening hits intact while dropping the borderline `why_do_we_dream` baseline.

## 0.2.1

- Added `softening-language` as the next accumulative `llm-slop` rule for repeated low-commitment phrasing such as modal-heavy variability disclaimers and tentative reporting.
- Expanded the fixture regression sidecars to cover the new real-world hits in `why_do_we_dream` and the Medical Outline corpus.
- Kept the packaged CLI release contract intact while bumping the workspace to the next semantic version.

## 0.2.0

- Added the `llm-slop` family with the first live rules for:
  - `llm-disclaimer`
  - `response-wrapper`
  - `generic-signposting`
  - `boilerplate-framing`
  - `llm-vocabulary`
- Split the old heuristic bucket into smaller owned rule families and aligned their sidecar/assertions test structure with the GuardRails test architecture.
- Added per-fixture expected-failure sidecars for the growing corpus, including the new Medical Outline AI-written fixture set.
- Standardized release versioning across the workspace and now enforce `prosesmasher --version` through packaged smoke coverage.

## 0.1.7

- Preserve visible text inside raw HTML embedded in markdown instead of dropping it. This fixes aside-heavy content copied from tools like Notion.
- Narrow the em-dash heuristic so it flags only closed em dashes like `word—word`, while allowing spaced forms like `word — word`.
- Reclassify `sentence-case` as a general quality heuristic while keeping its current heading-targeted behavior.
- Clarify internal crate metadata on crates.io and tighten the release/distribution plumbing around the public `prosesmasher` package.

## 0.1.6

- Refreshed the GitHub-facing and crates.io-facing README copy after the public repo launch.
- Tightened the opener and command walkthrough text so the docs read like product documentation instead of generic tool boilerplate.
- Removed the stale `CLAUDE.md` repo note from version control.

## 0.1.5

- Extended `negation-reframe` with a narrow internal-state corrective branch so it catches patterns like:
  - `don't stop having feelings`
  - `they stop showing them`
- Kept that branch narrow enough to avoid generic behavioral follow-up pairs such as `Children don't stop at the corner. They turn left instead.`

## 0.1.4

- Added a new `fragment-stacking` heuristic to catch clipped cadence runs such as `Short. Short. Longer payoff.` without emitting overlapping duplicate hits.
- Broadened affirmation-closer detection to catch short standalone `That's the ...` formula sentences.
- Tightened corrective-negation detection:
  - still catches `Not to X. To Y.`
  - still catches `This isn't X. It's Y.`
  - now also catches curated same-root framing pairs like `does not mean -> it means`
  - avoids broad generic same-root matches outside the framing verb family
- Updated the `substack-en` preset word-count envelope to `500..1000`.

## 0.1.3

- Realigned the canonical config taxonomy with the public check families:
  - `quality.heuristics` for rhetorical/style heuristics
  - `quality.flow` for paragraph and repetition controls
  - `quality.readability` for readability thresholds
- Updated the domain config model, JSON loader, full config example, shipped preset assets, fixtures, and docs to use the new shape.
- Removed the remaining drift where flow/readability settings were still nested under `quality.heuristics`.

## 0.1.2

- Added clean machine-readable CLI semantics:
  - exit `0` for success
  - exit `1` for check failures
  - exit `2` for operational failures
- JSON mode now keeps stdout pure JSON on check failures and keeps stderr empty in that case.
- Added `schema_version`, `exit_reason`, and failure/check `kind` fields to JSON output.
- Added `check --list-checks` with check IDs, groups, default-enabled state, and supported locales.
- Added text output modes: `failures`, `full`, `summary`, and `paths`, with failure-focused text as the default.
- Expanded the published README with install, presets, config semantics, CI usage, limitations, and audience guidance.
- Added packaged smoke tests to catch published-asset regressions.

## 0.1.1

- Fixed published `cargo install` builds so preset-backed commands work:
  - `check --preset ...`
  - `dump-config --preset ...`
  - `dump-config --full-config`
- Embedded shipped preset JSON into the published crate graph instead of relying on monorepo-relative asset paths.
- Added stricter CLI exit semantics:
  - `0` for success
  - `1` for check failures
  - `2` for operational failures
- JSON mode now keeps stdout machine-readable on lint failures and reports `schema_version`, `exit_reason`, and failure `kind`.
- Added `check --list-checks`.
- Added text output modes: `failures`, `full`, `summary`, `paths`.
- Tightened publish metadata for the internal crate graph.

## 0.1.0

- Initial crates.io release.
- Included the core prose validation engine, shipped presets, JSON output, and document-policy checks.
- Preset-backed commands in the published package were broken because preset assets were not packaged correctly. This is fixed in `0.1.1`.
