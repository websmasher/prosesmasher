# Presets

These presets are curated starting points for English prose workflows.

They are intended for:
- generated first drafts
- editorial review loops
- CI-style prose validation

They are not universal truth. They are opinionated defaults that you should copy and tune for your domain.

The library already carries the default anti-slop lexicon, simplicity pairs, repetition exclusions, and baseline quality thresholds for English. The shipped presets mostly add document-shape policy and a few threshold deltas instead of restating those defaults.

## Presets

- `general-en.json` — baseline quality defaults with no document-shape policy
- `article-short-en.json` — short-form article shape
- `article-medium-en.json` — standard article/blog shape
- `article-long-en.json` — long-form article shape
- `docs-en.json` — docs/reference-style prose
- `landing-page-en.json` — concise marketing/site copy

## Notes

- These presets are English-first. Many phrase/pattern lists are explicitly English.
- `requiredTerms`, `recommendedTerms`, and substring rules are left empty in the presets because those are domain-specific.
- Copy a preset into your own config and then customize:
  - prohibited terms for your organization
  - required terminology
  - threshold strictness
  - locale-specific term lists
