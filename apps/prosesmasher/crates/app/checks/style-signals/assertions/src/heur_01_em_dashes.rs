use prosesmasher_app_checks_style_signals_runtime::EmDashCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_evidence_len, assert_first_evidence_str, assert_first_evidence_u64,
};

crate::define_rule_assertions!(EmDashCheck, "em-dashes", "No Closed Em-Dashes", None);

pub fn assert_match_evidence(
    doc: &Document,
    config: &CheckConfig,
    expected_sentence: &str,
    expected_match_count: u64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_evidence_len(&result, "em-dashes", 1);
    assert_first_evidence_u64(&result, "em-dashes", "match_count", expected_match_count);
    assert_first_evidence_str(&result, "em-dashes", "sentence", expected_sentence);
}
