use low_expectations::ExpectationSuite;
use low_expectations::types::SuiteValidationResult;
use prosesmasher_app_checks_core_runtime::Check;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

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
