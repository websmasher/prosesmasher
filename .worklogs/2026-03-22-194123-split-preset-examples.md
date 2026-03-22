# Split preset examples from shipped presets

**Date:** 2026-03-22 19:41
**Scope:** `apps/prosesmasher/presets/README.md`, `apps/prosesmasher/presets/examples/full-config-en.json`

## Summary
Added a dedicated full-surface example config and clarified the preset directory structure so example files and real shipped presets are no longer mixed conceptually. The real presets remain minimal and opinionated; the example file shows the entire canonical config shape.

## Context & Problem
After rationalizing the shipped presets, there was still one remaining source of confusion: users needed both
- a real preset set that avoided overcommitting to debatable policy choices
- and a single reference file showing all available config options

Keeping those concerns mixed in one preset set would pull the real shipped configs back toward verbose “show every option” JSON, which is exactly what we had just cleaned up.

## Decisions Made

### Add one explicit full-config example instead of bloating real presets
- **Chose:** Create `apps/prosesmasher/presets/examples/full-config-en.json` as the canonical full-surface example.
- **Why:** One explicit example file is enough to document the schema without forcing every real preset to restate defaults or edge-case options.
- **Alternatives considered:**
  - Expand `general-en.json` to show everything — rejected because it would stop being a real defaults-only preset.
  - Add multiple example configs for each subsystem — rejected because that would fragment the reference surface unnecessarily.

### Keep the README explicit about the difference
- **Chose:** Update `apps/prosesmasher/presets/README.md` to distinguish between example configs and shipped presets.
- **Why:** Without that note, future edits would likely drift back toward using real presets as documentation dumps.
- **Alternatives considered:**
  - Leave the split implicit in the directory tree — rejected because the difference is semantic, not just structural.

## Architectural Notes
This keeps the preset surface clean:
- shipped presets = minimal starting points
- example configs = schema reference

That fits the canonical-only config model we already settled on. It also reinforces the earlier decision that the library owns the default English quality baseline, while presets and examples serve different jobs:
- presets choose a starting policy
- examples show the config vocabulary

## Information Sources
- `.worklogs/2026-03-22-192945-rationalize-presets.md` — preset simplification immediately before this split
- `apps/prosesmasher/presets/README.md` — preset-facing documentation
- `apps/prosesmasher/crates/domain/types/src/config.rs` — canonical config shape used to build the example
- Verification run:
  - `cargo test -q -p prosesmasher-adapters-outbound-fs`

## Open Questions / Future Considerations
- If we later add `--preset` CLI support, we should decide whether example configs remain in the same directory tree or move to a separate docs/examples location to avoid any discovery ambiguity.
- The full example is English-focused because that is where built-in defaults exist today.

## Key Files for Context
- `apps/prosesmasher/presets/examples/full-config-en.json` — full canonical config example
- `apps/prosesmasher/presets/README.md` — explains example-vs-preset split
- `.worklogs/2026-03-22-192945-rationalize-presets.md` — prior preset taxonomy cleanup

## Next Steps / Continuation Plan
1. Decide whether the CLI should support `--preset <name>` now that the preset surface is small and the example split is explicit.
2. If preset selection is added, make sure discovery ignores `presets/examples/` and only treats the real top-level preset JSON files as selectable presets.
3. Use the steady-parent and generator-pipeline research to define the next preset families, especially whether metadata/microcopy should become a separate future mode rather than a document preset.
