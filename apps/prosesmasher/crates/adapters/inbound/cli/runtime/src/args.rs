//! CLI argument parsing via clap.

use std::path::PathBuf;

use clap::{ArgGroup, Parser};

/// Prose quality checker — deterministic AI slop detection.
#[derive(Parser, Debug)]
#[command(
    name = "prosesmasher",
    version,
    about = "Validate prose quality in markdown and short-form text using deterministic checks.",
    long_about = "Validate prose quality in markdown and short-form text using deterministic checks.\n\nThe tool is English-first and opinionated about AI-slop, salesy boilerplate, readability, and markdown structure.\n\nConfig model:\n- JSON config uses camelCase keys\n- top-level shape is `locale`, `quality`, and `documentPolicy`\n- `quality.lexical` is mostly override/merge-driven policy\n- `quality.heuristics` holds rhetorical and style heuristics\n- `quality.flow` holds paragraph and repetition controls\n- `quality.readability` holds readability thresholds\n- `documentPolicy` is opt-in structural policy; omitted fields stay off\n- `defaults: true` means merge with built-in defaults; `defaults: false` means replace them\n\nPreset model:\n- presets are partial policy profiles with shared quality defaults and different structural envelopes\n- `dump-config --full-config` prints the full editable schema example",
    after_help = "Commands and common combinations:\n  prosesmasher list-presets\n      Show shipped preset names and what each one is for.\n\n  prosesmasher dump-config --full-config\n      Print the full editable config example to stdout.\n\n  prosesmasher dump-config --preset article-en\n      Print a shipped preset profile to stdout.\n\n  prosesmasher check --list-checks\n      List all available checks with IDs, groups, default-enabled state, and locale support.\n\n  prosesmasher check --list-checks --group readability --format json\n      List only readability checks in machine-readable form.\n\n  prosesmasher check draft.md --preset article-en\n      Validate one markdown file with a shipped preset.\n\n  prosesmasher check drafts/ --preset substack-en --group quality\n      Validate a directory recursively, but only run quality checks.\n\n  prosesmasher check draft.md --config my-config.json --format json\n      Emit JSON only on stdout. Exit 1 means check failures; exit 2 means operational failure.\n\n  prosesmasher check draft.md --config my-config.json --format json --include-checks\n      Include the full per-check results array in JSON output.\n\n  prosesmasher check draft.md --preset article-en --text-mode summary\n      Print one summary line per file.\n\n  prosesmasher check draft.md --preset article-en --text-mode paths\n      Print only failing file paths."
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
            .required(false)
            .multiple(false)
    ))]
    Check {
        /// File or directory to check (recursively finds .md files in directories).
        ///
        /// Required unless `--list-checks` is used.
        path: Option<PathBuf>,

        /// List available checks instead of validating files.
        #[arg(long)]
        list_checks: bool,

        /// Config file (JSON) to use for validation.
        ///
        /// Uses canonical camelCase config with `locale`, `quality`, and `documentPolicy`.
        #[arg(long)]
        config: Option<PathBuf>,

        /// Shipped preset policy profile to use instead of --config.
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

        /// Text output mode for human-readable runs.
        #[arg(long, default_value = "failures")]
        text_mode: TextMode,

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
        /// Dump a shipped preset profile by name.
        #[arg(long)]
        preset: Option<String>,

        /// Dump the full editable config example.
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

/// Text verbosity / shape for `--format text`.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TextMode {
    /// Print only failing checks plus a summary.
    Failures,
    /// Print every check line and a summary.
    Full,
    /// Print one summary line per file.
    Summary,
    /// Print only failing file paths.
    Paths,
}

#[cfg(test)]
#[path = "args_tests/mod.rs"]
mod tests;
