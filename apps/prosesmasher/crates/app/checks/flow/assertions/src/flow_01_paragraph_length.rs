use prosesmasher_app_checks_flow_runtime::ParagraphLengthCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_evidence_len, assert_first_evidence_i64, assert_first_evidence_str,
};

crate::define_rule_assertions!(ParagraphLengthCheck, "paragraph-length", "Paragraph Length");

pub fn assert_paragraph_length_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_text: &str,
    expected_sentence_count: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_evidence_len(&result, "paragraph-length", 1);
    assert_first_evidence_str(&result, "paragraph-length", "paragraph_text", expected_text);
    assert_first_evidence_i64(
        &result,
        "paragraph-length",
        "sentence_count",
        expected_sentence_count,
    );
}
