//! Output formatting for check results.

use std::path::Path;

use low_expectations::types::SuiteValidationResult;
use serde::Serialize;

use crate::args::OutputFormat;

/// A file's check results — the top-level JSON output object.
#[derive(Debug, Serialize)]
pub struct FileResult {
    /// Path to the file that was checked.
    pub file: String,
    /// Whether all checks passed.
    pub success: bool,
    /// Number of checks evaluated.
    pub evaluated: usize,
    /// Number of checks that passed.
    pub passed: usize,
    /// Number of checks that failed.
    pub failed: usize,
    /// Individual check results.
    pub checks: Vec<CheckOutput>,
}

/// A single check result for JSON output.
#[derive(Debug, Serialize)]
pub struct CheckOutput {
    /// Check column/ID.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether this check passed.
    pub success: bool,
    /// The observed value (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed: Option<serde_json::Value>,
}

/// Format a single check result line for text output.
#[must_use]
pub fn format_line(success: bool, label: &str, observed: &str) -> String {
    let status = if success { "PASS" } else { "FAIL" };
    format!("  {status:6} {label:<30} {observed}")
}

/// Output check results for a single file.
#[allow(clippy::print_stdout)] // reason: CLI output is the product
pub fn output_result(
    file: &Path,
    result: &SuiteValidationResult,
    format: &OutputFormat,
) {
    match format {
        OutputFormat::Text => print_text(file, result),
        OutputFormat::Json => print_json(file, result),
    }
}

#[allow(clippy::print_stdout)]
fn print_text(file: &Path, result: &SuiteValidationResult) {
    println!("{}", file.display());

    for (column, vr) in &result.results {
        let label = vr
            .expectation_config
            .meta
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or(column);
        let observed = vr
            .result
            .observed_value
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();

        println!("{}", format_line(vr.success, label, &observed));
    }

    let stats = &result.statistics;
    println!(
        "\n{} checks: {} passed, {} failed\n",
        stats.evaluated_expectations,
        stats.successful_expectations,
        stats.unsuccessful_expectations,
    );
}

#[allow(clippy::print_stdout, clippy::disallowed_methods)] // JSON serialization output
fn print_json(file: &Path, result: &SuiteValidationResult) {
    let checks: Vec<CheckOutput> = result
        .results
        .iter()
        .map(|(column, vr)| {
            let label = vr
                .expectation_config
                .meta
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or(column)
                .to_owned();
            CheckOutput {
                id: column.clone(),
                label,
                success: vr.success,
                observed: vr.result.observed_value.clone(),
            }
        })
        .collect();

    let file_result = FileResult {
        file: file.display().to_string(),
        success: result.success,
        evaluated: result.statistics.evaluated_expectations,
        passed: result.statistics.successful_expectations,
        failed: result.statistics.unsuccessful_expectations,
        checks,
    };

    // serde_json::to_string is banned by clippy.toml — allow here as this IS the output
    if let Ok(json) = serde_json::to_string_pretty(&file_result) {
        println!("{json}");
    }
}

/// Build a `FileResult` from a suite result (for testing).
#[must_use]
pub fn build_file_result(file: &Path, result: &SuiteValidationResult) -> FileResult {
    let checks: Vec<CheckOutput> = result
        .results
        .iter()
        .map(|(column, vr)| {
            let label = vr
                .expectation_config
                .meta
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or(column)
                .to_owned();
            CheckOutput {
                id: column.clone(),
                label,
                success: vr.success,
                observed: vr.result.observed_value.clone(),
            }
        })
        .collect();

    FileResult {
        file: file.display().to_string(),
        success: result.success,
        evaluated: result.statistics.evaluated_expectations,
        passed: result.statistics.successful_expectations,
        failed: result.statistics.unsuccessful_expectations,
        checks,
    }
}

#[cfg(test)]
#[path = "output_tests.rs"]
mod tests;
