//! Output formatting for check results.

use std::collections::BTreeSet;
use std::path::Path;

use low_expectations::types::{Severity, SuiteValidationResult, ValidationResult};
use serde_json::Value;
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
    /// Summary block for rewrite-oriented consumers.
    pub summary: ResultSummary,
    /// Whether a rewrite should be requested.
    pub rewrite_needed: bool,
    /// Deterministic rewrite instructions derived from failed checks.
    pub rewrite_brief: Vec<String>,
    /// Failed checks with deterministic rewrite guidance.
    pub failures: Vec<FailureOutput>,
    /// Individual check results.
    pub checks: Vec<CheckOutput>,
}

/// Summary counts for a single file result.
#[derive(Debug, Serialize)]
pub struct ResultSummary {
    /// Number of checks evaluated.
    pub evaluated: usize,
    /// Number of checks that passed.
    pub passed: usize,
    /// Number of checks that failed.
    pub failed: usize,
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

/// A failed check represented for rewrite loops.
#[derive(Debug, Serialize)]
pub struct FailureOutput {
    /// Check column/ID.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Severity for orchestration layers.
    pub severity: &'static str,
    /// Deterministic explanation of the failure.
    pub message: String,
    /// What the expectation was checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking: Option<String>,
    /// Structured expected values derived from expectation kwargs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Value>,
    /// The observed value (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed: Option<serde_json::Value>,
    /// Deterministic evidence derived from failing values when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<Value>>,
    /// Deterministic rewrite instruction for the model.
    pub rewrite_hint: &'static str,
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
    let file_result = build_file_result(file, result);

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
            let label = check_label(column, &vr.expectation_config.meta).to_owned();
            CheckOutput {
                id: column.clone(),
                label,
                success: vr.success,
                observed: vr.result.observed_value.clone(),
            }
        })
        .collect();

    let failures: Vec<FailureOutput> = result
        .results
        .iter()
        .filter(|(_, vr)| !vr.success)
        .map(|(column, vr)| {
            let label = check_label(column, &vr.expectation_config.meta).to_owned();
            let message = failure_message(column, &label);
            FailureOutput {
                id: column.clone(),
                label,
                severity: failure_severity(vr),
                message,
                checking: check_checking(&vr.expectation_config.meta),
                expected: expected_value(vr),
                observed: vr.result.observed_value.clone(),
                evidence: vr.result.partial_unexpected_list.clone(),
                rewrite_hint: rewrite_hint(column),
            }
        })
        .collect();

    let rewrite_brief = build_rewrite_brief(&failures);
    let summary = ResultSummary {
        evaluated: result.statistics.evaluated_expectations,
        passed: result.statistics.successful_expectations,
        failed: result.statistics.unsuccessful_expectations,
    };

    FileResult {
        file: file.display().to_string(),
        success: result.success,
        evaluated: summary.evaluated,
        passed: summary.passed,
        failed: summary.failed,
        summary,
        rewrite_needed: !result.success,
        rewrite_brief,
        failures,
        checks,
    }
}

fn check_label<'a>(
    column: &'a str,
    meta: &'a std::collections::BTreeMap<String, Value>,
) -> &'a str {
    meta
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or(column)
}

fn build_rewrite_brief(failures: &[FailureOutput]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut brief = Vec::new();

    for failure in failures {
        let hint = failure.rewrite_hint.to_owned();
        if seen.insert(hint.clone()) {
            brief.push(hint);
        }
    }

    brief
}

