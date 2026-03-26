# Slop Taxonomy And NLP Follow-Up Plan

## Purpose

Capture the useful takeaways from the paper **“Measuring AI ‘Slop’ in Text”** and turn them into a practical future plan for `prosesmasher`.

This is not an implementation plan for the next commit. It is a design note for future work:
- which parts of the paper are actually actionable for deterministic detection
- which parts are conceptually useful but not yet operational
- where a lightweight NLP library such as `nlprule` would help
- which new rule families are worth building from that

Paper:
- https://arxiv.org/pdf/2509.19163

## Main Takeaways

### 1. “Slop” is not one metric

The paper treats slop as a composite over multiple dimensions rather than a single label. The useful high-level buckets are:
- information utility
- information quality
- style quality

For `prosesmasher`, that reinforces the current architecture:
- many interpretable checks
- grouped into owned families
- with concrete evidence

This argues against:
- one monolithic slop score
- one classifier replacing the current rule system

### 2. Span-level evidence is more useful than global judgment

The paper shows binary slop judgments are subjective, but span-level annotations and code-level explanations are much more informative.

That aligns well with:
- our per-check evidence output
- fixture sidecars storing expected failed rules plus evidence fragments
- rule assertions built around specific public-output expectations

### 3. Domain matters

The paper finds different dimensions matter more in different domains:
- news: relevance, density, tone, coherence
- QA: factuality and structure matter more

For `prosesmasher`, this suggests:
- presets and modes should matter more over time
- not every rule should be equally strict for every writing mode

### 4. Full automation is still weak

The paper is explicit that:
- current automatic metrics do not fully capture slop
- LLM-as-judge performs poorly for binary slop labeling
- LLM span extraction is also weak

That is an argument to keep `prosesmasher`:
- deterministic
- interpretable
- evidence-first

Not to turn it into:
- a remote LLM judge
- a black-box classifier

## What Is Actually Actionable For Us

The paper’s strongest practical signals for our tool are:
- repetition
- templatedness
- density / low-information padding
- structural scaffolding

The less actionable ones for a deterministic CLI right now are:
- relevance
- coherence
- tone
- factuality

Those are real slop dimensions, but the paper itself does not provide a reliable lightweight automatic recipe for them.

So future work should favor:
- local structural detectors
- local rhetorical detectors
- cheap document-level aggregate thresholds

Not:
- opaque global scoring
- LLM-based judging

## Proposed Future Dimensions Mapping

These dimensions should eventually exist as internal metadata on checks, even if they do not change behavior yet.

- `density`
  Low-information padding, filler, saying little with many words
- `repetition`
  Repeated words, repeated phrase families, repeated transitions
- `templatedness`
  Repeated shallow structures or list/setup formulas
- `structure`
  Formulaic answer scaffolds, bullet/list overuse, repeated staging
- `tone`
  Flattened assistant voice, generic outside-observer posture
- `quality`
  Disclaimer leakage, service-wrapper language, factuality proxies where available

This should be treated as a reporting axis, not a replacement for families/check IDs.

## Candidate Future Rules

### 1. Template Repetition

**Goal**
- Catch repeated shallow sentence skeletons, not just repeated words

**Why**
- This is one of the best paper-backed additions that can still be deterministic
- It covers templatedness better than current lexical repetition alone

**Rule idea**
- Within a paragraph or short window, normalize sentences to a shallow template
- Flag when the same template appears repeatedly

Examples of possible templates:
- `PRON VERB NOUN`
- `SUBJECT COPULA ADJ`
- `PERSON , ROLE , VERB THAT ...`

**Detection sketch**
- tokenize
- POS tag
- collapse to coarse tags or chunk templates
- compare repeated templates in a sliding window

**Why this matters**
- catches repetitive structure even when lexical items vary

### 2. Transition Template Overuse

**Goal**
- Go beyond exact rhetorical phrases and catch families of transitional scaffolds

**Why**
- The paper explicitly calls out repeated transitional phrases as part of repetition/templatedness

**Rule idea**
- detect repeated discourse templates such as:
  - setup clause + broad claim
  - transition phrase + safety advice
  - abstract staging clause + explanation clause

This is a more structural successor to:
- `generic-signposting`
- `boilerplate-framing`

### 3. Low Information Density

**Goal**
- Catch prose that is verbose but semantically thin

**Why**
- Information density is one of the strongest overall predictors in the paper

