use prosesmasher_app_checks_lexical_runtime::RecommendedTermsCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_failure_count, assert_observed_i64, assert_success_count,
};

crate::define_rule_assertions!(
    RecommendedTermsCheck,
    "recommended-terms",
    "Recommended Terms"
);

pub fn assert_match_count(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: i64,
    expected_successes: usize,
    expected_failures: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_observed_i64(&result, "recommended-terms", expected_observed);
    assert_success_count(&result, expected_successes, message);
    assert_failure_count(&result, expected_failures, message);
}
