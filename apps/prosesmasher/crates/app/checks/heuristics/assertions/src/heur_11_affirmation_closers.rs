use prosesmasher_app_checks_heuristics_runtime::AffirmationClosersCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(
    AffirmationClosersCheck,
    "affirmation-closers",
    "Affirmation Closers",
    None
);

pub fn assert_affirmation_closer_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(
        &result,
        "affirmation-closers",
        "matched_text",
        expected_match,
    );
    assert_first_evidence_str(
        &result,
        "affirmation-closers",
        "sentence",
        expected_sentence,
    );
}
