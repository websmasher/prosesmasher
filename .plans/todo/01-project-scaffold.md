# Plan 01: Project Scaffold

## Goal

Create the new parser crate, add all workspace dependencies, write all domain types, port traits, and stubs. Zero business logic. Everything compiles with `cargo check`.

## Prerequisites

- Existing 5-crate scaffold is compiling (domain/types, ports/outbound/traits, app/core, adapters/inbound/cli, adapters/outbound/fs)
- Read `check_structure.md` for the full architecture context

## What to do

### 1. Create new crate: `adapters/outbound/parser`

Create `crates/adapters/outbound/parser/` with:
- `Cargo.toml` — name `prosesmasher-adapters-outbound-parser`, depends on `prosesmasher-ports-outbound-traits` and `prosesmasher-domain-types`
- `src/lib.rs` — doc comment + placeholder use imports

Add to workspace `Cargo.toml` members list:
```toml
"crates/adapters/outbound/parser",
```

Add to `guardrail3.toml`:
```toml
[rust.apps.prosesmasher-adapters-outbound-parser]
type = "library"
layer = "adapters"
```

### 2. Add workspace dependencies

Add to `[workspace.dependencies]` in `apps/prosesmasher/Cargo.toml`:
```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
pulldown-cmark = "0.12"
icu_segmenter = "1"
icu_casemap = "1"
hyphenation = { version = "0.8", features = ["embed_en", "embed_de", "embed_es", "embed_pt", "embed_fr", "embed_ru"] }
```

Also add:
```toml
walkdir = "2"
```

Add `low-expectations` as a path dependency (exact path to be verified at implementation time — expected at `../../low-expectations/crates/low-expectations` relative to workspace root, or use git dependency if repos are separate). Add to `app/core` Cargo.toml as a dependency. This is needed from Plan 04 onward.

### 3. Write domain types (`domain/types/src/`)

Split `lib.rs` into modules. Create these files:

**`locale.rs`**:
```rust
pub enum Locale {
    En, Ru, De, Es, Pt, Fr, Id,
}
```
With `Debug, Clone, Copy, PartialEq, Eq` derives.

**`document.rs`**:
```rust
pub struct Document {
    pub locale: Locale,
    pub sections: Vec<Section>,
    pub metadata: DocumentMetadata,
}

pub struct Section {
    pub heading: Option<Heading>,
    pub blocks: Vec<Block>,
}

pub struct Heading {
    pub level: u8,
    pub text: String,
}

pub enum Block {
    Paragraph(Paragraph),
    List(ListBlock),
    BlockQuote(Vec<Block>),
    CodeBlock(String),
}

pub struct Paragraph {
    pub sentences: Vec<Sentence>,
    pub has_bold: bool,
    pub has_italic: bool,
    pub links: Vec<Link>,
}

pub struct Sentence {
    pub text: String,
    pub words: Vec<Word>,    // ICU4X word-segmented, each with syllable count
    pub word_count: usize,
}

pub struct Word {
    pub text: String,
    pub syllable_count: usize,  // via hyphenation crate, filled by parser
}

pub struct Link {
    pub text: String,
    pub url: String,
}

pub struct ListBlock {
    pub ordered: bool,
    pub items: Vec<String>,
}
```
All with `Debug, Clone` derives. `Word` also gets `PartialEq, Eq` for test assertions.

**`metadata.rs`**:
```rust
pub struct DocumentMetadata {
    pub total_words: usize,
    pub total_sentences: usize,
    pub total_syllables: usize,
    pub heading_counts: HeadingCounts,
    pub bold_count: usize,
    pub italic_count: usize,
    pub paragraph_count: usize,
    pub link_count: usize,
}

pub struct HeadingCounts {
    pub h1: usize,
    pub h2: usize,
    pub h3: usize,
    pub h4_plus: usize,
}
```
With `Debug, Clone, Default` derives.

