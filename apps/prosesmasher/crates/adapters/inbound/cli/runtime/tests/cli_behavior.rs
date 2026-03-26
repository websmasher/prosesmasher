#![allow(clippy::disallowed_methods, clippy::panic)]

use clap as _;
use low_expectations as _;
use prosesmasher_adapters_inbound_cli_assertions::args::{
    assert_cli_error_contains, assert_cli_exit, parse_ok,
};
use prosesmasher_adapters_inbound_cli_runtime as _;
use prosesmasher_adapters_inbound_cli_runtime::{CliExit, run};
use prosesmasher_adapters_outbound_fs as _;
use prosesmasher_adapters_outbound_parser as _;
use prosesmasher_app_checks_catalog_runtime as _;
use prosesmasher_app_checks_core_runtime as _;
use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
use serde as _;
use serde_json as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir as _;

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|err| panic!("clock drift: {err}"))
        .as_nanos();
    std::env::temp_dir().join(format!(
        "prosesmasher-cli-runtime-{label}-{}-{nanos}",
        std::process::id()
    ))
}

fn fixture_config_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/test-config.json")
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
}

fn run_ok(argv: &[&str], message: &str) -> CliExit {
    run(parse_ok(argv)).unwrap_or_else(|err| panic!("{message}: run failed: {err}"))
}

fn run_err(argv: &[&str], message: &str) -> Box<dyn std::error::Error> {
    run(parse_ok(argv)).expect_err(message)
}

#[test]
fn list_checks_succeeds_without_path() {
    assert_cli_exit(
        run_ok(
            &["prosesmasher", "check", "--list-checks"],
            "check --list-checks should succeed without a path",
        ),
        CliExit::Success,
        "check --list-checks should succeed without a path",
    );
}

#[test]
fn list_checks_rejects_path_argument() {
    assert_cli_error_contains(
        run_err(
            &["prosesmasher", "check", "draft.md", "--list-checks"],
            "check --list-checks should reject explicit paths",
        )
        .as_ref(),
        "Do not pass a path with --list-checks.",
        "check --list-checks should reject explicit paths",
    );
}

#[test]
fn list_checks_rejects_other_runtime_flags() {
    assert_cli_error_contains(
        run_err(
            &[
                "prosesmasher",
                "check",
                "--list-checks",
                "--preset",
                "article-en",
            ],
            "check --list-checks should reject config-source flags",
        )
        .as_ref(),
        "Use --list-checks by itself",
        "check --list-checks should reject config-source flags",
    );
}

#[test]
fn list_presets_returns_success() {
    assert_cli_exit(
        run_ok(
            &["prosesmasher", "list-presets"],
            "list-presets should succeed",
        ),
        CliExit::Success,
        "list-presets should succeed",
    );
}

