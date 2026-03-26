use prosesmasher_app_checks_style_signals_runtime::ColonDramaticCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(ColonDramaticCheck, "colon-dramatic", "Dramatic Colon", None);

pub fn assert_dramatic_colon_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "colon-dramatic", "matched_text", expected_match);
    assert_first_evidence_str(&result, "colon-dramatic", "sentence", expected_sentence);
}
