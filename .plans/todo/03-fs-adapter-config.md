# Plan 03: FS Adapter + Config Loading

## Goal

Implement file reading and JSON config loading. `CheckConfig` gets serde deserialization + garde validation in the FS adapter layer (not in domain types — domain stays pure).

## Prerequisites

- Plan 01 completed (domain types including `CheckConfig`, `TermLists`, `Thresholds`, `Range`, error types)
- `FileReader` and `ConfigLoader` traits defined in ports

## What to do

### 1. Add deps to FS adapter crate

`crates/adapters/outbound/fs/Cargo.toml`:
```toml
[dependencies]
prosesmasher-ports-outbound-traits = { path = "../../../ports/outbound/traits" }
prosesmasher-domain-types = { path = "../../../domain/types" }
serde = { workspace = true }
serde_json = { workspace = true }
garde = { workspace = true }
```

### 2. Implement `FsFileReader`

```rust
pub struct FsFileReader;

impl FileReader for FsFileReader {
    fn read_to_string(&self, path: &Path) -> Result<String, ReadError> { ... }
}
```

Note: `std::fs::read_to_string`, `std::fs::File`, and `serde_json::from_str` are all banned by clippy.toml. The FS adapter IS the centralized fs module, so it needs exemptions:

```rust
#[allow(clippy::disallowed_types)]   // reason: this IS the centralized fs module
#[allow(clippy::disallowed_methods)] // reason: this IS the centralized fs module
```

Apply these allows at the function level (not module-wide) with reason comments on each.

For JSON deserialization, use the `Validated<T>` pattern as clippy.toml mandates:
1. Deserialize raw JSON bytes into `ConfigDto` using a raw deserializer with `#[allow(clippy::disallowed_methods)]` (reason: FS adapter centralizes all deserialization)
2. Validate with `garde::Validate`
3. Convert `ConfigDto` → domain `CheckConfig`

### 3. Create serde DTOs for config

In the FS adapter (NOT in domain types), create internal serde structs that mirror `CheckConfig`:

```rust
#[derive(Deserialize, garde::Validate)]
#[serde(rename_all = "camelCase")]
struct ConfigDto {
    locale: String,
    terms: TermListsDto,
    thresholds: ThresholdsDto,
}
```

With garde validation rules on fields (non-empty locale, valid ranges, etc.).

### 4. Implement `FsConfigLoader`

```rust
pub struct FsConfigLoader;

impl ConfigLoader for FsConfigLoader {
    fn load_config(&self, path: &Path) -> Result<CheckConfig, ConfigError> {
        // 1. Read file via FsFileReader
        // 2. Deserialize JSON into ConfigDto
        // 3. Validate with garde
        // 4. Convert ConfigDto → domain CheckConfig
    }
}
```

The DTO-to-domain conversion handles:
- camelCase (JSON) → snake_case (Rust) via `#[serde(rename_all = "camelCase")]`
- `locale: String` → `Locale` enum: parse "en"→`Locale::En`, "ru"→`Locale::Ru`, etc. Return `ConfigError::ValidationFailed` for unknown locales.
- `simplicityPairs: Vec<[String; 2]>` → `Vec<(String, String)>` tuple conversion
- All `Option<T>` threshold fields default to `None` when absent from JSON

Implement as a `TryFrom<ConfigDto> for CheckConfig` or a standalone conversion function.

### 5. Create a sample config file for testing

`tests/fixtures/sample-config.json`:
```json
{
  "locale": "en",
  "terms": {
    "bannedWords": ["actually", "leverage"],
    "bannedPhrases": ["let's dive in"],
    "genderedTerms": ["mama"],
    "forbiddenTerms": [],
    "raceTerms": [],
    "hedgeWords": ["might", "perhaps"],
    "simplicityPairs": [["utilize", "use"]],
    "negationSignals": ["not", "isn't"],
    "reframeSignals": ["it's", "that's"],
    "llmOpeners": ["the interesting part is"],
    "affirmationClosers": ["and that's the key"],
    "summativePatterns": ["and that's what makes"],
    "falseQuestionPatterns": ["isn't that what"],
    "humbleBraggerPhrases": ["in my experience"],
    "jargonFakerPhrases": ["debugging your"],
    "stopWords": ["the", "a", "is", "in", "to", "and"]
  },
  "thresholds": {
    "wordCount": { "min": 650, "max": 1000 },
    "h2Count": { "min": 2, "max": 6 },
    "h3Min": 0,
    "boldMin": 3,
    "maxParagraphSentences": 4,
    "maxExclamationsPerParagraph": 1,
    "maxHedgesPerSentence": 1,
    "fleschKincaidMin": 50.0,
    "gunningFogMax": 14.0,
    "avgSentenceLengthMax": 25,
    "wordRepetitionMax": 5
  }
}
```

### 6. Tests

- Load sample config, verify all fields populated correctly
- Load config with missing optional fields (thresholds), verify defaults
- Load invalid JSON, verify `ConfigError::InvalidJson`
- Load config with invalid values (negative word count), verify garde catches it
- Read a file that doesn't exist, verify `ReadError::NotFound`

## Verification

```bash
cd apps/prosesmasher
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## What this plan does NOT do

- No CLI integration
- No check implementations
- No domain type changes (serde stays in the adapter)
