use prosesmasher_app_checks_llm_slop_runtime::LlmDisclaimerCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    LlmDisclaimerCheck,
    "llm-disclaimer",
    "LLM Disclaimer",
    Some(&[Locale::En])
);

pub fn assert_disclaimer_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "llm-disclaimer", "matched_text", expected_match);
    assert_first_evidence_str(&result, "llm-disclaimer", "sentence", expected_sentence);
}
