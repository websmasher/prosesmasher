use super::*;
use clap::Parser;

#[test]
#[allow(clippy::panic)] // test assertion
fn parse_check_with_file() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { path, config, group, format } => {
            assert_eq!(path.to_str(), Some("foo.md"), "path");
            assert!(config.is_none(), "no config");
            assert!(group.is_none(), "no group");
            assert!(matches!(format, OutputFormat::Text), "default format is text");
        }
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
    }
}

#[test]
#[allow(clippy::panic)] // test assertion
fn parse_check_with_group() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md", "--group", "terms"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { group, .. } => {
            assert_eq!(group.as_deref(), Some("terms"), "group");
        }
    }
}

#[test]
fn parse_missing_path_fails() {
    let args = Args::try_parse_from(["prosesmasher", "check"]);
    assert!(args.is_err(), "missing path should fail");
}

#[test]
fn parse_no_subcommand_fails() {
    let args = Args::try_parse_from(["prosesmasher"]);
    assert!(args.is_err(), "no subcommand should fail");
}

#[test]
#[allow(clippy::panic)]
fn parse_check_with_format_json() {
    let args = Args::try_parse_from(["prosesmasher", "check", "foo.md", "--format", "json"]);
    assert!(args.is_ok(), "should parse");
    let args = args.unwrap_or_else(|e| panic!("parse failed: {e}"));
    match args.command {
        Command::Check { format, .. } => {
            assert!(matches!(format, OutputFormat::Json), "format should be json");
        }
    }
}
