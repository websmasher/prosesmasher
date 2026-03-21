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

        /// Only run checks from this group: terms, patterns, structure, readability.
        #[arg(long)]
        group: Option<String>,
    },
}
