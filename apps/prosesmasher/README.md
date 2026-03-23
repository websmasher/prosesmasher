# prosesmasher workspace

`prosesmasher` is a deterministic prose-quality validator for markdown and short-form text.

This workspace contains the publishable crate graph behind the CLI:

- `prosesmasher` — user-facing installable binary
- `prosesmasher-adapters-inbound-cli` — CLI adapter and composition root
- `prosesmasher-app-core` — prose-quality checks and validation runner
- `prosesmasher-adapters-outbound-fs` — filesystem-backed file/config loading
- `prosesmasher-adapters-outbound-parser` — markdown parsing and text segmentation
- `prosesmasher-domain-types` — domain data structures and config types
- `prosesmasher-ports-outbound-traits` — port traits for adapters

The CLI validates prose with deterministic lexical checks, heuristic anti-slop checks, readability checks, and optional document-policy constraints.

Typical usage:

```bash
prosesmasher list-presets
prosesmasher dump-config --full-config
prosesmasher check draft.md --preset article-en
prosesmasher check draft.md --config my-config.json --format json
```
