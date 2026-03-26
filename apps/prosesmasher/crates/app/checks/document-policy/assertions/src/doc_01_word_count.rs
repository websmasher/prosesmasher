use prosesmasher_app_checks_document_policy_runtime::WordCountCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_expectation_kwarg_i64, assert_observed_i64,
};

crate::define_rule_assertions!(WordCountCheck, "word-count", "Word Count");

pub fn assert_range_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: i64,
    expected_min: i64,
    expected_max: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_observed_i64(&result, "word-count", expected_observed);
    assert_expectation_kwarg_i64(&result, "word-count", "min_value", expected_min);
    assert_expectation_kwarg_i64(&result, "word-count", "max_value", expected_max);
}

pub fn assert_observed_word_count(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_pass(&result, message);
    assert_observed_i64(&result, "word-count", expected_observed);
}
