# Check Crate Restructure Plan

## Goal

Restructure the `prosesmasher` workspace so check ownership follows real hexagonal boundaries and future check growth does not recreate the coupling and test-topology problems seen in `guardrail3`.

The target shape is:
- one crate per **family** of checks
- one file per **individual** check
- orchestration separated from concrete checks
- shared test support separated from runtime crates
- parser behavior tested in the parser crate, not re-proved inside every check family

This plan is about **folder and crate structure**, not about implementing new checks yet.

## Architectural Principles

### 1. Split by ownership boundary, not by whim

We do **not** want:
- one giant `app/core` crate containing all checks forever
- one crate per rule

We **do** want:
- one crate per check family
- one small core crate for shared check contracts and runner
- one small catalog crate for assembling families and exposing the registry to adapters

### 2. Use actual nested folders

Do not fake nesting by flattening names in the filesystem.

Correct:

```text
crates/app/checks/core
crates/app/checks/lexical
crates/app/checks/heuristics
```

Cargo package names can still be fully qualified:
- `prosesmasher-app-checks-core`
- `prosesmasher-app-checks-lexical`

### 3. Tests should be layered, but tests should not call tests

Reusable things should be:
- typed input builders
- assertion helpers
- small family harness helpers

Not:
- test functions wrapping other test functions

### 4. Parser concerns must stay out of check crates

Check crates should consume typed `Document` values from the domain layer.

They should **not** own:
- markdown walking
- HTML extraction
- sentence segmentation
- syllable counting

Those belong to the parser adapter and stay there.

## Current Workspace Shape

Today the relevant runtime tree is:

```text
apps/prosesmasher/
├── Cargo.toml
├── crates/
│   ├── domain/
│   │   └── types/
│   ├── ports/
│   │   └── outbound/traits/
│   ├── app/
│   │   └── core/
│   │       └── src/
│   │           ├── check.rs
│   │           ├── runner.rs
│   │           ├── test_helpers.rs
│   │           ├── quality/
│   │           │   ├── lexical/
│   │           │   ├── heuristics/
│   │           │   ├── flow/
│   │           │   └── readability/
│   │           └── document_policy/
│   └── adapters/
│       ├── inbound/cli/
│       └── outbound/
│           ├── fs/
│           └── parser/
└── packages/
    └── prosesmasher/
```

The main structural problem is that `app/core` currently owns too many responsibilities:
- shared check contract
- shared runner
- all concrete check families
- shared test support

And the CLI adapter still owns registry logic in `src/checks.rs`, which is app-layer composition and should not stay in an adapter.

## Proposed Target Workspace Tree

