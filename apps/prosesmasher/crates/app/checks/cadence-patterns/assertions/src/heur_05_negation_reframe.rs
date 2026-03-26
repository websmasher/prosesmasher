use prosesmasher_app_checks_cadence_patterns_runtime::NegationReframeCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(
    NegationReframeCheck,
    "negation-reframe",
    "Negation-Reframe Pattern",
    None
);

pub fn assert_negation_reframe_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "negation-reframe", "matched_text", expected_match);
}

pub fn assert_negation_reframe_pair(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    expected_next_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "negation-reframe", "matched_text", expected_match);
    assert_first_evidence_str(&result, "negation-reframe", "sentence", expected_sentence);
    assert_first_evidence_str(
        &result,
        "negation-reframe",
        "next_sentence",
        expected_next_sentence,
    );
}
