# Presets

These presets are curated starting points for English prose workflows.

They are intended for:
- generated first drafts
- editorial review loops
- CI-style prose validation

They are not universal truth. They are opinionated defaults that you should copy and tune for your domain.

## Presets

- `general-en.json` — balanced default for general web prose
- `blog-strict-en.json` — tighter style for editorial/blog writing
- `technical-article-en.json` — longer technical explainers and tutorials
- `docs-en.json` — product/docs/reference-style prose
- `landing-page-en.json` — concise marketing/site copy
- `essay-en.json` — longer argument-driven prose with a bit more structural freedom

## Notes

- These presets are English-first. Many phrase/pattern lists are explicitly English.
- `requiredTerms` and `recommendedTerms` are left empty in the presets because those are domain-specific.
- Copy a preset into your own config and then customize:
  - banned terms for your organization
  - required terminology
  - threshold strictness
  - locale-specific term lists
