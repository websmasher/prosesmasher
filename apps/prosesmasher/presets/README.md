# Presets and Examples

These presets are curated starting points for English prose workflows.

They are intended for:
- generated first drafts
- editorial review loops
- CI-style prose validation

They are not universal truth. They are opinionated defaults that you should copy and tune for your domain.

The library already carries the default anti-slop lexicon, simplicity pairs, repetition exclusions, and baseline quality thresholds for English. The shipped presets differ only by `documentPolicy`. Quality stays on the shared library defaults everywhere.

## Example Configs

- `examples/full-config-en.json` — canonical reference showing the full config surface, including every major lexical, heuristic, and document-policy option

Use the example config to see what is possible.
Use the shipped presets when you want a real starting point without overcommitting to debatable policy choices.

## Presets

- `general-en.json` — baseline quality defaults with no document-shape policy
- `article-en.json` — standard article structure
- `substack-en.json` — longer, looser newsletter/article structure
- `email-en.json` — short prose body with no heading policy
- `tweet-en.json` — very short prose body with no heading policy

## Notes

- These presets are English-first. Many phrase/pattern lists are explicitly English.
- `requiredTerms`, `recommendedTerms`, and substring rules are left empty in the presets because those are domain-specific.
- Copy a preset into your own config and then customize:
  - prohibited terms for your organization
  - required terminology
  - threshold strictness
  - locale-specific term lists
