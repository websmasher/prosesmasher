# prosesmasher

Deterministic prose linting for markdown and short-form text.

`prosesmasher` is a CLI for deterministic prose linting to root out AI (and human) slop. It is built for teams that want repeatable prose checks in CI, editor tooling, and content pipelines. The tool is English-first and intentionally opinionated about AI-style boilerplate, readability, and markdown structure.

It covers four broad areas:

- lexical policy such as prohibited, required, and recommended terms
- rhetorical heuristics for stock phrasing and corrective boilerplate
- flow and readability checks
- optional document policy such as word count, headings, and code fences

## What it is for

Use `prosesmasher` when you want:

- deterministic prose checks in CI
- a shared house style for docs, articles, emails, or social copy
- machine-readable failure output for content pipelines
- a way to catch familiar AI-slop patterns without calling an LLM

Use something else when you want:

- grammar correction
- fact checking
- semantic review by an LLM
- a universal oracle for “good writing”

This tool is a policy engine. Some checks are widely useful. Some are house-style on purpose.

## Install

From source via Cargo:

```bash
cargo install prosesmasher
```

Fast binary install with `cargo-binstall`:

```bash
cargo binstall prosesmasher
```

## Quickstart

List shipped presets:

```bash
prosesmasher list-presets
```

Dump a shipped preset:

```bash
prosesmasher dump-config --preset article-en
```

Dump the full editable config:

```bash
prosesmasher dump-config --full-config
```

Check one file with a preset:

```bash
prosesmasher check draft.md --preset article-en
```

Check a directory recursively with your own config:

```bash
prosesmasher check content/ --config prosesmasher.json
```

Emit machine-readable JSON:

```bash
prosesmasher check draft.md --config prosesmasher.json --format json
```

List checks instead of validating files:

```bash
prosesmasher check --list-checks
prosesmasher check --list-checks --group readability --format json
```

## Command surface

Top-level commands:

- `check`
- `list-presets`
- `dump-config`

`check` supports:

- `--preset <name>` or `--config <path>` (mutually exclusive)
- `--group <group>`
- `--check <id,id,id>`
- `--format text|json`
- `--text-mode failures|full|summary|paths`
- `--include-checks`
- `--list-checks`

Examples:

```bash
prosesmasher check draft.md --preset article-en
prosesmasher check drafts/ --preset substack-en --group quality
prosesmasher check draft.md --config prose.json --format json
prosesmasher check draft.md --config prose.json --format json --include-checks
prosesmasher check draft.md --preset article-en --text-mode summary
prosesmasher check draft.md --preset article-en --text-mode paths
```

## Presets

Shipped presets:

- `general-en`
- `article-en`
- `substack-en`
- `email-en`
- `tweet-en`

Preset philosophy:

- presets share the same quality defaults
- presets differ mainly in structural document policy
- `dump-config --full-config` is the full editable schema example, not a preset

## Config model

Config uses camelCase JSON keys.

Top-level shape:

```json
{
  "locale": "en",
  "quality": {
    "lexical": {},
    "heuristics": {},
    "flow": {},
    "readability": {}
  },
  "documentPolicy": {}
}
```

Important semantics:

- `quality.lexical` is mostly override and merge driven
- `quality.heuristics` holds rhetorical and style heuristics
- `quality.flow` holds paragraph and repetition controls
- `quality.readability` holds readability thresholds
- `documentPolicy` is opt-in structural policy
- override lists use `defaults`, `add`, and `remove`
- `defaults: true` merges with built-in defaults
- `defaults: false` replaces built-in defaults
- recommended-term pools use `allowInflections` to control stem matching

Minimal example:

```json
{
  "locale": "en",
  "quality": {
    "lexical": {
      "prohibitedTerms": {
        "defaults": true,
        "add": ["live coaching calls"],
        "remove": []
      }
    },
    "flow": {
      "paragraphLength": {
        "maxSentences": 6
      }
    }
  },
  "documentPolicy": {
    "wordCount": {
      "min": 500,
      "max": 1000
    }
  }
}
```

## Output

Text mode defaults to failure-focused output.

Text modes:

- `failures` prints only failing checks plus a summary
- `full` prints every check plus a summary
- `summary` prints one summary line per file
- `paths` prints only failing file paths

JSON mode is for machine consumers:

- stdout contains only JSON
- stderr stays empty on check failures
- exit `0` means success
- exit `1` means check failures
- exit `2` means operational failure

JSON includes:

- `schema_version`
- `exit_reason`
- summary counts
- failed checks with evidence
- rewrite hints
- optional `checks` when `--include-checks` is set

Example:

```bash
prosesmasher check draft.md --config prose.json --format json
```

## Check families

- `lexical` for prohibited, required, recommended, and simplicity policy
- `heuristics` for anti-slop and stylistic house rules
- `flow` for paragraph length and repetition
- `readability` for Flesch-Kincaid, Gunning Fog, Coleman-Liau, and average sentence length
- `document-policy` for structural markdown rules

Discover the exact shipped checks from the CLI:

```bash
prosesmasher check --list-checks
```

## Language support

The tool is English-first today.

- English has the strongest heuristic and lexical support
- parser and locale plumbing exist beyond English
- heuristic coverage outside English is partial

If you need non-English checking, inspect `check --list-checks` and your config instead of assuming feature parity.

## CI use

Fail a job on prose violations:

```bash
prosesmasher check content/ --preset article-en
```

Store JSON for another tool:

```bash
prosesmasher check content/ --config prose.json --format json > result.json
```

Keep noisy logs short:

```bash
prosesmasher check content/ --preset article-en --text-mode paths
```

## Limits

- This tool does not rewrite text for you.
- It does not fact check.
- It does not try to be a neutral judge of all prose.
- False positives are possible because several checks are deliberately opinionated.
- English support is much stronger than support for other locales.

## Repo layout

- `apps/prosesmasher/` is the Rust workspace
- `crates/` holds the hexagonal app structure
- `packages/prosesmasher/` is the installable wrapper crate

## License

MIT
