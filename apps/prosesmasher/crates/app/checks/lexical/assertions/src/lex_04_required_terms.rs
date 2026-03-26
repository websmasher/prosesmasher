use prosesmasher_app_checks_lexical_runtime::RequiredTermsCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_failure_count, assert_success_count,
};

crate::define_rule_assertions!(RequiredTermsCheck, "required-terms", "Required Terms");

pub fn assert_term_counts(
    doc: &Document,
    config: &CheckConfig,
    expected_successes: usize,
    expected_failures: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_success_count(&result, expected_successes, message);
    assert_failure_count(&result, expected_failures, message);
}
