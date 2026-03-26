# Five-Round Test Attack Hardening

**Date:** 2026-03-26 00:35
**Scope:** `apps/prosesmasher/crates/adapters/inbound/cli`, `apps/prosesmasher/crates/adapters/outbound/fs`, `apps/prosesmasher/crates/adapters/outbound/parser`, `apps/prosesmasher/crates/app/checks/catalog`, `apps/prosesmasher/crates/domain/types`, `apps/prosesmasher/packages/prosesmasher`

## Summary
Ran five adversarial post-migration test-attack rounds against the new runtime/assertions layout, fixed the meaningful gaps that surfaced, and reverified the full workspace. The final state tightens black-box coverage, moves more reusable semantics into assertions crates, removes the last structural mismatch in catalog sidecars, and updates stale parser fixture expectations to match the resolved public contract.

## Context & Problem
The codebase had just been reorganized into GuardRails-style family crates with sidecar tests and sibling assertions crates, but the user explicitly wanted an “ideal result” rather than a first-pass migration. That meant treating the new structure as suspect until multiple rounds of adversarial review proved that the split was not only compiling, but also satisfying the testing architecture rules in spirit: assertions should own reusable semantics, sidecars should own scenarios, public black-box surfaces should be pinned, and fixture comments should not drift from the actual parser contract.

The work started from a green but not fully attacked tree. Prior rounds had already fixed the biggest migration breakage, but they had not yet closed the loop on several important post-split weaknesses:
- CLI runtime/package black-box coverage was still thinner than the public `run` surface.
- FS and domain assertions crates existed, but some reusable semantics were still stranded in sidecars.
- Parser public tests still had fixture-comment drift and were missing black-box coverage for some meaningful public branches.
- `catalog` still had a structural sidecar mismatch (`catalog_tests` attached to `lib.rs`).

## Decisions Made

### Treat the migration as incomplete until repeated attack rounds converged
- **Chose:** Run five rounds of attack rather than treating the first green workspace run as sufficient.
- **Why:** The user requirement was specifically to keep attacking until meaningful issues stopped appearing. The migration changed test ownership boundaries across many crates, so “compiles and passes once” was not a trustworthy stopping point.
- **Alternatives considered:**
  - Stop after the first green workspace run — rejected because it would only prove syntactic validity, not architectural convergence.
  - Attack only the app/check crates — rejected because the user’s concern was repo-wide RS-TEST compliance, including adapters and package smoke tests.

### Narrow CLI assertions to semantics instead of harness orchestration
- **Chose:** Remove `run(parse_ok(...))` execution from CLI assertion helpers and make the runtime black-box tests own execution while assertions only verify parsed args / exits / error content.
- **Why:** The stale helper pattern let the assertions crate partially own harness execution, which is exactly the kind of ownership drift the new architecture was meant to prevent.
- **Alternatives considered:**
  - Leave the helpers as-is because the tests were passing — rejected because it violated the intended split between harness and assertions.
  - Move all runtime helpers into the sidecar only — rejected because the parser/args semantic assertions are still legitimately reusable.

### Expand black-box coverage only for meaningful public branches
- **Chose:** Add/keep runtime/package black-box coverage for `list-presets`, `dump-config --full-config`, invalid group/check filtering, parser image suppression, parser blockquotes, and visible-vs-hidden raw HTML handling.
- **Why:** These are real public contract branches where regressions would matter to downstream callers or users. The attack rounds specifically surfaced them as meaningful gaps.
- **Alternatives considered:**
  - Add broad black-box duplication of every sidecar parser case — rejected because that would produce ceremony and maintenance noise without proportional value.
  - Keep those branches covered only by internal sidecars — rejected because the architecture now explicitly distinguishes reusable public behavior from internal scenario generation.

