# Plan 05: CLI Adapter

## Goal

Implement the clap-based CLI, composition root, and output formatting. End-to-end working: `prosesmasher check file.md --config config.json` → parse → run checks → print results.

## Prerequisites

- Plan 01 (types), Plan 02 (parser), Plan 03 (fs/config), Plan 04 (check trait + 3 checks) all completed

## What to do

### 1. Add deps to CLI crate

`crates/adapters/inbound/cli/Cargo.toml`:
```toml
[dependencies]
prosesmasher-domain-types = { path = "../../../domain/types" }
prosesmasher-ports-outbound-traits = { path = "../../../ports/outbound/traits" }
prosesmasher-app-core = { path = "../../../app/core" }
prosesmasher-adapters-outbound-fs = { path = "../../outbound/fs" }
prosesmasher-adapters-outbound-parser = { path = "../../outbound/parser" }
garde = { workspace = true }
clap = { workspace = true }
low-expectations = { ... }  # same as app/core
```

### 2. Implement `args.rs`

Clap derive-based argument parsing:

```rust
#[derive(clap::Parser)]
#[command(name = "prosesmasher", about = "Prose quality checker")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Check markdown files for prose quality issues
    Check {
        /// File or directory to check
        path: PathBuf,

        /// Config file (JSON) with term lists and thresholds
        #[arg(long)]
        config: Option<PathBuf>,

        /// Only run checks from this group: terms, patterns, structure, readability
        #[arg(long)]
        group: Option<String>,
    },
}
```

### 3. Implement `output.rs`

Format `SuiteValidationResult` into colored terminal output:

```
essay.md
  FAIL  banned-words         "actually" found (expected: none)
  PASS  word-count           812 words (expected: 650-1000)

2 checks: 1 passed, 1 failed
```

- PASS in green, FAIL in red, WARN in yellow
- Show check label + observed value + expected value
- Summary line at the end with counts
- Exit code 0 if all pass, 1 if any fail

### 4. Implement `main.rs` — composition root

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::Check { path, config, group } => {
            // 1. Construct adapters
            let file_reader = FsFileReader;
            let config_loader = FsConfigLoader;
            let parser = MarkdownParser;

            // 2. Load config (or use defaults)
            let check_config = match config {
                Some(p) => config_loader.load_config(&p)?,
                None => CheckConfig::default(),
            };

            // 3. Collect files to check
            // Use `walkdir` crate (added to workspace deps in Plan 01) for recursive .md discovery.
            // walkdir is allowed in the CLI adapter (it's the entry point, not scattered fs access).
            // Add #[allow(clippy::disallowed_methods)] with reason for any std::fs calls needed.
            let files = if path.is_dir() {
                // walkdir::WalkDir::new(&path).into_iter()
                //   .filter_map(|e| e.ok())
                //   .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
                //   .map(|e| e.into_path())
                //   .collect()
                todo!() // implement with walkdir
            } else {
                vec![path]
            };

            // 4. Collect checks (filtered by group if specified)
            let all_checks = collect_checks(group.as_deref());

            // 5. For each file: read → parse → run checks → print
            let mut any_failed = false;
            for file in &files {
                let content = file_reader.read_to_string(file)?;
                let doc = parser.parse(&content, &check_config.locale)?;
                let check_refs: Vec<&dyn Check> = all_checks.iter().map(|c| c.as_ref()).collect();
                let result = run_checks(&check_refs, &doc, &check_config);
                print_result(file, &result);
                if !result.success {
                    any_failed = true;
                }
            }

            if any_failed {
                // std::process::exit is banned. Return an error instead.
                return Err("One or more checks failed".into());
            }
        }
    }
    Ok(())
}
```

Note: `std::process::exit` is banned. Return an error type from main instead, or use a custom exit mechanism.

### 5. Implement `collect_checks`

```rust
fn collect_checks(group: Option<&str>) -> Vec<Box<dyn Check>> {
    match group {
        Some("terms") => terms::all_checks(),
        Some("patterns") => patterns::all_checks(),
        Some("structure") => structure::all_checks(),
        Some("readability") => readability::all_checks(),
        None => {
            let mut all = Vec::new();
            all.extend(terms::all_checks());
            all.extend(patterns::all_checks());
            all.extend(structure::all_checks());
            all.extend(readability::all_checks());
            all
        }
        Some(_unknown) => {
            // Note: println!/eprintln! are banned by clippy.
            // Return an error via Result instead of printing + returning empty vec.
            // Change function signature to return Result<Vec<Box<dyn Check>>, String>.
            return Err(format!("Unknown check group: {_unknown}"));
        }
    }
}
```

### 6. Directory scanning

When `path` is a directory, find all `.md` files recursively. Use the `FileReader` trait's capabilities or walk the directory in the CLI adapter (directory walking is CLI-specific behavior, not a port concern).

### 7. End-to-end test

Create a test markdown file and config file in `tests/fixtures/`:
- `tests/fixtures/test-essay.md` — short markdown with known issues (an em-dash, a banned word)
- `tests/fixtures/test-config.json` — config with matching term lists and thresholds

Run `cargo run -- check tests/fixtures/test-essay.md --config tests/fixtures/test-config.json` and verify output.

## Verification

```bash
cd apps/prosesmasher
cargo build
cargo run -- check tests/fixtures/test-essay.md --config tests/fixtures/test-config.json
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## What this plan does NOT do

- No additional checks beyond the 3 from Plan 04
- No colored output (can use plain text first, add colors later)
