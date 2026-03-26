use prosesmasher_app_checks_heuristics_runtime::TripleRepeatCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(
    TripleRepeatCheck,
    "triple-repeat",
    "Triple Repeat Opener",
    None
);

pub fn assert_triple_repeat_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    sentence_1: &str,
    sentence_2: &str,
    sentence_3: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "triple-repeat", "matched_text", expected_match);
    assert_first_evidence_str(&result, "triple-repeat", "sentence_1", sentence_1);
    assert_first_evidence_str(&result, "triple-repeat", "sentence_2", sentence_2);
    assert_first_evidence_str(&result, "triple-repeat", "sentence_3", sentence_3);
}