```text
apps/prosesmasher/
├── Cargo.toml
├── clippy.toml
├── crates/
│   ├── domain/
│   │   └── types/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── config.rs
│   │           ├── document.rs
│   │           ├── error.rs
│   │           ├── lib.rs
│   │           ├── lib_tests.rs
│   │           ├── locale.rs
│   │           └── metadata.rs
│   ├── ports/
│   │   └── outbound/
│   │       └── traits/
│   │           ├── Cargo.toml
│   │           └── src/
│   │               ├── config_loader.rs
│   │               ├── document_parser.rs
│   │               ├── file_reader.rs
│   │               └── lib.rs
│   ├── app/
│   │   └── checks/
│   │       ├── core/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── check.rs
│   │       │   │   ├── lib.rs
│   │       │   │   ├── runner.rs
│   │       │   │   ├── runner_tests.rs
│   │       │   │   └── result_support.rs
│   │       │   └── tests/
│   │       │       └── locale_skip.rs
│   │       ├── catalog/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── catalog.rs
│   │       │   │   ├── descriptors.rs
│   │       │   │   ├── groups.rs
│   │       │   │   ├── lib.rs
│   │       │   │   ├── registry.rs
│   │       │   │   └── registry_tests.rs
│   │       │   └── tests/
│   │       │       └── group_listing.rs
│   │       ├── lexical/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── lib.rs
│   │       │   │   ├── mod_support.rs
│   │       │   │   ├── prohibited_terms.rs
│   │       │   │   ├── prohibited_terms_tests.rs
│   │       │   │   ├── required_terms.rs
│   │       │   │   ├── required_terms_tests.rs
│   │       │   │   ├── recommended_terms.rs
│   │       │   │   ├── recommended_terms_tests.rs
│   │       │   │   ├── simplicity.rs
│   │       │   │   ├── simplicity_tests.rs
│   │       │   │   ├── hedge_words.rs
│   │       │   │   └── hedge_words_tests.rs
│   │       │   └── tests/
│   │       │       └── family_thresholds.rs
│   │       ├── heuristics/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── lib.rs
│   │       │   │   ├── mod_support.rs
│   │       │   │   ├── affirmation_closers.rs
│   │       │   │   ├── affirmation_closers_tests.rs
│   │       │   │   ├── colon_dramatic.rs
│   │       │   │   ├── colon_dramatic_tests.rs
│   │       │   │   ├── em_dashes.rs
│   │       │   │   ├── em_dashes_tests.rs
│   │       │   │   ├── exclamation_density.rs
│   │       │   │   ├── exclamation_density_tests.rs
│   │       │   │   ├── fake_timestamps.rs
│   │       │   │   ├── fake_timestamps_tests.rs
│   │       │   │   ├── false_question.rs
│   │       │   │   ├── false_question_tests.rs
│   │       │   │   ├── fragment_stacking.rs
│   │       │   │   ├── fragment_stacking_tests.rs
│   │       │   │   ├── humble_bragger.rs
│   │       │   │   ├── humble_bragger_tests.rs
│   │       │   │   ├── jargon_faker.rs
│   │       │   │   ├── jargon_faker_tests.rs
│   │       │   │   ├── llm_openers.rs
│   │       │   │   ├── llm_openers_tests.rs
│   │       │   │   ├── negation_reframe.rs
│   │       │   │   ├── negation_reframe_tests.rs
│   │       │   │   ├── sentence_case.rs
│   │       │   │   ├── sentence_case_tests.rs
│   │       │   │   ├── smart_quotes.rs
│   │       │   │   ├── smart_quotes_tests.rs
│   │       │   │   ├── summative_closer.rs
│   │       │   │   ├── summative_closer_tests.rs
│   │       │   │   ├── triple_repeat.rs
│   │       │   │   └── triple_repeat_tests.rs
│   │       │   └── tests/
│   │       │       ├── family_registry.rs
│   │       │       └── threshold_behavior.rs
│   │       ├── flow/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── lib.rs
│   │       │   │   ├── paragraph_length.rs
│   │       │   │   ├── paragraph_length_tests.rs
│   │       │   │   ├── word_repetition.rs
│   │       │   │   └── word_repetition_tests.rs
│   │       │   └── tests/
│   │       │       └── family_behavior.rs
│   │       ├── readability/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── lib.rs
│   │       │   │   ├── avg_sentence_length.rs
│   │       │   │   ├── avg_sentence_length_tests.rs
│   │       │   │   ├── coleman_liau.rs
│   │       │   │   ├── coleman_liau_tests.rs
│   │       │   │   ├── flesch_kincaid.rs
│   │       │   │   ├── flesch_kincaid_tests.rs
│   │       │   │   ├── gunning_fog.rs
│   │       │   │   └── gunning_fog_tests.rs
│   │       │   └── tests/
│   │       │       └── family_behavior.rs
│   │       ├── document-policy/
│   │       │   ├── Cargo.toml
│   │       │   ├── src/
│   │       │   │   ├── lib.rs
│   │       │   │   ├── bold_density.rs
│   │       │   │   ├── bold_density_tests.rs
│   │       │   │   ├── code_fences.rs
│   │       │   │   ├── code_fences_tests.rs
│   │       │   │   ├── heading_counts.rs
│   │       │   │   ├── heading_counts_tests.rs
│   │       │   │   ├── heading_hierarchy.rs
│   │       │   │   ├── heading_hierarchy_tests.rs
│   │       │   │   ├── word_count.rs
│   │       │   │   ├── word_count_tests.rs
│   │       │   │   └── mod_support.rs
│   │       │   └── tests/
│   │       │       └── family_behavior.rs
│   │       └── test-support/
│   │           ├── Cargo.toml
│   │           └── src/
│   │               ├── assertions.rs
│   │               ├── builders.rs
│   │               ├── lib.rs
│   │               └── snippets.rs
│   └── adapters/
│       ├── inbound/
│       │   └── cli/
│       │       ├── Cargo.toml
│       │       └── src/
│       │           ├── args.rs
│       │           ├── args_tests.rs
│       │           ├── lib.rs
│       │           ├── lib_tests.rs
│       │           ├── main.rs
│       │           ├── output.rs
│       │           └── output_tests.rs
│       └── outbound/
│           ├── fs/
│           │   ├── Cargo.toml
│           │   ├── presets/
│           │   └── src/
│           │       ├── config_dto.rs
│           │       ├── config_loader.rs
│           │       ├── config_loader_tests.rs
│           │       ├── file_reader.rs
│           │       ├── file_reader_tests.rs
│           │       └── lib.rs
│           └── parser/
│               ├── Cargo.toml
│               ├── src/
│               │   ├── html_text.rs
│               │   ├── lib.rs
│               │   ├── lib_tests.rs
│               │   ├── markdown.rs
│               │   ├── markdown_tests.rs
│               │   ├── segmenter.rs
│               │   ├── segmenter_tests.rs
│               │   ├── syllables.rs
│               │   └── syllables_tests.rs
│               └── tests/
│                   └── fixtures/
└── packages/
    └── prosesmasher/
        ├── Cargo.toml
        ├── README.md
        ├── src/
        │   └── main.rs
        └── tests/
            └── packaged_cli_smoke.rs
```

