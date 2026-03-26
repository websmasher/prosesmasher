use std::path::Path;

use low_expectations::types::SuiteValidationResult;
use prosesmasher_adapters_inbound_cli_runtime::output::{
    FileResult, build_file_result, format_line,
};
use serde_json::Value;

#[derive(Debug)]
pub struct ExpectedCheck<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub kind: &'a str,
    pub success: bool,
    pub observed: Option<Value>,
}

#[derive(Debug)]
pub struct ExpectedFailure<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub kind: &'a str,
    pub severity: &'a str,
    pub message_contains: &'a str,
    pub checking: Option<&'a str>,
    pub expected: Option<Value>,
    pub observed: Option<Value>,
    pub evidence_len: Option<usize>,
    pub rewrite_hint_contains: &'a str,
}

#[must_use]
pub fn format(success: bool, label: &str, observed: &str) -> String {
    format_line(success, label, observed)
}

#[must_use]
pub fn build(path: &Path, result: &SuiteValidationResult, include_checks: bool) -> FileResult {
    build_file_result(path, result, include_checks)
}

pub fn assert_status_line(line: &str, success: bool, label: &str, observed: &str, message: &str) {
    let expected_status = if success { "PASS" } else { "FAIL" };
    assert!(
        line.contains(expected_status),
        "{message}: missing status `{expected_status}` in `{line}`"
    );
    assert!(
        line.contains(label),
        "{message}: missing label `{label}` in `{line}`"
    );
    assert!(
        observed.is_empty() || line.contains(observed),
        "{message}: missing observed `{observed}` in `{line}`"
    );
}

#[allow(clippy::too_many_arguments)]
pub fn assert_result_summary(
    file_result: &FileResult,
    expected_file: &str,
    expected_success: bool,
    expected_exit_reason: &str,
    expected_evaluated: usize,
    expected_passed: usize,
    expected_failed: usize,
    message: &str,
) {
    assert_eq!(file_result.schema_version, 1, "{message}: schema_version");
    assert_eq!(file_result.file, expected_file, "{message}: file");
    assert_eq!(
        file_result.success, expected_success,
        "{message}: success flag"
    );
    assert_eq!(
        file_result.exit_reason, expected_exit_reason,
        "{message}: exit reason"
    );
    assert_eq!(
        file_result.evaluated, expected_evaluated,
        "{message}: evaluated"
    );
    assert_eq!(file_result.passed, expected_passed, "{message}: passed");
    assert_eq!(file_result.failed, expected_failed, "{message}: failed");
    assert_eq!(
        file_result.summary.evaluated, expected_evaluated,
        "{message}: summary evaluated"
    );
    assert_eq!(
        file_result.summary.passed, expected_passed,
        "{message}: summary passed"
    );
    assert_eq!(
        file_result.summary.failed, expected_failed,
        "{message}: summary failed"
    );
    assert_eq!(
        file_result.rewrite_needed, !expected_success,
        "{message}: rewrite_needed"
    );
}

pub fn assert_checks_hidden(file_result: &FileResult, message: &str) {
    assert!(
        file_result.checks.is_none(),
        "{message}: checks should be hidden"
    );
}

pub fn assert_check_count(file_result: &FileResult, expected_len: usize, message: &str) {
    assert_eq!(
        file_result.checks.as_ref().map(Vec::len),
        Some(expected_len),
        "{message}: checks length"
    );
}

pub fn assert_check_present(file_result: &FileResult, expected: ExpectedCheck<'_>, message: &str) {
    let checks = file_result
        .checks
        .as_ref()
        .unwrap_or_else(|| panic!("{message}: checks should be present"));
    let check = checks
        .iter()
        .find(|check| check.id == expected.id)
        .unwrap_or_else(|| panic!("{message}: missing check `{}`", expected.id));

    assert_eq!(check.label, expected.label, "{message}: check label");
    assert_eq!(check.kind, expected.kind, "{message}: check kind");
    assert_eq!(check.success, expected.success, "{message}: check success");
    assert_eq!(
        check.observed, expected.observed,
        "{message}: check observed"
    );
}

pub fn assert_no_failures(file_result: &FileResult, message: &str) {
    assert!(
        file_result.failures.is_empty(),
        "{message}: failures should be empty"
    );
    assert!(
        file_result.rewrite_brief.is_empty(),
        "{message}: rewrite_brief should be empty"
    );
}

pub fn assert_rewrite_brief_contains(file_result: &FileResult, expected: &str, message: &str) {
    assert!(
        file_result
            .rewrite_brief
            .iter()
            .any(|brief| brief.contains(expected)),
        "{message}: rewrite_brief missing `{expected}`"
    );
}

pub fn assert_failure_count(file_result: &FileResult, expected_len: usize, message: &str) {
    assert_eq!(
        file_result.failures.len(),
        expected_len,
        "{message}: failure count"
    );
}

pub fn assert_failure_present(
    file_result: &FileResult,
    expected: ExpectedFailure<'_>,
    message: &str,
) {
    let failure = file_result
        .failures
        .iter()
        .find(|failure| failure.id == expected.id)
        .unwrap_or_else(|| panic!("{message}: missing failure `{}`", expected.id));

    assert_eq!(failure.label, expected.label, "{message}: failure label");
    assert_eq!(failure.kind, expected.kind, "{message}: failure kind");
    assert_eq!(
        failure.severity, expected.severity,
        "{message}: failure severity"
    );
    assert!(
        failure.message.contains(expected.message_contains),
        "{message}: failure message"
    );
    assert_eq!(
        failure.checking.as_deref(),
        expected.checking,
        "{message}: failure checking"
    );
    assert_eq!(
        failure.expected, expected.expected,
        "{message}: failure expected"
    );
    assert_eq!(
        failure.observed, expected.observed,
        "{message}: failure observed"
    );
    assert_eq!(
        failure.evidence.as_ref().map(Vec::len),
        expected.evidence_len,
        "{message}: failure evidence length"
    );
    assert!(
        failure
            .rewrite_hint
            .contains(expected.rewrite_hint_contains),
        "{message}: failure rewrite hint"
    );
}
