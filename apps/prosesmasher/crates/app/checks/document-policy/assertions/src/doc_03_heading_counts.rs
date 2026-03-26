use prosesmasher_app_checks_document_policy_runtime::HeadingCountsCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_failure_count, assert_first_evidence_i64, assert_success_count,
};

crate::define_rule_assertions!(HeadingCountsCheck, "heading-counts", "Heading Counts");

pub fn assert_h2_failure_observed(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_failure_count(&result, 1, message);
    assert_first_evidence_i64(&result, "h2-count", "observed", expected_observed);
}

pub fn assert_single_success(doc: &Document, config: &CheckConfig, message: &str) {
    let result = run(doc, config);
    assert_success_count(&result, 1, message);
}
