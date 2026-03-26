use prosesmasher_app_checks_rhetorical_framing_runtime::LlmOpenersCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(LlmOpenersCheck, "llm-openers", "LLM Openers", None);

pub fn assert_llm_opener_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "llm-openers", "matched_text", expected_match);
    assert_first_evidence_str(&result, "llm-openers", "sentence", expected_sentence);
}
