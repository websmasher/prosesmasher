# Summary

Migrated Rust persona-signals checks into the textlint phrase family. Added `humble-bragger` and `jargon-faker`, phrase fixture coverage, manifest rows, and corpus comparison reports.

# Decisions Made

- Placed both rules under `phrases` because Rust detects fixed multi-word phrases in sentences.
- Tightened `jargon-faker` beyond Rust by skipping literal technical targets such as `debug your build script` and `optimizing for latency`.
- Kept corpus comparison reports even though the current fixture corpus has zero real findings for this family.

# Key Files

- `.plans/2026-05-13-002825-migrate-persona-signals-textlint.md`
- `.plans/2026-05-13-003341-persona-signals-rust-good-catches.md`
- `.plans/2026-05-13-003341-persona-signals-textlint-diff.md`
- `packages/textlint-rules/src/families/phrases/humble-bragger.ts`
- `packages/textlint-rules/src/families/phrases/jargon-faker.ts`
- `behavior/fixtures/textlint-rules/phrases/family.md`

# Verification

- `npm run validate`
- `./scripts/behavior-verify.sh`
- `python3 scripts/verify-textlint-implementation-manifest.py --manifest .plans/2026-05-12-203442-textlint-implementation-plan.md.manifest.toml all`
- Rust/textlint corpus comparison across fixture files: both found 0 `humble-bragger` and 0 `jargon-faker` findings.

# Next Steps

- Migrate the next Rust-backed family.
