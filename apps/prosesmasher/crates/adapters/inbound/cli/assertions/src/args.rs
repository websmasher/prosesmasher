use clap::Parser;
use prosesmasher_adapters_inbound_cli_runtime::CliExit;
use prosesmasher_adapters_inbound_cli_runtime::args::{Args, Command, OutputFormat, TextMode};

#[derive(Debug)]
pub enum ConfigSource<'a> {
    None,
    Config(&'a str),
    Preset(&'a str),
}

fn output_format_name(actual: &OutputFormat) -> &'static str {
    match actual {
        OutputFormat::Text => "text",
        OutputFormat::Json => "json",
    }
}

fn text_mode_name(actual: &TextMode) -> &'static str {
    match actual {
        TextMode::Failures => "failures",
        TextMode::Full => "full",
        TextMode::Summary => "summary",
        TextMode::Paths => "paths",
    }
}

pub fn parse_ok<I, T>(argv: I) -> Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    match Args::try_parse_from(argv) {
        Ok(args) => args,
        Err(err) => panic!("parse failed: {err}"),
    }
}

pub fn assert_parse_err<I, T>(argv: I, message: &str)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    assert!(Args::try_parse_from(argv).is_err(), "{message}");
}

pub fn assert_parse_ok<I, T>(argv: I, message: &str)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    assert!(Args::try_parse_from(argv).is_ok(), "{message}");
}

#[allow(clippy::too_many_arguments)]
pub fn assert_check_command(
    args: &Args,
    expected_path: Option<&str>,
    expected_list_checks: bool,
    expected_config_source: ConfigSource<'_>,
    expected_group: Option<&str>,
    expected_check_filter: Option<&str>,
    expected_format: &str,
    expected_text_mode: &str,
    expected_include_checks: bool,
    message: &str,
) {
    match &args.command {
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
        } => {
            let actual_path = path.as_deref().and_then(std::path::Path::to_str);
            assert_eq!(actual_path, expected_path, "{message}: path");
            assert_eq!(*list_checks, expected_list_checks, "{message}: list_checks");
            match expected_config_source {
                ConfigSource::None => {
                    assert!(config.is_none(), "{message}: expected no config path");
                    assert!(preset.is_none(), "{message}: expected no preset");
                }
                ConfigSource::Config(expected) => {
                    assert_eq!(
                        config.as_deref().and_then(std::path::Path::to_str),
                        Some(expected),
                        "{message}: config path"
                    );
                    assert!(preset.is_none(), "{message}: preset should be absent");
                }
                ConfigSource::Preset(expected) => {
                    assert!(config.is_none(), "{message}: config should be absent");
                    assert_eq!(preset.as_deref(), Some(expected), "{message}: preset");
                }
            }
            assert_eq!(group.as_deref(), expected_group, "{message}: group");
            assert_eq!(
                check.as_deref(),
                expected_check_filter,
                "{message}: check filter"
            );
            assert!(
                output_format_name(format) == expected_format,
                "{message}: format"
            );
            assert!(
                text_mode_name(text_mode) == expected_text_mode,
                "{message}: text mode"
            );
            assert_eq!(
                *include_checks, expected_include_checks,
                "{message}: include_checks"
            );
        }
        Command::ListPresets | Command::DumpConfig { .. } => {
            panic!("{message}: expected `check` command")
        }
    }
}

pub fn assert_list_presets_command(args: &Args, message: &str) {
    assert!(
        matches!(args.command, Command::ListPresets),
        "{message}: expected `list-presets` command"
    );
}

pub fn assert_dump_config_full_command(args: &Args, message: &str) {
    match &args.command {
        Command::DumpConfig {
            preset,
            full_config,
        } => {
            assert!(preset.is_none(), "{message}: preset should be absent");
            assert!(*full_config, "{message}: --full-config should be set");
        }
        Command::Check { .. } | Command::ListPresets => {
            panic!("{message}: expected `dump-config` command")
        }
    }
}

pub fn assert_dump_config_preset_command(args: &Args, expected_preset: &str, message: &str) {
    match &args.command {
        Command::DumpConfig {
            preset,
            full_config,
        } => {
            assert_eq!(
                preset.as_deref(),
                Some(expected_preset),
                "{message}: preset"
            );
            assert!(!full_config, "{message}: --full-config should be false");
        }
        Command::Check { .. } | Command::ListPresets => {
            panic!("{message}: expected `dump-config` command")
        }
    }
}

pub fn assert_cli_exit(actual: CliExit, expected: CliExit, message: &str) {
    assert_eq!(actual, expected, "{message}: unexpected CLI exit");
}

pub fn assert_cli_error_contains(
    err: &(dyn std::error::Error + 'static),
    expected: &str,
    message: &str,
) {
    let actual = err.to_string();
    assert!(
        actual.contains(expected),
        "{message}: expected error containing `{expected}`, got `{actual}`"
    );
}
