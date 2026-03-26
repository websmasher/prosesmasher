use prosesmasher_app_checks_persona_signals_runtime::JargonFakerCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(JargonFakerCheck, "jargon-faker", "Jargon Faker", None);

pub fn assert_jargon_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "jargon-faker", "matched_text", expected_match);
    assert_first_evidence_str(&result, "jargon-faker", "sentence", expected_sentence);
}