**`config.rs`**:
```rust
pub struct CheckConfig {
    pub locale: Locale,
    pub terms: TermLists,
    pub thresholds: Thresholds,
}

pub struct TermLists {
    pub banned_words: Vec<String>,
    pub banned_phrases: Vec<String>,
    pub gendered_terms: Vec<String>,
    pub forbidden_terms: Vec<String>,
    pub race_terms: Vec<String>,
    pub hedge_words: Vec<String>,
    pub simplicity_pairs: Vec<(String, String)>,
    pub negation_signals: Vec<String>,
    pub reframe_signals: Vec<String>,
    pub llm_openers: Vec<String>,
    pub affirmation_closers: Vec<String>,
    pub summative_patterns: Vec<String>,
    pub false_question_patterns: Vec<String>,
    pub humble_bragger_phrases: Vec<String>,
    pub jargon_faker_phrases: Vec<String>,
    pub stop_words: Vec<String>,
}

pub struct Thresholds {
    pub word_count: Option<Range>,
    pub h2_count: Option<Range>,
    pub h3_min: Option<usize>,
    pub bold_min: Option<usize>,
    pub max_paragraph_sentences: Option<usize>,
    pub max_exclamations_per_paragraph: Option<usize>,
    pub max_hedges_per_sentence: Option<usize>,
    pub flesch_kincaid_min: Option<f64>,
    pub gunning_fog_max: Option<f64>,
    pub avg_sentence_length_max: Option<usize>,
    pub word_repetition_max: Option<usize>,
    pub coleman_liau_max: Option<f64>,
}

pub struct Range {
    pub min: usize,
    pub max: usize,
}
```
With `Debug, Clone, Default` derives. NO serde derives here — domain types are pure. Serde lives in the FS adapter.

**`error.rs`**:
```rust
pub enum ReadError {
    NotFound(String),
    PermissionDenied(String),
    Io(String),
}

pub enum ParseError {
    InvalidMarkdown(String),
    SegmentationFailed(String),
}

pub enum ConfigError {
    NotFound(String),
    InvalidJson(String),
    ValidationFailed(String),
}
```
With `Debug, Clone` derives, `Display` impl for each, and `std::error::Error` impl (needed for `Box<dyn Error>` in CLI main). Domain types crate will need no external deps for this — `std::error::Error` and `std::fmt::Display` are stdlib.

**Important derives across all types:**
- All structs: `Debug, Clone`
- `DocumentMetadata`, `HeadingCounts`, `Thresholds`: also `Default` (needed for test fixtures and optional config)
- `Locale`: `Debug, Clone, Copy, PartialEq, Eq`
- `Severity`: `Debug, Clone, PartialEq, Eq`
- `Word`: `Debug, Clone, PartialEq, Eq`
- `Range`: `Debug, Clone, Copy`

**`lib.rs`** — re-export all modules:
```rust
pub mod locale;
pub mod document;
pub mod metadata;
pub mod config;
pub mod error;

pub use locale::Locale;
pub use document::*;
pub use metadata::*;
pub use config::*;
pub use error::*;
```

### 4. Write port traits (`ports/outbound/traits/src/`)

**`file_reader.rs`**:
```rust
use std::path::Path;
use prosesmasher_domain_types::ReadError;

pub trait FileReader {
    fn read_to_string(&self, path: &Path) -> Result<String, ReadError>;
}
```

**`document_parser.rs`**:
```rust
use prosesmasher_domain_types::{Document, Locale, ParseError};

pub trait DocumentParser {
    fn parse(&self, markdown: &str, locale: &Locale) -> Result<Document, ParseError>;
}
```

**`config_loader.rs`**:
```rust
use std::path::Path;
use prosesmasher_domain_types::{CheckConfig, ConfigError};

pub trait ConfigLoader {
    fn load_config(&self, path: &Path) -> Result<CheckConfig, ConfigError>;
}
```

**`lib.rs`** — re-export:
```rust
pub mod file_reader;
pub mod document_parser;
pub mod config_loader;

pub use file_reader::FileReader;
pub use document_parser::DocumentParser;
pub use config_loader::ConfigLoader;
```

### 5. Update `app/core` stub

Replace placeholder imports with actual use of domain types:
```rust
//! Application core — check trait, runner, and all checks.

use prosesmasher_domain_types as _;
```

No check trait yet — that's Plan 04.

### 6. Update adapter stubs

**`adapters/outbound/fs/src/lib.rs`** — placeholder:
```rust
//! Filesystem adapter — FileReader and ConfigLoader implementations.

use prosesmasher_ports_outbound_traits as _;
```

**`adapters/outbound/parser/src/lib.rs`** — placeholder:
```rust
//! Markdown parser adapter — DocumentParser implementation.

use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
```

**`adapters/inbound/cli/`** — update imports to use real types but keep hello world main for now.

### 7. Wire crate dependencies

Update each crate's `Cargo.toml` to add the dependencies listed in the Dependencies Summary table from `check_structure.md`. Don't add external crate deps yet (pulldown-cmark, icu, etc.) — those come in Plan 02/03 when the adapters are implemented.

## Verification

```bash
cd apps/prosesmasher
cargo check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

All must pass. No warnings.

## What this plan does NOT do

- No business logic (no check implementations, no parsing, no file I/O)
- No external crate dependencies beyond what's needed to compile types
- No low-expectations dependency yet
- No CLI implementation
- No tests beyond compile checks