const fn failure_severity(vr: &ValidationResult) -> &'static str {
    match vr.expectation_config.severity {
        Severity::Critical => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

fn check_checking(meta: &std::collections::BTreeMap<String, Value>) -> Option<String> {
    meta.get("checking")
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}

fn expected_value(vr: &ValidationResult) -> Option<Value> {
    let kwargs = &vr.expectation_config.kwargs;

    if let (Some(min), Some(max)) = (kwargs.get("min_value"), kwargs.get("max_value")) {
        return Some(serde_json::json!({
            "min": min,
            "max": max
        }));
    }

    if let Some(value_set) = kwargs.get("value_set") {
        return Some(serde_json::json!({
            "allowed": value_set
        }));
    }

    if let Some(expected) = kwargs.get("expected") {
        return Some(expected.clone());
    }

    None
}

fn failure_message(id: &str, label: &str) -> String {
    match id {
        "prohibited-terms" => "Found prohibited terms.".to_owned(),
        "hedge-stacking" => "Found sentences with too many hedging words.".to_owned(),
        "simplicity" => "Found terms that should be simplified.".to_owned(),
        "required-terms" => "Required terms are missing.".to_owned(),
        "recommended-terms" => "Not enough recommended terms were used.".to_owned(),
        "em-dashes" => "Found em dashes.".to_owned(),
        "smart-quotes" => "Found smart quotes.".to_owned(),
        "exclamation-density" => "One or more paragraphs use too many exclamation marks.".to_owned(),
        "negation-reframe" => "Found negation-reframe rhetoric.".to_owned(),
        "triple-repeat" => "Found repeated sentence openers.".to_owned(),
        "fake-timestamps" => "Found fabricated-looking timestamps.".to_owned(),
        "colon-dramatic" => "Found dramatic colon patterns.".to_owned(),
        "llm-openers" => "Found generic LLM-style openers.".to_owned(),
        "affirmation-closers" => "Found affirmation-style closers.".to_owned(),
        "summative-closer" => "Found summative closer patterns.".to_owned(),
        "false-question" => "Found false-question rhetoric.".to_owned(),
        "humble-bragger" => "Found humble-brag phrasing.".to_owned(),
        "jargon-faker" => "Found jargon-faker phrasing.".to_owned(),
        "word-count" => "Document word count is outside the configured range.".to_owned(),
        "heading-hierarchy" => "Heading hierarchy is invalid.".to_owned(),
        "h2-count" => "H2 heading count is outside the configured range.".to_owned(),
        "h3-count" => "H3 heading count is below the configured minimum.".to_owned(),
        "bold-density" => "Not enough bold emphasis was found.".to_owned(),
        "paragraph-length" => "One or more paragraphs are too long.".to_owned(),
        "sentence-case" => "One or more headings are not in sentence case.".to_owned(),
        "code-fences" => "Found code fences in prose content.".to_owned(),
        "word-repetition" => "Found repeated words above the configured threshold.".to_owned(),
        "flesch-kincaid" => "Readability is below the configured minimum.".to_owned(),
        "gunning-fog" => "Readability is above the configured maximum fog threshold.".to_owned(),
        "coleman-liau" => "Readability is above the configured Coleman-Liau threshold.".to_owned(),
        "avg-sentence-length" => "Average sentence length exceeds the configured maximum.".to_owned(),
        _ => format!("{label} failed."),
    }
}

fn rewrite_hint(id: &str) -> &'static str {
    match id {
        "prohibited-terms" => "Rewrite the affected text without the prohibited terms.",
        "hedge-stacking" => "Reduce hedging so each sentence makes a clearer claim.",
        "simplicity" => "Replace complex wording with the simpler configured alternatives.",
        "required-terms" => "Add the missing required terms naturally in relevant sections.",
        "recommended-terms" => "Add more recommended terms naturally until the minimum is met.",
        "em-dashes" => "Replace em dashes with commas, periods, or cleaner sentence splits.",
        "smart-quotes" => "Replace smart quotes with straight quotes.",
        "exclamation-density" => "Reduce exclamation marks in the affected paragraphs.",
        "negation-reframe" => "Rewrite the passage without the not-X-it-is-Y pattern.",
        "triple-repeat" => "Vary sentence openings instead of repeating the same opener pattern.",
        "fake-timestamps" => "Remove overly specific fabricated-looking timestamps unless they are necessary and real.",
        "colon-dramatic" => "Rewrite the sentence without the dramatic colon construction.",
        "llm-openers" => "Replace the generic opener with a direct, specific opening.",
        "affirmation-closers" => "Replace the affirmation-style closer with a concrete ending.",
        "summative-closer" => "Rewrite the ending without the summative closer formula.",
        "false-question" => "Replace the rhetorical question with a direct statement.",
        "humble-bragger" => "Remove the self-positioning phrasing and state the point directly.",
        "jargon-faker" => "Replace the fake-jargon phrasing with literal language.",
        "word-count" => "Adjust the document length to fit the configured word-count range.",
        "heading-hierarchy" => "Fix heading levels so they follow the configured hierarchy.",
        "h2-count" => "Adjust the number of H2 headings to fit the configured range.",
        "h3-count" => "Add more H3 headings if the structure needs them.",
        "bold-density" => "Add bold emphasis to enough paragraphs if scannability requires it.",
        "paragraph-length" => "Split long paragraphs into shorter ones.",
        "sentence-case" => "Rewrite headings into sentence case.",
        "code-fences" => "Remove code fences from prose content.",
        "word-repetition" => "Reduce repeated word usage in the affected text.",
        "flesch-kincaid" => "Rewrite for simpler readability with shorter, clearer sentences.",
        "gunning-fog" => "Rewrite for lower complexity and shorter sentences.",
        "coleman-liau" => "Rewrite with simpler words and shorter sentences.",
        "avg-sentence-length" => "Shorten sentences to reduce average sentence length.",
        _ => "Rewrite the affected text to satisfy this check.",
    }
}

#[cfg(test)]
#[path = "output_tests.rs"]
mod tests;
