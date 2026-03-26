use prosesmasher_app_checks_document_policy_runtime::BoldDensityCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_failure_count, assert_observed_i64, assert_success_count,
};

crate::define_rule_assertions!(BoldDensityCheck, "bold-density", "Bold Density");

pub fn assert_bold_count(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: i64,
    expected_successes: usize,
    expected_failures: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_observed_i64(&result, "bold-density", expected_observed);
    assert_success_count(&result, expected_successes, message);
    assert_failure_count(&result, expected_failures, message);
}
