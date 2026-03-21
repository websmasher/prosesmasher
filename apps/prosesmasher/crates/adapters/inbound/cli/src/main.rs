//! prosesmasher CLI entry point.

use std::process::ExitCode;

use clap::Parser;
use low_expectations as _;
use serde as _;
use serde_json as _;
use walkdir as _;

type CliResult = Result<(), Box<dyn std::error::Error>>;

use prosesmasher_adapters_inbound_cli::args::{Args, Command, OutputFormat};
use prosesmasher_adapters_inbound_cli::checks::{collect_checks, filter_checks_by_id};
use prosesmasher_adapters_inbound_cli::collect_files;
use prosesmasher_adapters_inbound_cli::output::output_result;
use prosesmasher_adapters_outbound_fs::FsConfigLoader;
use prosesmasher_adapters_outbound_fs::FsFileReader;
use prosesmasher_adapters_outbound_parser::MarkdownParser;
use prosesmasher_app_core::check::Check;
use prosesmasher_app_core::runner::run_checks;
use prosesmasher_ports_outbound_traits::{ConfigLoader, DocumentParser, FileReader};

#[allow(clippy::print_stderr)] // reason: CLI error reporting
fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> CliResult {
    match args.command {
        Command::Check {
            path,
            config,
            group,
            check,
            format,
        } => run_check_command(&path, config.as_deref(), group.as_deref(), check.as_deref(), &format),
    }
}

fn run_check_command(
    path: &std::path::Path,
    config_path: Option<&std::path::Path>,
    group: Option<&str>,
    check_ids: Option<&str>,
    format: &OutputFormat,
) -> CliResult {
    let file_reader = FsFileReader;
    let config_loader = FsConfigLoader;
    let parser = MarkdownParser;

    let check_config = match config_path {
        Some(p) => config_loader.load_config(p)?,
        None => prosesmasher_domain_types::CheckConfig::default(),
    };

    let files = collect_files(path);
    if files.is_empty() {
        return Err("No .md files found".into());
    }

    // Collect checks: group filter first, then ID filter
    let mut all_checks = collect_checks(group).map_err(|e| -> Box<dyn std::error::Error> {
        e.into()
    })?;

    if let Some(ids) = check_ids {
        all_checks = filter_checks_by_id(all_checks, ids).map_err(|e| -> Box<dyn std::error::Error> {
            e.into()
        })?;
    }

    let mut any_failed = false;
    for file in &files {
        let content = file_reader.read_to_string(file)?;
        let doc = parser.parse(&content, &check_config.locale)?;
        let check_refs: Vec<&dyn Check> = all_checks.iter().map(AsRef::as_ref).collect();
        let result = run_checks(&check_refs, &doc, &check_config);

        output_result(file, &result, format);

        if !result.success {
            any_failed = true;
        }
    }

    if any_failed {
        return Err("One or more checks failed".into());
    }

    Ok(())
}
