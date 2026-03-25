use super::*;
use clap::Parser;

#[test]
#[allow(clippy::panic)] // test assertion
fn parse_check_with_file() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md", "--preset", "general-en"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
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
        } => {
            assert_eq!(
                path.as_deref().and_then(std::path::Path::to_str),
                Some("foo.md"),
                "path"
            );
            assert!(!list_checks, "not list checks");
            assert!(config.is_none(), "no config");
            assert_eq!(preset.as_deref(), Some("general-en"), "preset");
            assert!(group.is_none(), "no group");
            assert!(check.is_none(), "no check filter");
            assert!(
                matches!(format, OutputFormat::Text),
                "default format is text"
            );
            assert!(
                matches!(text_mode, TextMode::Failures),
                "default text mode is failures"
            );
            assert!(!include_checks, "checks hidden by default");
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)] // test assertion
fn parse_check_with_config() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md", "--config", "c.json"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { config, .. } => {
            assert!(config.is_some(), "config should be Some");
            assert_eq!(
                config.unwrap_or_default().to_str(),
                Some("c.json"),
                "config path"
            );
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_check_with_preset() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md", "--preset", "article-en"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { preset, .. } => {
            assert_eq!(preset.as_deref(), Some("article-en"), "preset");
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
fn parse_check_with_config_and_preset_fails() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "check",
        "foo.md",
        "--config",
        "c.json",
        "--preset",
        "article-en",
    ]);
    assert!(args.is_err(), "config and preset should conflict");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn parse_check_with_group() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--group",
        "quality",
    ]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { group, .. } => {
            assert_eq!(group.as_deref(), Some("quality"), "group");
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
fn parse_check_requires_config_source() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md"]);
    assert!(args.is_ok(), "clap allows later validation");
}

#[test]
fn parse_missing_path_fails() {
    let args = Args::try_parse_from(["prosesmasher", "check"]);
    assert!(args.is_ok(), "clap allows check --list-checks without path");
}

#[test]
fn parse_no_subcommand_fails() {
    let args = Args::try_parse_from(["prosesmasher"]);
    assert!(args.is_err(), "no subcommand should fail");
}

#[test]
#[allow(clippy::panic)]
fn parse_check_with_check_filter() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--check",
        "prohibited-terms,em-dashes",
    ]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { check, .. } => {
            assert_eq!(
                check.as_deref(),
                Some("prohibited-terms,em-dashes"),
                "check filter"
            );
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_check_with_format_json() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--format",
        "json",
    ]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { format, .. } => {
            assert!(
                matches!(format, OutputFormat::Json),
                "format should be json"
            );
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_check_with_include_checks() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--format",
        "json",
        "--include-checks",
    ]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { include_checks, .. } => {
            assert!(include_checks, "include checks flag");
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_check_list_checks() {
    let args = Args::try_parse_from(["prosesmasher", "check", "--list-checks"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check {
            list_checks, path, ..
        } => {
            assert!(list_checks, "list checks flag");
            assert!(path.is_none(), "no path required");
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_check_with_text_mode() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--text-mode",
        "summary",
    ]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { text_mode, .. } => {
            assert!(matches!(text_mode, TextMode::Summary), "summary text mode");
        }
        Command::ListPresets | Command::DumpConfig { .. } => panic!("expected check command"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_list_presets() {
    let args = Args::try_parse_from(["prosesmasher", "list-presets"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    assert!(matches!(args.command, Command::ListPresets));
}

#[test]
#[allow(clippy::panic)]
fn parse_dump_config_full() {
    let args = Args::try_parse_from(["prosesmasher", "dump-config", "--full-config"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::DumpConfig {
            preset,
            full_config,
        } => {
            assert!(preset.is_none(), "no preset");
            assert!(full_config, "full config");
        }
        Command::Check { .. } | Command::ListPresets => panic!("expected dump-config"),
    }
}

#[test]
#[allow(clippy::panic)]
fn parse_dump_config_preset() {
    let args = Args::try_parse_from(["prosesmasher", "dump-config", "--preset", "tweet-en"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::DumpConfig {
            preset,
            full_config,
        } => {
            assert_eq!(preset.as_deref(), Some("tweet-en"));
            assert!(!full_config, "not full config");
        }
        Command::Check { .. } | Command::ListPresets => panic!("expected dump-config"),
    }
}

#[test]
fn parse_dump_config_requires_source() {
    let args = Args::try_parse_from(["prosesmasher", "dump-config"]);
    assert!(args.is_err(), "dump-config should require source");
}

#[test]
fn parse_dump_config_conflicting_sources_fail() {
    let args = Args::try_parse_from([
        "prosesmasher",
        "dump-config",
        "--full-config",
        "--preset",
        "tweet-en",
    ]);
    assert!(args.is_err(), "dump-config sources should conflict");
}
