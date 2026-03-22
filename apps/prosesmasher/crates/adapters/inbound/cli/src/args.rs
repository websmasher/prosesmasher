//! CLI argument parsing via clap.

use std::path::PathBuf;

use clap::Parser;

/// Prose quality checker — deterministic AI slop detection.
#[derive(Parser, Debug)]
#[command(name = "prosesmasher", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands.
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Check markdown files for prose quality issues.
    Check {
        /// File or directory to check (recursively finds .md files in directories).
        path: PathBuf,

        /// Config file (JSON) with term lists and thresholds.
        #[arg(long)]
        config: Option<PathBuf>,

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