**Rule idea**
- paragraph-level or document-level heuristic combining:
  - high function-word ratio
  - high filler-phrase density
  - low concrete noun / content-word density
  - repeated abstract framing

**Important**
- This should not be a first-pass rule if it is too noisy
- It likely needs to begin as a warning-oriented aggregate signal

### 4. Formulaic List Scaffolding

**Goal**
- Detect repeated “here are factors / some examples include / the following reasons” scaffolds at a broader structural level

**Why**
- This is very common in AI-written informational content
- It fits the paper’s templatedness and structure buckets

**Relation to existing rules**
- probably extends or supersedes parts of `boilerplate-framing`

### 5. Bullet/List Templatedness

**Goal**
- Detect excessive bulletized formula writing where list items share the same shallow template

**Why**
- The paper specifically mentions predictable formatting patterns and excessive bullet points under templatedness

**Rule idea**
- only evaluate when a document has list-heavy content
- compare adjacent list items for repeated syntactic pattern

This is not urgent, but it is a good fit for the taxonomy.

## NLP Library Proposal: `nlprule`

## Why `nlprule` is the best current candidate

If we add NLP support, the most plausible option is:
- `nlprule`

Reason:
- Rust-native
- lightweight enough to be realistic for a CLI
- exposes the kinds of annotations useful for deterministic rule writing:
  - tokenization
  - lemmas
  - POS tags
  - chunks / rule-based matching support

This is a much better fit than:
- `spaCy`
  Python runtime + model distribution overhead
- `stanza`
  heavier pipeline/model setup than we want
- `LanguageTool`
  wrong abstraction and too heavy for this use case

## What We Would Use `nlprule` For

### A. POS-template repetition
- repeated sentence skeletons
- repeated list-item skeletons
- repeated clause scaffolds

### B. Better template-family matching
- match families like:
  - collective subject + desire verb
  - setup clause + abstract claim
  - list preface + category noun + intro verb

This would let us reduce brittle exact word dependence without introducing a black-box model.

### C. Optional future density proxies
- rough content-word vs function-word ratios
- chunk-based proposition density approximations

This is lower priority than template repetition.

## What We Should Not Use NLP For Yet

- global coherence scoring
- relevance scoring
- factuality scoring
- tone scoring

Those remain either:
- too expensive
- too noisy
- too model-dependent

for the current CLI architecture.

## Recommended Build Order

### Phase 1: Metadata and Taxonomy
- add an internal `dimension` tag to checks
- do not change output or behavior yet
- use it only for internal organization and future reporting

### Phase 2: `nlprule` Spike
- create a small experimental branch or dev-only module
- benchmark startup time, binary size impact, and runtime overhead
- prove we can get stable POS/chunk output on real fixture corpora

### Phase 3: One Real Rule
- implement only one `nlprule`-powered rule first:
  - `template-repetition`

This is the strongest candidate because:
- paper-backed
- deterministic
- explainable
- clearly distinct from current lexical repetition

### Phase 4: Extend Existing LLM-Slop Rules Structurally
- use shallow templates to broaden:
  - `generic-signposting`
  - `boilerplate-framing`
  - `universalizing-claims`

### Phase 5: Consider Density Aggregate
- only after real-corpus review
- only if noise stays manageable

## Success Criteria For Any NLP Addition

Before we accept NLP in the core CLI, it should satisfy all of these:
- startup overhead remains acceptable
- binary size growth is reasonable
- rule behavior is interpretable
- evidence output remains concrete
- false-positive rate is low on existing fixtures
- it catches meaningful cases the current surface heuristics miss

If it fails those, do not add it.

## Good Immediate Next Research Tasks

1. Evaluate `nlprule` as a dependency spike in a throwaway prototype
2. Design `template-repetition` in the same runtime/assertions/sidecar architecture
3. Map every current check to a future `dimension` field
4. Add a corpus specifically rich in generic self-help / marketing sludge, because current fixtures are better for disclaimer/wrapper slop than for universalizing/templatedness patterns

## Explicit Non-Goals

This plan is **not** proposing:
- replacing rules with an LLM judge
- replacing evidence with a single slop score
- importing a heavy Python NLP stack into the main CLI
- adding vague “quality metrics” that do not change detection in an interpretable way

The point is to add only signals that can become:
- deterministic rules
- with evidence
- in the current family architecture
