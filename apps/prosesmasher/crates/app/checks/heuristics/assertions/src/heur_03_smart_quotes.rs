use prosesmasher_app_checks_heuristics_runtime::SmartQuotesCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(SmartQuotesCheck, "smart-quotes", "No Smart Quotes", None);

pub fn assert_quote_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "smart-quotes", "matched_text", expected_match);
    assert_first_evidence_str(&result, "smart-quotes", "sentence", expected_sentence);
}
