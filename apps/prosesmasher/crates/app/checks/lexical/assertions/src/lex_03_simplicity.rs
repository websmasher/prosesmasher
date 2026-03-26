use prosesmasher_app_checks_lexical_runtime::SimplicityCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_failure_count, assert_observed_strs, assert_success_count,
};

crate::define_rule_assertions!(SimplicityCheck, "simplicity", "Simplicity");

pub fn assert_complex_terms(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: &[&str],
    expected_successes: usize,
    expected_failures: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_observed_strs(&result, "simplicity", expected_observed);
    assert_success_count(&result, expected_successes, message);
    assert_failure_count(&result, expected_failures, message);
}
