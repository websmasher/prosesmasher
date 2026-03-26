use prosesmasher_app_checks_lexical_runtime::ProhibitedTermsCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_evidence_len, assert_failure_count, assert_observed_strs,
};

crate::define_rule_assertions!(ProhibitedTermsCheck, "prohibited-terms", "Prohibited Terms");

pub fn assert_prohibited_matches(
    doc: &Document,
    config: &CheckConfig,
    expected_observed: &[&str],
    expected_evidence_len: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_failure_count(&result, 1, message);
    assert_observed_strs(&result, "prohibited-terms", expected_observed);
    assert_evidence_len(&result, "prohibited-terms", expected_evidence_len);
}
