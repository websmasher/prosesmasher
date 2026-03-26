#![allow(clippy::disallowed_methods, clippy::panic)]

use prosesmasher_adapters_inbound_cli as _;
use prosesmasher_assertions::packaged_cli_smoke::{
    assert_exit_code, assert_stderr_empty, assert_stdout_contains, assert_success, fixture_path,
    run_workspace_command, run_wrapper,
};

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|err| panic!("clock drift: {err}"))
        .as_nanos();
    std::env::temp_dir().join(format!(
        "prosesmasher-package-smoke-{label}-{}-{nanos}",
        std::process::id()
    ))
}

fn write_file(path: &std::path::Path, contents: &str) {
    fs::write(path, contents).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
}

#[test]
fn packaged_fs_crate_contains_preset_assets() {
    let output = run_workspace_command(&[
        "package",
        "--list",
        "-p",
        "prosesmasher-adapters-outbound-fs-runtime",
    ]);

    assert_success(&output, "cargo package --list should succeed");
    assert_stdout_contains(
        &output,
        "presets/article-en.json",
        "the article preset should be shipped in the packaged fs crate",
    );
    assert_stdout_contains(
        &output,
        "presets/full-config-en.json",
        "the full config example should be shipped in the packaged fs crate",
    );
}

#[test]
fn wrapper_binary_lists_checks() {
    let output = run_wrapper(&["check", "--list-checks", "--format", "json"]);

    assert_success(
        &output,
        "check --list-checks should succeed through the packaged binary",
    );
    assert_stderr_empty(&output, "list-checks should not emit stderr");
    assert_stdout_contains(
        &output,
        "\"schema_version\": 1",
        "list-checks JSON should expose the schema version",
    );
    assert_stdout_contains(
        &output,
        "\"id\": \"word-count\"",
        "list-checks JSON should include known checks",
    );
}

#[test]
fn wrapper_binary_lists_presets() {
    let output = run_wrapper(&["list-presets"]);

    assert_success(
        &output,
        "list-presets should succeed through the packaged binary",
    );
    assert_stderr_empty(&output, "list-presets should not emit stderr");
    assert_stdout_contains(
        &output,
        "Available presets:",
        "list-presets should print the shipped preset header",
    );
    assert_stdout_contains(
        &output,
        "general-en",
        "list-presets should include general-en",
    );
}

#[test]
fn wrapper_binary_reports_release_version() {
    let output = run_wrapper(&["--version"]);

    assert_success(
        &output,
        "--version should succeed through the packaged binary",
    );
    assert_stderr_empty(&output, "--version should not emit stderr");
    assert_stdout_contains(
        &output,
        env!("CARGO_PKG_VERSION"),
        "--version should print the packaged release version",
    );
}

#[test]
fn wrapper_binary_dump_config_uses_preset_assets() {
    let output = run_wrapper(&["dump-config", "--preset", "article-en"]);

    assert_success(&output, "dump-config should succeed");
    assert_stdout_contains(
        &output,
        "\"locale\": \"en\"",
        "dump-config should print a JSON config",
    );
}

#[test]
fn wrapper_binary_dump_config_full_uses_embedded_example() {
    let output = run_wrapper(&["dump-config", "--full-config"]);

    assert_success(&output, "dump-config --full-config should succeed");
    assert_stderr_empty(&output, "dump-config --full-config should not emit stderr");
    assert_stdout_contains(
        &output,
        "\"quality\": {",
        "dump-config --full-config should print the full canonical config",
    );
    assert_stdout_contains(
        &output,
        "\"documentPolicy\": {",
        "dump-config --full-config should include document policy",
    );
}

#[test]
fn wrapper_binary_summary_mode_success_path() {
    let dir = unique_temp_dir("summary-success");
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

    let output = run_wrapper(&[
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
        "--text-mode",
        "summary",
    ]);

    assert_success(&output, "summary-mode success path should exit zero");
    assert_stderr_empty(
        &output,
        "summary-mode success path should keep stderr empty",
    );
    assert_stdout_contains(
        &output,
        "PASS",
        "summary-mode success path should print a PASS summary line",
    );
    assert_stdout_contains(
        &output,
        markdown
            .to_str()
            .unwrap_or_else(|| panic!("markdown path utf8")),
        "summary-mode success path should print the checked file path",
    );
    assert_stdout_contains(
        &output,
        "1 checks: 1 passed, 0 failed",
        "summary-mode success path should print the summary counts",
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wrapper_binary_json_failure_path_preserves_contract() {
    let output = run_wrapper(&[
        "check",
        fixture_path()
            .to_str()
            .unwrap_or_else(|| panic!("fixture path utf8")),
        "--preset",
        "article-en",
        "--format",
        "json",
    ]);

    assert_exit_code(&output, 1, "failing content should exit with lint code 1");
    assert_stderr_empty(
        &output,
        "json mode should keep stderr empty for check failures",
    );
    assert_stdout_contains(
        &output,
        "\"schema_version\": 1",
        "json output should include the schema version",
    );
    assert_stdout_contains(
        &output,
        "\"exit_reason\": \"check-failures\"",
        "json output should include the lint-failure exit reason",
    );
}