### Fix fixture comments and public tests instead of blessing implementation artifacts
- **Chose:** Update parser fixture headers (`01-clean-article.md`, `04-multilingual-stress.md`) to match the resolved parser behavior and align `public_surface.rs` with top-level list intent instead of recursive-list implementation artifacts.
- **Why:** Fixture comments are part of the human-readable contract. Leaving them stale invites future regressions back toward already-rejected expectations.
- **Alternatives considered:**
  - Ignore the fixture comments because tests already encoded the truth — rejected because the attack rounds showed that stale fixture headers are exactly the sort of source-of-truth drift that misleads future agents.
  - Change the public test to bless the parser’s recursive nested-list emission detail — rejected because that would pin the wrong contract.

### Move more semantics into FS/domain assertions without breaking type boundaries
- **Chose:** Add reusable semantic assertions in FS/domain assertion crates, but express domain default-config checks via extracted facts rather than concrete runtime-only types when the shared crate boundary would otherwise create duplicate-type problems.
- **Why:** The shared assertions crate must remain usable both inside and outside the runtime crate. For domain types, directly accepting the runtime crate’s concrete internal types caused the “second type instance” problem in unit tests.
- **Alternatives considered:**
  - Keep those semantics inline in sidecars — rejected because that preserved the exact drift the attack rounds were complaining about.
  - Make runtime internals more public so the assertions crate could take concrete types — rejected because that would degrade the production boundary just to satisfy tests.

### Normalize the last catalog structural mismatch
- **Chose:** Rename the catalog sidecar from `catalog_tests` to `lib_tests` and rewire `lib.rs` accordingly.
- **Why:** `catalog_tests` attached to `lib.rs` was the last deterministic structural mismatch in the new layout. The sidecar should correspond to the module it actually tests.
- **Alternatives considered:**
  - Leave it because the tests still compiled — rejected because the user explicitly wanted exact adherence to the structural rules, not “close enough.”

## Architectural Notes
- The final test layout is now materially closer to the intended split:
  - assertions crates own reusable public-behavior semantics,
  - sidecar test modules own scenario construction,
  - external black-box tests pin the public runtime/package surfaces.
- Domain assertions required a specific compromise: shared assertions on default English config behavior are represented as extracted facts instead of concrete `CheckConfig` references, because the assertions crate and the runtime crate cannot safely share the same internal type instance in unit-test compilation.
- Parser black-box coverage is intentionally selective. The goal is not to duplicate all internal parser sidecar cases, but to pin the meaningful public branches that are likely to regress or that define the public parsing contract.
- Catalog now matches the same sidecar naming convention already used elsewhere (`lib.rs` <-> `lib_tests/`), which removes the last deterministic mismatch found by local structural scans.

