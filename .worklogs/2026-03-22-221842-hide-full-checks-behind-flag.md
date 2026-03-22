# Hide Full Checks Behind Flag

**Date:** 2026-03-22 22:18
**Scope:** `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/args_tests.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs`, `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs`

## Summary
Changed JSON output so the default contract is failure-focused and omits the full `checks` array. Added `--include-checks` to opt back into the full per-check payload when needed, and updated the CLI help and tests accordingly.

## Context & Problem
After running the validator on real `steady-parent` articles, the user called out that the JSON output was still too verbose for practical rewrite-loop use. The main source of noise was the full `checks` array, which duplicated the important information already available in `failures` and made the payload much larger than necessary.

The desired behavior is: default JSON should contain only the actionable failure-oriented structure, and everything else should be hidden behind an explicit flag.

## Decisions Made

### Hide `checks` by default in JSON output
- **Chose:** Make `FileResult.checks` optional and omit it unless the caller explicitly asks for it.
- **Why:** The rewrite loop only needs the compact failure contract by default. The full array is useful for debugging, but it should not be the default product surface.
- **Alternatives considered:**
  - Keep `checks` always present — rejected because it keeps the default payload too large and noisy.
  - Remove `checks` entirely — rejected because there is still value in exposing the full check matrix for debugging and inspection.

### Add an explicit `--include-checks` flag
- **Chose:** Add `--include-checks` to the `check` command and wire it through to JSON output generation.
- **Why:** This is a direct and readable switch: compact output by default, full detail on demand.
- **Alternatives considered:**
  - Add a separate JSON mode like `--format full-json` — rejected because the output format is still JSON in both cases; only the verbosity changes.
  - Reuse an existing generic verbosity flag — rejected because the CLI did not already have one, and the behavior change is specific to JSON structure.

### Keep text output unchanged
- **Chose:** Leave text mode alone for now.
- **Why:** The user's complaint was specifically about the JSON rewrite-loop payload. Text mode remains a separate UX surface.
- **Alternatives considered:**
  - Collapse text output too — rejected because that is a separate UX decision and should not be bundled into the JSON contract change.

## Architectural Notes
`FileResult.checks` is now `Option<Vec<CheckOutput>>` with `skip_serializing_if`, which keeps the serialized shape compact without changing the rest of the failure-oriented contract. The inclusion decision is made in the CLI layer and threaded into `build_file_result`, so the app/core validation engine remains untouched.

This keeps the default machine-readable contract centered on:
- summary
- rewrite flags and instructions
- failures

while preserving the full pass/fail matrix as an explicit opt-in.

## Information Sources
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — current JSON output contract
- `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs` — CLI wiring point for output behavior
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs` — command surface and help text
- Real CLI runs on `/Users/tartakovsky/Projects/steady-parent/apps/landing/content/blog/posts/en/adhd-sensory-asd-signs.mdx`

## Open Questions / Future Considerations
- The top-level JSON still includes both scalar summary fields and the nested `summary` object. If payload minimization continues, that duplication may be worth revisiting.
- Text output may eventually want a similar “failures only” mode, but that should be handled separately.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output.rs` — JSON contract and conditional `checks` serialization
- `apps/prosesmasher/crates/adapters/inbound/cli/src/main.rs` — how `--include-checks` is threaded through
- `apps/prosesmasher/crates/adapters/inbound/cli/src/args.rs` — user-facing flag and help text
- `apps/prosesmasher/crates/adapters/inbound/cli/src/output_tests.rs` — regression coverage for hidden vs included checks
- `.worklogs/2026-03-22-213522-clean-output-contract.md` — earlier output cleanup context

## Next Steps / Continuation Plan
1. Run the compact JSON output through the actual rewrite loop and confirm the payload is now the right size and shape for the next prompt stage.
2. If the payload is still too verbose, decide whether to remove duplicated top-level summary fields and keep only one summary representation.
3. Separately address MDX component noise in `paragraph-length`, which is now one of the last obvious sources of low-value failures on real article runs.