## Ownership by Crate

### `crates/app/checks/core`

Owns:
- `Check` trait
- `BoxedCheck`
- `CheckMode` if added
- `run_checks`
- small shared app-layer helper types used across families

Must not own:
- concrete lexical/heuristic/readability/document-policy checks
- CLI grouping logic

Reason:
- this is the app-layer contract and runner, not a family bucket

### `crates/app/checks/catalog`

Owns:
- family assembly into `all_checks()`
- descriptors for listing checks
- group mapping and default enablement metadata

Takes over responsibilities currently in:
- `apps/prosesmasher/crates/adapters/inbound/cli/src/checks.rs`

Must not own:
- CLI argument parsing
- output formatting
- actual rule implementations

Reason:
- check registry/orchestration is app logic, not adapter logic

### `crates/app/checks/lexical`

Owns:
- lexical-term checks only
- family-local lexical matcher support if needed

Current source of truth:
- `apps/prosesmasher/crates/app/core/src/quality/lexical/*`

### `crates/app/checks/heuristics`

Owns:
- rhetorical and pattern checks
- the future LLM-slop checks
- family-local matcher support for phrase families and accumulative checks

Current source of truth:
- `apps/prosesmasher/crates/app/core/src/quality/heuristics/*`

Important note:
- `sentence_case.rs` should physically live here if it is conceptually heuristic/style, not imported from another family via `#[path]`

### `crates/app/checks/flow`

Owns:
- flow checks only

Current source of truth:
- `apps/prosesmasher/crates/app/core/src/quality/flow/*`

### `crates/app/checks/readability`

Owns:
- readability formulas and thresholds

Current source of truth:
- `apps/prosesmasher/crates/app/core/src/quality/readability/*`

### `crates/app/checks/document-policy`

Owns:
- structure and format policy checks

Current source of truth:
- `apps/prosesmasher/crates/app/core/src/document_policy/*`

### `crates/app/checks/test-support`

Owns:
- typed `Document` builders
- shared assertion helpers for checks
- small reusable fixture snippets if needed

Current source of truth:
- `apps/prosesmasher/crates/app/core/src/test_helpers.rs`

Rules:
- `publish = false`
- dev-oriented only
- no runtime code should depend on this crate

### Adapters

#### CLI

Keeps:
- args
- output
- composition root

Loses:
- direct ownership of check registry logic

Should depend on:
- `checks/catalog`
- `checks/core`
- parser adapter
- fs adapter

#### Parser

Keeps:
- markdown parsing
- HTML extraction
- segmentation
- syllable counting

No check family should test parser behavior except through end-to-end integration where truly necessary.

## Exact File Move Plan

### Move from `app/core` to `app/checks/core`

Move:
- `src/check.rs`
- `src/runner.rs`
- `src/runner_tests.rs`

Potential additions:
- `src/result_support.rs` for any shared finding-format helpers later

### Move from CLI adapter to `app/checks/catalog`

Move or rewrite:
- `src/checks.rs`
- `src/checks_tests.rs`

These should become app-layer catalog/registry code.

Then update CLI to call catalog APIs rather than assembling groups itself.

### Move lexical family

Move:
- `src/quality/lexical/*`

To:
- `crates/app/checks/lexical/src/*`

### Move heuristics family

Move:
- `src/quality/heuristics/*`

To:
- `crates/app/checks/heuristics/src/*`

Important cleanup:
- stop using `#[path = "../../document_policy/sentence_case.rs"]`
- put `sentence_case.rs` physically in heuristics if it belongs there conceptually

### Move flow family

Move:
- `src/quality/flow/*`

To:
- `crates/app/checks/flow/src/*`

### Move readability family

Move:
- `src/quality/readability/*`

To:
- `crates/app/checks/readability/src/*`

### Move document policy family

Move:
- `src/document_policy/*`

To:
- `crates/app/checks/document-policy/src/*`

### Move shared test helpers

