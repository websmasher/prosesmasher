use prosesmasher_app_checks_flow_runtime::WordRepetitionCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_evidence_len, assert_first_evidence_i64, assert_first_evidence_str,
};

crate::define_rule_assertions!(WordRepetitionCheck, "word-repetition", "Word Repetition");

pub fn assert_word_repetition_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_word: &str,
    expected_count: i64,
    expected_paragraph: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_evidence_len(&result, "word-repetition", 1);
    assert_first_evidence_str(&result, "word-repetition", "word", expected_word);
    assert_first_evidence_i64(&result, "word-repetition", "count", expected_count);
    assert_first_evidence_str(
        &result,
        "word-repetition",
        "paragraph_text",
        expected_paragraph,
    );
}