## Information Sources
- GuardRails family test architecture doc:
  - `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- Test-attack workflow skill:
  - `/Users/tartakovsky/.codex/skills/test-attack/SKILL.md`
- Prior migration/backstory worklogs:
  - `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md`
  - `.worklogs/2026-03-25-220947-normalize-sidecar-synthetic-layout.md`
  - `.worklogs/2026-03-25-223349-complete-rs-test-runtime-assertions-split.md`
- Current runtime/assertions code and tests:
  - `apps/prosesmasher/crates/adapters/inbound/cli/...`
  - `apps/prosesmasher/crates/adapters/outbound/fs/...`
  - `apps/prosesmasher/crates/adapters/outbound/parser/...`
  - `apps/prosesmasher/crates/app/checks/catalog/...`
  - `apps/prosesmasher/crates/domain/types/...`
  - `apps/prosesmasher/packages/prosesmasher/...`
- Repeated local verification:
  - `cargo fmt --manifest-path apps/prosesmasher/Cargo.toml --all`
  - `cargo test --manifest-path apps/prosesmasher/Cargo.toml --workspace`
  - structural scans for sidecar/assertions mismatches and forbidden flat tests
- Parallel attack rounds from three focused subagents covering:
  - CLI/package
  - parser
  - fs/domain

## Open Questions / Future Considerations
- The parser still preserves doubled spacing around removed inline images (`Before  after.`). The new public black-box test intentionally does not bless a whitespace-normalization policy there; it only pins the meaningful contract that alt text does not leak. If the product later wants whitespace normalization, that should be a deliberate parser behavior change, not an accidental test artifact.
- `domain/types` now has materially better shared assertions coverage, but it is still not a rule-by-rule project like the check crates. If GuardRails eventually enforces a stricter “every reusable semantic assertion must live outside the sidecar” rule, domain may still need more extraction.
- Unix-only permission-denied coverage in FS remains environment-conditional when running as root. The tests are now stronger than before, but a truly deterministic root-safe permission-denied strategy would need a different mechanism.

## Key Files for Context
- `apps/prosesmasher/crates/adapters/inbound/cli/assertions/src/args.rs` — CLI assertion helpers after removing harness execution from the assertions layer
- `apps/prosesmasher/crates/adapters/inbound/cli/runtime/tests/cli_behavior.rs` — public runtime black-box CLI contract coverage
- `apps/prosesmasher/packages/prosesmasher/tests/packaged_cli_smoke.rs` — packaged binary smoke coverage for public CLI surface
- `apps/prosesmasher/crates/adapters/outbound/fs/assertions/src/config_loader.rs` — FS config-loader semantic assertions now owning more reusable expectations
- `apps/prosesmasher/crates/adapters/outbound/fs/runtime/src/config_loader_tests/synthetic.rs` — FS sidecar scenarios using the expanded assertions layer
- `apps/prosesmasher/crates/adapters/outbound/parser/runtime/tests/public_surface.rs` — parser black-box tests for public behavior branches
- `apps/prosesmasher/crates/adapters/outbound/parser/runtime/tests/public_parser_contract.rs` — public parser contract on section splitting and fixture intent
- `apps/prosesmasher/crates/adapters/outbound/parser/runtime/tests/fixtures/01-clean-article.md` — updated canonical fixture header matching resolved parser behavior
- `apps/prosesmasher/crates/adapters/outbound/parser/runtime/tests/fixtures/04-multilingual-stress.md` — updated canonical fixture header matching resolved parser behavior
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib.rs` — catalog runtime entrypoint now wired to `lib_tests`
- `apps/prosesmasher/crates/app/checks/catalog/runtime/src/lib_tests/synthetic.rs` — renamed sidecar matching the module it actually tests
- `apps/prosesmasher/crates/domain/types/assertions/src/config.rs` — shared assertions for default English config semantics using extracted facts
- `apps/prosesmasher/crates/domain/types/assertions/src/errors.rs` — shared domain error-display/source assertions
- `apps/prosesmasher/crates/domain/types/runtime/src/lib_tests/synthetic.rs` — sidecar scenarios updated to lean on shared domain assertions
- `.worklogs/2026-03-25-223349-complete-rs-test-runtime-assertions-split.md` — prior repo-wide migration state before this hardening pass
- `.worklogs/2026-03-25-220947-normalize-sidecar-synthetic-layout.md` — prior normalization of sidecar synthetic layout
- `.worklogs/2026-03-25-220023-split-checks-into-family-crates.md` — original family-crate split

## Next Steps / Continuation Plan
1. If more slop checks are added, start by creating the runtime/assertions pairing in the owning family crate before writing scenarios, then immediately run a local structural scan to ensure the sidecar and assertions filenames match the owning module.
2. If GuardRails begins enforcing the architecture automatically, run it against `apps/prosesmasher` and compare its findings to the manual structural scans used here; use `catalog`, `domain/types`, and the adapters as the first pressure-test targets because they were the most migration-sensitive areas.
3. If parser behavior around removed inline images needs to become part of the public contract, decide explicitly whether whitespace should collapse and then update both `runtime/src/markdown.rs` and `runtime/tests/public_surface.rs` together.
