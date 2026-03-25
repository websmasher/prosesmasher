use low_expectations::types::SuiteValidationResult;
use prosesmasher_app_checks_core_runtime::{Check, run_checks};
use prosesmasher_app_checks_test_support::result_helpers::assert_metadata;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

pub fn run(checks: &[&dyn Check], doc: &Document, config: &CheckConfig) -> SuiteValidationResult {
    run_checks(checks, doc, config)
}

pub fn assert_matching_locale_runs(checks: &[&dyn Check], doc: &Document, config: &CheckConfig) {
    let result = run(checks, doc, config);
    assert_eq!(
        result.statistics.evaluated_expectations, 1,
        "matching locale should run"
    );
}

pub fn assert_non_matching_locale_skips(
    checks: &[&dyn Check],
    doc: &Document,
    config: &CheckConfig,
) {
    let result = run(checks, doc, config);
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "non-matching locale should skip"
    );
}

pub fn assert_empty_checks_yield_empty(doc: &Document, config: &CheckConfig) {
    let result = run(&[], doc, config);
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no checks should evaluate"
    );
}

pub fn assert_check_metadata_for_locale(
    check: &dyn Check,
    expected_id: &str,
    expected_label: &str,
    locales: Option<&[Locale]>,
) {
    assert_metadata(check, expected_id, expected_label, locales);
}
