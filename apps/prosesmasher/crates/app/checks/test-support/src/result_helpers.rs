use low_expectations::ExpectationSuite;
use low_expectations::types::{SuiteValidationResult, ValidationResult};
use prosesmasher_app_checks_core_runtime::Check;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::Value;

#[must_use]
pub fn run_single_check(
    check: &dyn Check,
    doc: &Document,
    config: &CheckConfig,
) -> SuiteValidationResult {
    let mut suite = ExpectationSuite::new("test");
    check.run(doc, config, &mut suite);
    suite.into_suite_result()
}

pub fn assert_pass(result: &SuiteValidationResult, message: &str) {
    assert_eq!(result.statistics.successful_expectations, 1, "{message}");
    assert_eq!(result.statistics.unsuccessful_expectations, 0, "{message}");
}

pub fn assert_fail(result: &SuiteValidationResult, message: &str) {
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "{message}");
}

pub fn assert_skipped(result: &SuiteValidationResult, message: &str) {
    assert_eq!(result.statistics.evaluated_expectations, 0, "{message}");
}

#[must_use]
pub fn validation<'a>(result: &'a SuiteValidationResult, check_id: &str) -> &'a ValidationResult {
    result
        .results
        .get(check_id)
        .unwrap_or_else(|| panic!("missing validation result for `{check_id}`"))
}

#[must_use]
pub fn evidence<'a>(result: &'a SuiteValidationResult, check_id: &str) -> &'a [Value] {
    validation(result, check_id)
        .result
        .partial_unexpected_list
        .as_deref()
        .unwrap_or_else(|| panic!("missing evidence for `{check_id}`"))
}

pub fn assert_observed_i64(result: &SuiteValidationResult, check_id: &str, expected: i64) {
    assert_eq!(
        validation(result, check_id)
            .result
            .observed_value
            .as_ref()
            .and_then(Value::as_i64),
        Some(expected),
        "observed value for `{check_id}`"
    );
}

pub fn assert_expectation_kwarg_i64(
    result: &SuiteValidationResult,
    check_id: &str,
    key: &str,
    expected: i64,
) {
    assert_eq!(
        validation(result, check_id)
            .expectation_config
            .kwargs
            .get(key)
            .and_then(Value::as_i64),
        Some(expected),
        "expectation kwarg `{key}` for `{check_id}`"
    );
}

pub fn assert_evidence_len(result: &SuiteValidationResult, check_id: &str, expected: usize) {
    assert_eq!(evidence(result, check_id).len(), expected, "evidence len");
}

pub fn assert_first_evidence_str(
    result: &SuiteValidationResult,
    check_id: &str,
    key: &str,
    expected: &str,
) {
    assert_eq!(
        evidence(result, check_id)
            .first()
            .and_then(|item: &Value| item.get(key))
            .and_then(Value::as_str),
        Some(expected),
        "first evidence `{key}` for `{check_id}`"
    );
}

pub fn assert_first_evidence_u64(
    result: &SuiteValidationResult,
    check_id: &str,
    key: &str,
    expected: u64,
) {
    assert_eq!(
        evidence(result, check_id)
            .first()
            .and_then(|item: &Value| item.get(key))
            .and_then(Value::as_u64),
        Some(expected),
        "first evidence `{key}` for `{check_id}`"
    );
}

pub fn assert_metadata(
    check: &dyn Check,
    expected_id: &str,
    expected_label: &str,
    expected_locales: Option<&[Locale]>,
) {
    assert_eq!(check.id(), expected_id, "id");
    assert_eq!(check.label(), expected_label, "label");
    assert_eq!(
        check.supported_locales(),
        expected_locales,
        "supported locales"
    );
}
