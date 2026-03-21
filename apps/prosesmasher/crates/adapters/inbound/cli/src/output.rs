//! Output formatting for check results.

use std::path::Path;

use low_expectations::types::SuiteValidationResult;

/// Format a single check result line.
///
/// Returns a string like `"  PASS   Some Label                    42"`.
#[must_use]
pub fn format_line(success: bool, label: &str, observed: &str) -> String {
    let status = if success { "PASS" } else { "FAIL" };
    format!("  {status:6} {label:<30} {observed}")
}

/// Print check results for a single file to stdout.
#[allow(clippy::print_stdout)] // reason: CLI output is the product
pub fn print_result(file: &Path, result: &SuiteValidationResult) {
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

#[cfg(test)]
#[path = "output_tests.rs"]
mod tests;
