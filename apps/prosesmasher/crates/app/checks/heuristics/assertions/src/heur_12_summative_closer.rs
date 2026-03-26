use prosesmasher_app_checks_heuristics_runtime::SummativeCloserCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(
    SummativeCloserCheck,
    "summative-closer",
    "Summative Closer",
    None
);

pub fn assert_summative_closer_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "summative-closer", "matched_text", expected_match);
    assert_first_evidence_str(&result, "summative-closer", "sentence", expected_sentence);
}
