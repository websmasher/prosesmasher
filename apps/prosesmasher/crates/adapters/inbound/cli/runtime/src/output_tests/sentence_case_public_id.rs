use low_expectations::ExpectationSuite;
use prosesmasher_adapters_inbound_cli_assertions::output::{
    ExpectedCheck, ExpectedFailure, assert_check_present, assert_failure_present, build,
};
use std::path::Path;

#[test]
fn build_file_result_normalizes_sentence_case_internal_ids() {
    let mut suite = ExpectationSuite::new("test");
    let _result = suite
        .record_custom_values(
            "sentence-case-Why Some People Feel Anxious in Social Situations, and How to Manage It",
            false,
            serde_json::json!({
                "rule": "sentence case",
                "max_capitalized_non_first_words": 2
            }),
            serde_json::json!("Why Some People Feel Anxious in Social Situations, and How to Manage It"),
            &[serde_json::json!({
                "heading_text": "Why Some People Feel Anxious in Social Situations, and How to Manage It",
                "capitalized_non_first_words": 6,
                "sentence_case_expected": true
            })],
        )
        .label("Sentence Case")
        .checking(
            "heading: \"Why Some People Feel Anxious in Social Situations, and How to Manage It\"",
        );

    let result = suite.into_suite_result();
    let file_result = build(Path::new("draft.md"), &result, true);

    assert_check_present(
        &file_result,
        ExpectedCheck {
            id: "sentence-case",
            label: "Sentence Case",
            kind: "heuristics",
            success: false,
            observed: Some(serde_json::json!(
                "Why Some People Feel Anxious in Social Situations, and How to Manage It"
            )),
        },
        "sentence-case checks should expose a stable public id",
    );
    assert_failure_present(
        &file_result,
        ExpectedFailure {
            id: "sentence-case",
            label: "Sentence Case",
            kind: "heuristics",
            severity: "error",
            message_contains: "not in sentence case",
            checking: Some(
                "heading: \"Why Some People Feel Anxious in Social Situations, and How to Manage It\"",
            ),
            expected: Some(serde_json::json!({
                "rule": "sentence case",
                "max_capitalized_non_first_words": 2
            })),
            observed: Some(serde_json::json!(
                "Why Some People Feel Anxious in Social Situations, and How to Manage It"
            )),
            evidence_len: Some(1),
            rewrite_hint_contains: "sentence case",
        },
        "sentence-case failures should not leak heading text into the public id",
    );
}