Move:
- `src/test_helpers.rs`

To:
- `crates/app/checks/test-support/src/builders.rs`
- possibly split assertion helpers into `assertions.rs`

## Dependency Graph After Restructure

### Runtime dependency arrows

```text
domain/types
ports/outbound/traits
        ^
        |
app/checks/core
        ^
        |
app/checks/{lexical,heuristics,flow,readability,document-policy}
        ^
        |
app/checks/catalog
        ^
        |
adapters/inbound/cli
        ^
        |
packages/prosesmasher
```

Additional runtime edges:
- adapters/outbound/fs -> domain + ports
- adapters/outbound/parser -> domain + ports
- CLI -> fs + parser + checks/core + checks/catalog

### Test-only dependency arrows

```text
app/checks/test-support -> domain/types + low-expectations

app/checks/{family crates}
  dev-depend on:
  - app/checks/test-support
  - maybe app/checks/core
```

Rules:
- family crates do not depend on each other
- runtime crates do not depend on `test-support`

## Test Topology Plan

### Layer 1: Rule semantics

Location:
- sidecar `*_tests.rs` beside each rule file

Purpose:
- exact behavior of one rule over typed `Document` inputs

Allowed:
- `test-support` builders
- family-local assertion helpers

Forbidden:
- parser behavior
- catalog behavior
- CLI behavior
- end-to-end fixture walking

### Layer 2: Family tests

Location:
- top-level `tests/*.rs` harnesses inside each family crate

Purpose:
- family-level threshold behavior
- family-owned shared matcher helpers
- ownership boundaries inside the family
- non-hit boundaries where one rule should not catch another rule's case

Important:
- these tests should reuse helpers, not call other test functions

### Layer 3: App orchestration tests

Location:
- `checks/core` and `checks/catalog`

Purpose:
- locale skipping
- all-check assembly
- group membership
- default enablement
- filtering by id/group

### Layer 4: Adapter/integration tests

Location:
- parser crate
- fs adapter
- CLI adapter
- packaged binary tests

Purpose:
- real parser behavior
- config loading
- output formatting
- end-to-end command behavior

## Why This Split Is Better Than Staying in `app/core`

- concrete checks stop depending on one giant catch-all app crate
- the CLI adapter stops owning app-layer registry logic
- future slop checks can grow inside `heuristics` without turning `app/core` into a god crate
- test support becomes explicit and reusable without leaking into runtime APIs
- family-level test harnesses become possible without mixing parser facts into rule semantics

## Why This Split Is Better Than One Crate Per Rule

- avoids crate explosion
- keeps shared matcher/support code family-owned
- reduces workspace maintenance churn
- preserves meaningful compile boundaries instead of artificial ones
- matches the actual stable ownership boundaries in the codebase

## Migration Sequence

### Phase 1: Introduce new crates and wire workspace

1. Add new workspace members under `crates/app/checks/*`
2. Create empty `Cargo.toml` + `src/lib.rs` for each
3. Move `check.rs` and `runner.rs` into `checks/core`
4. Fix all imports to compile

### Phase 2: Extract families one by one

1. Extract `lexical`
2. Extract `flow`
3. Extract `readability`
4. Extract `document-policy`
5. Extract `heuristics`

Reason for order:
- lower shared complexity first
- heuristics last because it has the most future churn and helper reuse

### Phase 3: Extract catalog from CLI

1. Create `checks/catalog`
2. Move registry/grouping logic from CLI adapter
3. Update CLI to call catalog APIs

### Phase 4: Extract shared test support

1. Create `checks/test-support`
2. Move `test_helpers.rs`
3. Update family dev-dependencies and imports

### Phase 5: Re-layer tests where needed

1. Keep rule-sidecar tests as pure rule semantics
2. Add family `tests/*.rs` harnesses only where family-level behavior needs coverage
3. Keep parser-backed tests in parser crate
4. Keep CLI/package integration tests in adapter/package crates

## Immediate Cleanup Opportunities During Migration

- Eliminate the current `#[path]` import of `sentence_case.rs` into heuristics
- Remove app-layer registry logic from CLI adapter
- Reduce `app/core` to nothing or retire it entirely after extraction
- Make test support explicit instead of crate-private-global

## Final State Expectation

After the migration:
- the workspace has real nested family crates under `app/checks/`
- each check family has clear ownership
- each rule still lives in its own file
- orchestration is separate from implementation
- adapters do not own app-layer registry logic
- tests are layered by concern instead of getting accidentally dragged through the whole system

That should give `prosesmasher` the strong family boundaries you want without recreating the `guardrail3` test and compile-topology mess.
