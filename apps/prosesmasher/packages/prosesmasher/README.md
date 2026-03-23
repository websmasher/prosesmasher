# prosesmasher

Deterministic prose-quality validator for markdown and short-form text.

`prosesmasher` is for teams that want repeatable prose linting in CI, editor tooling, and content pipelines without calling an LLM. It focuses on:

- lexical policy such as prohibited, required, and recommended terms
- built-in anti-slop heuristics for generic AI-style phrasing
- readability and flow checks
- optional markdown document-policy checks such as word count and heading structure

It is not a generic truth source for “good writing.” Many checks are intentionally opinionated house-style heuristics, especially the AI-tell and boilerplate detectors.

## Audience

`prosesmasher` is aimed at:

- content teams with a house style
- markdown-heavy docs and blog workflows
- CI linting for generated or edited prose
- tooling that needs machine-readable prose failures

## Language Support

The tool is English-first today.

- English has the strongest default lexical policy and heuristic support.
- Other locales exist in the types and parser pipeline, but heuristic coverage is partial.
- `check --list-checks` shows locale-specific restrictions for checks such as `fake-timestamps`.

## Install

```bash
cargo install prosesmasher
```

## Quickstart

List shipped presets:

```bash
prosesmasher list-presets
```

Inspect a preset profile:

```bash
prosesmasher dump-config --preset article-en
```

Print the full editable config example:

```bash
prosesmasher dump-config --full-config
```

Run validation with a shipped preset:

```bash
prosesmasher check draft.md --preset article-en
```

Run validation with your own config:

```bash
prosesmasher check draft.md --config prosesmasher.json --format json
```

List all available checks:

```bash
prosesmasher check --list-checks
prosesmasher check --list-checks --group readability --format json
```

## Presets vs Full Config

- Presets are partial policy profiles. They keep shared quality defaults and mainly differ in structural document policy.
- `--full-config` is the full editable schema example, not a preset.

Current shipped presets:

- `general-en`
- `article-en`
- `substack-en`
- `email-en`
- `tweet-en`

## Config Model

Config uses camelCase JSON keys.

Top-level shape:

```json
{
  "locale": "en",
  "quality": {
    "lexical": {},
    "heuristics": {}
  },
  "documentPolicy": {}
}
```

Important semantics:

- `quality.lexical` is mostly merge-driven override policy
- `quality.heuristics` contains rhetorical and style heuristics
- `quality.flow` contains paragraph and repetition controls
- `quality.readability` contains readability thresholds
- `documentPolicy` is opt-in; omitted fields stay off
- override lists use `defaults`, `add`, and `remove`
- `defaults: true` means merge with built-in defaults
- `defaults: false` means replace built-in defaults
- recommended-term pools use `allowInflections` to control stem matching behavior

Example:

```json
{
  "locale": "en",
  "quality": {
    "lexical": {
      "prohibitedTerms": {
        "defaults": true,
        "add": ["live coaching calls"],
        "remove": ["actually"]
      }
    },
    "heuristics": {
      "llmOpeners": {
        "enabled": true
      }
    },
    "flow": {
      "paragraphLength": {
        "maxSentences": 6
      }
    },
    "readability": {
      "avgSentenceLengthMax": 24
    }
  },
  "documentPolicy": {
    "wordCount": {
      "min": 1000,
      "max": 2200
    }
  }
}
```

## CLI Modes

Text mode defaults to failure-focused output.

```bash
prosesmasher check draft.md --preset article-en
prosesmasher check draft.md --preset article-en --text-mode full
prosesmasher check draft.md --preset article-en --text-mode summary
prosesmasher check docs/ --preset article-en --text-mode paths
```

Available text modes:

- `failures` — only failing checks plus a summary
- `full` — every check plus a summary
- `summary` — one summary line per file
- `paths` — only failing file paths

## JSON Contract

When `--format json` is used:

- stdout contains only JSON
- stderr stays empty for check failures
- exit code `1` means check failures only
- exit code `2` means operational failure such as bad args, config, IO, or parse errors

The JSON payload includes:

- `schema_version`
- `exit_reason`
- summary counts
- per-failure evidence
- rewrite hints
- optional `checks` array behind `--include-checks`

Example:

```bash
prosesmasher check draft.md --config prosesmasher.json --format json
```

## CI Usage

Fail the job on prose violations:

```bash
prosesmasher check content/ --preset article-en
```

Consume machine-readable output:

```bash
prosesmasher check content/ --config prosesmasher.json --format json > result.json
```

Use path-only text output in noisy CI logs:

```bash
prosesmasher check content/ --preset article-en --text-mode paths
```

## Check Families

- `lexical` — prohibited, required, recommended, and simplicity policy
- `heuristics` — anti-slop and stylistic house rules
- `flow` — paragraph-length and repetition checks
- `readability` — Flesch-Kincaid, Gunning Fog, Coleman-Liau, average sentence length
- `document-policy` — structural markdown rules

Discover them from the CLI:

```bash
prosesmasher check --list-checks
```

## Philosophy And Limitations

- The tool is deterministic. It does not judge with an LLM.
- Several checks are intentionally opinionated and reflect house-style policy, not universal writing truth.
- English support is much stronger than other locales right now.
- False-positive suppression is still limited; tune presets and custom config accordingly.
- For machine consumers, prefer JSON mode and treat `exit_reason` as the stable high-level outcome.

## Changelog

See [`../../CHANGELOG.md`](../../CHANGELOG.md) for release notes.
