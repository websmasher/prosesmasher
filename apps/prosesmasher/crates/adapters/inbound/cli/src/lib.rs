//! CLI adapter — composition root and argument handling.

use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
pub mod args;
pub mod checks;
pub mod output;

type BoxError = Box<dyn std::error::Error>;
type CliResult = Result<CliExit, BoxError>;
type ConfigResult = Result<prosesmasher_domain_types::CheckConfig, BoxError>;

use crate::args::{Args, Command, OutputFormat, TextMode};
use crate::checks::{collect_checks, filter_checks_by_id, list_checks};
use crate::output::{output_result, print_check_catalog};
use prosesmasher_adapters_outbound_fs::{
    FsConfigLoader, FsFileReader, full_config_contents, preset_contents, shipped_presets,
};
use prosesmasher_adapters_outbound_fs::config_loader::parse_config_json;
use prosesmasher_adapters_outbound_parser::MarkdownParser;
use prosesmasher_app_core::check::Check;
use prosesmasher_app_core::runner::run_checks;
use prosesmasher_ports_outbound_traits::{ConfigLoader, DocumentParser, FileReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliExit {
    Success,
    CheckFailures,
}

#[derive(Debug, Clone, Copy)]
struct CheckCommandInput<'a> {
    path: Option<&'a std::path::Path>,
    list_checks_only: bool,
    config_path: Option<&'a std::path::Path>,
    preset_name: Option<&'a str>,
    group: Option<&'a str>,
    check_ids: Option<&'a str>,
    format: &'a OutputFormat,
    text_mode: &'a TextMode,
    include_checks: bool,
}

/// Collect markdown files from a path.
///
/// If the path is a directory, recursively finds all `.md` files and returns
/// them sorted. If the path is a single file, returns it as-is (regardless
/// of extension — the user explicitly chose it).
pub fn collect_files(path: &std::path::Path) -> Vec<PathBuf> {
    if path.is_dir() {
        let mut files: Vec<PathBuf> = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().is_some_and(|ext| ext == "md")
            })
            .map(walkdir::DirEntry::into_path)
            .collect();
        files.sort();
        files
    } else {
        vec![path.to_path_buf()]
    }
}

/// Parse CLI args from the process environment and execute the command.
#[must_use]
#[allow(clippy::print_stderr)] // reason: CLI error reporting
pub fn main_entry() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(CliExit::Success) => ExitCode::SUCCESS,
        Ok(CliExit::CheckFailures) => ExitCode::from(1),
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::from(2)
        }
    }
}

/// Run the CLI with already-parsed args.
///
/// # Errors
///
/// Returns an error when argument combinations are invalid, input files or
/// configs cannot be read, markdown cannot be parsed, or one or more selected
/// checks fail.
pub fn run(args: Args) -> CliResult {
    match args.command {
        Command::Check {
            path,
            list_checks,
            config,
            preset,
            group,
            check,
            format,
            text_mode,
            include_checks,
        } => run_check_command(CheckCommandInput {
            path: path.as_deref(),
            list_checks_only: list_checks,
            config_path: config.as_deref(),
            preset_name: preset.as_deref(),
            group: group.as_deref(),
            check_ids: check.as_deref(),
            format: &format,
            text_mode: &text_mode,
            include_checks,
        }),
        Command::ListPresets => {
            run_list_presets_command();
            Ok(CliExit::Success)
        }
        Command::DumpConfig { preset, full_config } => {
            run_dump_config_command(preset.as_deref(), full_config)
        }
    }
}

fn run_check_command(input: CheckCommandInput<'_>) -> CliResult {
    if input.list_checks_only {
        if input.path.is_some() {
            return Err("Do not pass a path with --list-checks.".into());
        }
        if input.config_path.is_some() || input.preset_name.is_some() || input.check_ids.is_some() {
            return Err("Use --list-checks by itself, optionally with --group and --format.".into());
        }
        let entries = list_checks(input.group).map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
        print_check_catalog(&entries, input.format);
        return Ok(CliExit::Success);
    }

    let path = input.path.ok_or("Missing path. Pass a file or directory, or use --list-checks.")?;

    let file_reader = FsFileReader;
    let config_loader = FsConfigLoader;
    let parser = MarkdownParser;

    let check_config = load_check_config(&config_loader, input.config_path, input.preset_name)?;

    let files = collect_files(path);
    if files.is_empty() {
        return Err("No .md files found".into());
    }

    let mut all_checks = collect_checks(input.group).map_err(|e| -> Box<dyn std::error::Error> {
        e.into()
    })?;

    if let Some(ids) = input.check_ids {
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

        output_result(file, &result, input.format, input.text_mode, input.include_checks);

        if !result.success {
            any_failed = true;
        }
    }

    if any_failed {
        return Ok(CliExit::CheckFailures);
    }

    Ok(CliExit::Success)
}

fn load_check_config(
    config_loader: &FsConfigLoader,
    config_path: Option<&Path>,
    preset_name: Option<&str>,
) -> ConfigResult {
    match (config_path, preset_name) {
        (Some(path), None) => Ok(config_loader.load_config(path)?),
        (None, Some(name)) => {
            let preset = preset_contents(name)
                .ok_or_else(|| format!("Unknown preset: {name}. Run `prosesmasher list-presets`."))?;
            Ok(parse_config_json(preset)?)
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
    let content = match (preset_name, full_config) {
        (Some(name), false) => preset_contents(name)
            .ok_or_else(|| format!("Unknown preset: {name}. Run `prosesmasher list-presets`."))?,
        (None, true) => full_config_contents(),
        _ => return Err("Use exactly one dump source: --preset <name> or --full-config.".into()),
    };

    let mut stdout = io::stdout();
    stdout.write_all(content.as_bytes())?;
    Ok(CliExit::Success)
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
