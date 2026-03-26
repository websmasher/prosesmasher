use prosesmasher_app_checks_llm_slop_runtime::LlmVocabularyCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    LlmVocabularyCheck,
    "llm-vocabulary",
    "LLM Vocabulary",
    Some(&[Locale::En])
);

pub fn assert_vocabulary_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "llm-vocabulary", "matched_text", expected_match);
}
