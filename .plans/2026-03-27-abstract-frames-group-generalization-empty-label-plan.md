# Abstract Frames, Group Generalization, And Empty Moral Labels

## Goal
Close the next three reviewed slop segments without broadening into generic high-quality explanatory prose:

1. abstract evaluation/meta frames
2. group-generalization openers
3. empty moral relabels

## Segment 1: Abstract Evaluation / Meta Frames

### Reviewed stems
- `The result worth caring about ...`
- `The bigger win is ...`
- `The useful alternatives ...`
- `The point is to ...`
- `The better move is ...`
- `What matters most ...`
- `What matters is ...`
- `What helps is ...`

### Rule home
- `slop_03_generic_signposting`

### Matcher shape
- construction-first, not one-off phrase bag
- small families:
  - `the + modifier? + abstract_noun + is`
  - `the + result + worth caring about + is`
  - `the + point + is + to`
  - `what + matters|helps + (most)? + is`

### Constraints
- keep the family curated; do not allow arbitrary nouns or verbs
- prefer short meta/compression frames
- explicitly avoid turning this into “any sentence beginning with `the point is` is bad”

### Review gate
- inspect additive hits in explainers and short-form corpora
- accept only lines that are clearly meta-framing rather than useful technical instruction

## Segment 2: Group-Generalization Openers

### Reviewed stem
- `Most parents keep reaching ...`

### Rule home
- `slop_07_universalizing_claims`

### Matcher shape
- subject family:
  - `most + human plural noun`
- continuation family:
  - broad behavior verbs like `keep`, `reach`, `go`, `fall`, `end up`

### Constraints
- only sentence-leading
- only reviewed human-group nouns
- do not generalize to any `most + noun`
- do not absorb concrete population statements like `most patients improved`

### Review gate
- start narrow
- synthetic-first unless real corpus additions are obviously good

## Segment 3: Empty Moral Relabels

### Reviewed stem
- `That is discipline.`

### Rule home
- `slop_09_empty_emphasis`

### Matcher shape
- deictic subject:
  - `that`
  - `this`
- copula:
  - `is`
- virtue label family:
  - start with `discipline`

### Constraints
- keep exact/narrow until more reviewed labels appear
- do not broaden to arbitrary abstract nouns yet

### Review gate
- synthetic-first
- only accept real-corpus additions if they are clearly empty relabels, not concise useful definitions

## Execution Order
1. Implement Segment 1 in `generic-signposting` and review additive corpus hits.
2. Implement Segment 2 in `universalizing-claims` and review additive hits.
3. Implement Segment 3 in `empty-emphasis` and review additive hits.
4. Run:
   - targeted tests
   - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
   - generated compares per model
   - explainer compare
   - social compare
5. Bump version, update changelog, write worklog, commit, push.
