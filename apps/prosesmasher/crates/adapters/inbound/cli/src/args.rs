//! CLI argument parsing via clap.

use std::path::PathBuf;

use clap::{ArgGroup, Parser};

/// Prose quality checker — deterministic AI slop detection.
#[derive(Parser, Debug)]
#[command(
    name = "prosesmasher",
    version,
    about = "Validate prose quality in markdown and short-form text using deterministic checks.",
    long_about = "Validate prose quality in markdown and short-form text using deterministic checks.\n\nTop-level workflow:\n- run `list-presets` to see shipped presets\n- run `dump-config --preset <name>` to inspect a preset\n- run `dump-config --full-config` to print the full editable config\n- run `check <path> --preset <name>` to validate with a shipped preset\n- run `check <path> --config <path>` to validate with your own config\n\nRules:\n- `check` requires exactly one config source: either `--preset` or `--config`\n- `dump-config` requires exactly one source: either `--preset <name>` or `--full-config`\n- presets keep shared quality defaults and differ only by document structure",
    after_help = "Commands and common combinations:\n  prosesmasher list-presets\n      Show all shipped preset names and what each one is for.\n\n  prosesmasher dump-config --full-config\n      Print the full config surface to stdout so you can save and edit it.\n\n  prosesmasher dump-config --preset article-en\n      Print one shipped preset to stdout so you can inspect or copy it.\n\n  prosesmasher check draft.md --preset article-en\n      Validate one markdown file with a shipped preset.\n\n  prosesmasher check drafts/ --preset substack-en --group quality\n      Validate a directory recursively, but only run quality checks.\n\n  prosesmasher check draft.md --config my-config.json --format json\n      Validate with your own config and emit machine-readable JSON.\n\n  prosesmasher check draft.md --config my-config.json --format json --include-checks\n      Include the full per-check results array in JSON output.\n\n  prosesmasher check draft.md --config my-config.json --check prohibited-terms,em-dashes\n      Validate with your own config but only run specific checks."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands.
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Check markdown files for prose quality issues.
    #[command(group(
        ArgGroup::new("config_source")
            .args(["config", "preset"])
            .required(true)
            .multiple(false)
    ))]
    Check {
        /// File or directory to check (recursively finds .md files in directories).
        path: PathBuf,

        /// Config file (JSON) to use for validation.
        #[arg(long)]
        config: Option<PathBuf>,

        /// Shipped preset name to use instead of --config.
        #[arg(long)]
        preset: Option<String>,

        /// Only run checks from this group: quality, document-policy, lexical, heuristics, flow, readability.
        #[arg(long)]
        group: Option<String>,

        /// Only run specific checks by ID (comma-separated).
        /// Example: --check prohibited-terms,em-dashes,word-count
        #[arg(long)]
        check: Option<String>,

        /// Output format: text (human-readable) or json (machine-readable).
        #[arg(long, default_value = "text")]
        format: OutputFormat,

        /// Include the full per-check results array in JSON output.
        #[arg(long)]
        include_checks: bool,
    },

    /// List the shipped presets that can be used with --preset.
    ListPresets,

    /// Dump config JSON to stdout.
    #[command(group(
        ArgGroup::new("dump_source")
            .args(["preset", "full_config"])
            .required(true)
            .multiple(false)
    ))]
    DumpConfig {
        /// Dump a shipped preset by name.
        #[arg(long)]
        preset: Option<String>,

        /// Dump the full editable config.
        #[arg(long = "full-config")]
        full_config: bool,
    },
}

/// Output format for check results.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text output.
    Text,
    /// Machine-readable JSON output.
    Json,
}

#[cfg(test)]
#[path = "args_tests.rs"]
mod tests;
