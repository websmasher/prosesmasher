//! prosesmasher CLI entry point.

use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use low_expectations as _;
use serde as _;
use serde_json as _;
use walkdir as _;

type BoxError = Box<dyn std::error::Error>;
type CliResult = Result<(), BoxError>;
type ConfigResult = Result<prosesmasher_domain_types::CheckConfig, BoxError>;

use prosesmasher_adapters_inbound_cli::args::{Args, Command, OutputFormat};
use prosesmasher_adapters_inbound_cli::checks::{collect_checks, filter_checks_by_id};
use prosesmasher_adapters_inbound_cli::collect_files;
use prosesmasher_adapters_inbound_cli::output::output_result;
use prosesmasher_adapters_outbound_fs::{
    FsConfigLoader, FsFileReader, full_config_path, preset_path, shipped_presets,
};
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
            preset,
            group,
            check,
            format,
        } => run_check_command(
            &path,
            config.as_deref(),
            preset.as_deref(),
            group.as_deref(),
            check.as_deref(),
            &format,
        ),
        Command::ListPresets => {
            run_list_presets_command();
            Ok(())
        }
        Command::DumpConfig { preset, full_config } => {
            run_dump_config_command(preset.as_deref(), full_config)
        }
    }
}

fn run_check_command(
    path: &std::path::Path,
    config_path: Option<&std::path::Path>,
    preset_name: Option<&str>,
    group: Option<&str>,
    check_ids: Option<&str>,
    format: &OutputFormat,
) -> CliResult {
    let file_reader = FsFileReader;
    let config_loader = FsConfigLoader;
    let parser = MarkdownParser;

    let check_config = load_check_config(&config_loader, config_path, preset_name)?;

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

fn load_check_config(
    config_loader: &FsConfigLoader,
    config_path: Option<&Path>,
    preset_name: Option<&str>,
) -> ConfigResult {
    match (config_path, preset_name) {
        (Some(path), None) => Ok(config_loader.load_config(path)?),
        (None, Some(name)) => {
            let preset = preset_path(name)
                .ok_or_else(|| format!("Unknown preset: {name}. Run `prosesmasher list-presets`."))?;
            Ok(config_loader.load_config(&preset)?)
        }
        (None, None) => Err("Use exactly one config source: --preset <name> or --config <path>.".into()),
        (Some(_), Some(_)) => Err("Use either --config or --preset, not both.".into()),
    }
}

#[allow(clippy::print_stdout)] // reason: CLI writes user-facing output to stdout
fn run_list_presets_command() {
    println!("Available presets:");
    for preset in shipped_presets() {
        println!("  {:<12} {}", preset.name, preset.description);
    }
}

fn run_dump_config_command(
    preset_name: Option<&str>,
    full_config: bool,
) -> CliResult {
    let path = match (preset_name, full_config) {
        (Some(name), false) => preset_path(name)
            .ok_or_else(|| format!("Unknown preset: {name}. Run `prosesmasher list-presets`."))?,
        (None, true) => full_config_path(),
        _ => return Err("Use exactly one dump source: --preset <name> or --full-config.".into()),
    };

    let content = FsFileReader.read_to_string(&path)?;
    let mut stdout = io::stdout();
    stdout.write_all(content.as_bytes())?;
    Ok(())
}
