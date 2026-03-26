#![allow(clippy::disallowed_methods, clippy::panic)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[must_use]
pub fn cargo_bin() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

#[must_use]
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap_or_else(|e| panic!("workspace root: {e}"))
}

#[must_use]
pub fn fixture_path() -> PathBuf {
    workspace_root().join("crates/adapters/inbound/cli/tests/fixtures/test-essay.md")
}

#[must_use]
pub fn run_workspace_command(args: &[&str]) -> Output {
    Command::new(cargo_bin())
        .current_dir(workspace_root())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("command failed to start: {e}"))
}

#[must_use]
pub fn run_wrapper(args: &[&str]) -> Output {
    let mut command = vec!["run", "-q", "-p", "prosesmasher", "--"];
    command.extend_from_slice(args);
    run_workspace_command(&command)
}

#[must_use]
pub fn stdout_string(output: &Output, message: &str) -> String {
    String::from_utf8(output.stdout.clone()).unwrap_or_else(|e| panic!("{message}: {e}"))
}

#[must_use]
pub fn stderr_string(output: &Output, message: &str) -> String {
    String::from_utf8(output.stderr.clone()).unwrap_or_else(|e| panic!("{message}: {e}"))
}

pub fn assert_success(output: &Output, message: &str) {
    assert!(output.status.success(), "{message}");
}

pub fn assert_exit_code(output: &Output, expected: i32, message: &str) {
    assert_eq!(output.status.code(), Some(expected), "{message}");
}

pub fn assert_stdout_contains(output: &Output, expected: &str, message: &str) {
    let stdout = stdout_string(output, "stdout utf8");
    assert!(
        stdout.contains(expected),
        "{message}: expected `{expected}` in stdout, got `{stdout}`"
    );
}

pub fn assert_stderr_empty(output: &Output, message: &str) {
    let stderr = stderr_string(output, "stderr utf8");
    assert!(stderr.is_empty(), "{message}: got stderr `{stderr}`");
}