#[test]
fn check_requires_exactly_one_config_source() {
    let dir = unique_temp_dir("missing-config-source");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));
    let markdown = dir.join("draft.md");
    write_file(&markdown, "Simple text with enough words to count cleanly.");

    assert_cli_error_contains(
        run_err(
            &[
                "prosesmasher",
                "check",
                markdown
                    .to_str()
                    .unwrap_or_else(|| panic!("markdown path utf8")),
            ],
            "check should reject missing config sources",
        )
        .as_ref(),
        "Use exactly one config source",
        "check should reject missing config sources",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn check_rejects_unknown_preset_names() {
    let dir = unique_temp_dir("unknown-preset");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));
    let markdown = dir.join("draft.md");
    write_file(&markdown, "Simple text with enough words to count cleanly.");

    assert_cli_error_contains(
        run_err(
            &[
                "prosesmasher",
                "check",
                markdown
                    .to_str()
                    .unwrap_or_else(|| panic!("markdown path utf8")),
                "--preset",
                "no-such-preset",
            ],
            "check should reject unknown preset names",
        )
        .as_ref(),
        "Unknown preset: no-such-preset",
        "check should reject unknown preset names",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn empty_markdown_directory_returns_no_md_files_error() {
    let dir = unique_temp_dir("empty-dir");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));

    assert_cli_error_contains(
        run_err(
            &[
                "prosesmasher",
                "check",
                dir.to_str().unwrap_or_else(|| panic!("dir path utf8")),
                "--config",
                fixture_config_path()
                    .to_str()
                    .unwrap_or_else(|| panic!("config path utf8")),
            ],
            "check should reject empty directories before running checks",
        )
        .as_ref(),
        "No .md files found",
        "check should reject empty directories before running checks",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn check_success_path_returns_success() {
    let dir = unique_temp_dir("check-success");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));
    let markdown = dir.join("draft.md");
    let config = dir.join("config.json");

    write_file(
        &markdown,
        "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu",
    );
    write_file(
        &config,
        r#"{
  "locale": "en",
  "documentPolicy": {
    "wordCount": { "min": 10, "max": 20 }
  }
}"#,
    );

    assert_cli_exit(
        run_ok(
            &[
                "prosesmasher",
                "check",
                markdown
                    .to_str()
                    .unwrap_or_else(|| panic!("markdown path utf8")),
                "--config",
                config
                    .to_str()
                    .unwrap_or_else(|| panic!("config path utf8")),
                "--check",
                "word-count",
            ],
            "check should return success when the selected check passes",
        ),
        CliExit::Success,
        "check should return success when the selected check passes",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn check_failures_return_lint_exit() {
    let dir = unique_temp_dir("check-failure");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));
    let markdown = dir.join("draft.md");
    let config = dir.join("config.json");

    write_file(&markdown, "alpha beta gamma delta epsilon");
    write_file(
        &config,
        r#"{
  "locale": "en",
  "documentPolicy": {
    "wordCount": { "min": 10, "max": 20 }
  }
}"#,
    );

    assert_cli_exit(
        run_ok(
            &[
                "prosesmasher",
                "check",
                markdown
                    .to_str()
                    .unwrap_or_else(|| panic!("markdown path utf8")),
                "--config",
                config
                    .to_str()
                    .unwrap_or_else(|| panic!("config path utf8")),
                "--check",
                "word-count",
            ],
            "check should return the lint-failure exit when selected checks fail",
        ),
        CliExit::CheckFailures,
        "check should return the lint-failure exit when selected checks fail",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn dump_config_full_config_returns_success() {
    assert_cli_exit(
        run_ok(
            &["prosesmasher", "dump-config", "--full-config"],
            "dump-config --full-config should succeed",
        ),
        CliExit::Success,
        "dump-config --full-config should succeed",
    );
}

#[test]
fn dump_config_rejects_unknown_preset_names() {
    assert_cli_error_contains(
        run_err(
            &["prosesmasher", "dump-config", "--preset", "no-such-preset"],
            "dump-config should reject unknown preset names",
        )
        .as_ref(),
        "Unknown preset: no-such-preset",
        "dump-config should reject unknown preset names",
    );
}

#[test]
fn check_rejects_unknown_group_names() {
    let dir = unique_temp_dir("unknown-group");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));
    let markdown = dir.join("draft.md");
    write_file(&markdown, "Simple text with enough words to count cleanly.");

    assert_cli_error_contains(
        run_err(
            &[
                "prosesmasher",
                "check",
                markdown
                    .to_str()
                    .unwrap_or_else(|| panic!("markdown path utf8")),
                "--preset",
                "general-en",
                "--group",
                "no-such-group",
            ],
            "check should reject unknown groups",
        )
        .as_ref(),
        "Unknown check group",
        "check should reject unknown groups",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn check_rejects_unknown_check_ids() {
    let dir = unique_temp_dir("unknown-check-id");
    fs::create_dir_all(&dir).unwrap_or_else(|err| panic!("mkdir {}: {err}", dir.display()));
    let markdown = dir.join("draft.md");
    write_file(&markdown, "Simple text with enough words to count cleanly.");

    assert_cli_error_contains(
        run_err(
            &[
                "prosesmasher",
                "check",
                markdown
                    .to_str()
                    .unwrap_or_else(|| panic!("markdown path utf8")),
                "--preset",
                "general-en",
                "--check",
                "no-such-check",
            ],
            "check should reject unknown check ids",
        )
        .as_ref(),
        "Unknown check IDs",
        "check should reject unknown check ids",
    );

    let _ = fs::remove_dir_all(&dir);
}
