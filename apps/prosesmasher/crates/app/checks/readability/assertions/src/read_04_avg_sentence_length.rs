use prosesmasher_app_checks_readability_runtime::AvgSentenceLengthCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_i64;

crate::define_rule_assertions!(
    AvgSentenceLengthCheck,
    "avg-sentence-length",
    "Average Sentence Length"
);

pub fn assert_average_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_average: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_i64(
        &result,
        "avg-sentence-length",
        "average_words_per_sentence",
        expected_average,
    );
}
