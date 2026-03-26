use low_expectations::ExpectationSuite;
use prosesmasher_adapters_inbound_cli_assertions::output::{
    ExpectedCheck, ExpectedFailure, assert_check_count, assert_check_present, assert_checks_hidden,
    assert_failure_count, assert_failure_present, assert_no_failures, assert_result_summary,
    assert_rewrite_brief_contains, assert_status_line, build, format,
};
use std::path::Path;

#[test]
fn format_pass_contains_pass() {
    let line = format(true, "Some Check", "42");
    assert_status_line(&line, true, "Some Check", "42", "format pass line");
}

#[test]
fn format_fail_contains_fail() {
    let line = format(false, "Some Check", "99");
    assert_status_line(&line, false, "Some Check", "99", "format fail line");
}

#[test]
fn format_line_contains_label() {
    let line = format(true, "Word Count", "800");
    assert_status_line(&line, true, "Word Count", "800", "format line label");
}

#[test]
fn format_line_contains_observed() {
    let line = format(false, "Em Dashes", "3");
    assert_status_line(&line, false, "Em Dashes", "3", "format line observed value");
}

#[test]
fn format_line_empty_observed() {
    let line = format(true, "Clean", "");
    assert_status_line(&line, true, "Clean", "", "format line empty observed");
}

#[test]
fn build_file_result_json_serializable() {
    let mut suite = ExpectationSuite::new("test");
    let _result = suite
        .expect_value_to_be_between("word-count", 800, 650, 1000)
        .label("Word Count");

    let result = suite.into_suite_result();
    let file_result = build(Path::new("test.md"), &result, true);

    assert_result_summary(
        &file_result,
        "test.md",
        true,
        "success",
        1,
        1,
        0,
        "success result summary",
    );
    assert_no_failures(&file_result, "success results should have no failures");
    assert_check_count(&file_result, 1, "checks should be included on request");
    assert_check_present(
        &file_result,
        ExpectedCheck {
            id: "word-count",
            label: "Word Count",
            kind: "document-policy",
            success: true,
            observed: Some(serde_json::json!(800)),
        },
        "success results should preserve check metadata",
    );

    #[allow(clippy::disallowed_methods)]
    let json = serde_json::to_string(&file_result);
    let json_str = json.unwrap_or_else(|err| panic!("should serialize to JSON: {err}"));
    assert!(
        json_str.contains("word-count"),
        "json should include check id"
    );
    assert!(
        json_str.contains("Word Count"),
        "json should include check label"
    );
    assert!(
        json_str.contains("rewrite_needed"),
        "json should include rewrite_needed"
    );
}

#[test]
fn build_file_result_includes_rewrite_guidance_for_failures() {
    let mut suite = ExpectationSuite::new("test");
    let _word_count_result = suite
        .expect_value_to_be_between("word-count", 1200, 650, 1000)
        .label("Word Count");
    let _em_dash_result = suite
        .expect_value_to_be_between("em-dashes", 2, 0, 0)
        .label("No Closed Em-Dashes")
        .checking("closed em dash count");

    let result = suite.into_suite_result();
    let file_result = build(Path::new("draft.md"), &result, false);

    assert_result_summary(
        &file_result,
        "draft.md",
        false,
        "check-failures",
        2,
        0,
        2,
        "failure result summary",
    );
    assert_failure_count(&file_result, 2, "failure count");
    assert_checks_hidden(&file_result, "checks should stay hidden by default");
    assert_rewrite_brief_contains(&file_result, "word-count range", "word-count rewrite brief");
    assert_rewrite_brief_contains(
        &file_result,
        "Replace closed em dashes",
        "em-dashes rewrite brief",
    );
    assert_failure_present(
        &file_result,
        ExpectedFailure {
            id: "word-count",
            label: "Word Count",
            kind: "document-policy",
            severity: "error",
            message_contains: "outside the configured range",
            checking: None,
            expected: Some(serde_json::json!({
                "min": 650,
                "max": 1000
            })),
            observed: Some(serde_json::json!(1200)),
            evidence_len: None,
            rewrite_hint_contains: "word-count range",
        },
        "word-count failure contract",
    );
    assert_failure_present(
        &file_result,
        ExpectedFailure {
            id: "em-dashes",
            label: "No Closed Em-Dashes",
            kind: "heuristics",
            severity: "error",
            message_contains: "Found closed em dashes",
            checking: Some("closed em dash count"),
            expected: Some(serde_json::json!({
                "min": 0,
                "max": 0
            })),
            observed: Some(serde_json::json!(2)),
            evidence_len: None,
            rewrite_hint_contains: "Replace closed em dashes",
        },
        "em-dashes failure contract",
    );
}

#[test]
fn build_file_result_sanitizes_readability_output() {
    let mut suite = ExpectationSuite::new("test");
    let _result = suite
        .record_custom_values(
            "gunning-fog",
            false,
            serde_json::json!({
                "formula": "fog",
                "maximum_score_x100": 1400
            }),
            serde_json::json!({
                "score": 14.67,
                "score_x100": 1467,
                "total_words": 100
            }),
            &[serde_json::json!({
                "score": 14.67,
                "score_x100": 1467,
                "total_words": 100
            })],
        )
        .label("Gunning Fog Index");

    let result = suite.into_suite_result();
    let file_result = build(Path::new("draft.md"), &result, false);

    assert_failure_present(
        &file_result,
        ExpectedFailure {
            id: "gunning-fog",
            label: "Gunning Fog Index",
            kind: "readability",
            severity: "error",
            message_contains: "Readability complexity is above the allowed maximum",
            checking: None,
            expected: Some(serde_json::json!({
                "formula": "fog",
                "maximum_score": 14.0
            })),
            observed: Some(serde_json::json!({
                "score": 14.67,
                "total_words": 100
            })),
            evidence_len: None,
            rewrite_hint_contains: "lower complexity",
        },
        "readability fields should be sanitized",
    );
}

#[test]
fn build_file_result_strips_internal_index_fields_from_evidence() {
    let mut suite = ExpectationSuite::new("test");
    let _result = suite
        .record_custom_values(
            "negation-reframe",
            false,
            serde_json::json!({ "min": 0, "max": 0 }),
            serde_json::json!(1),
            &[serde_json::json!({
                "section_index": 3,
                "paragraph_index": 4,
                "sentence_index": 2,
                "sentence_index_next": 3,
                "pattern_type": "inline",
                "matched_text": "x, not y",
                "sentence": "A pediatrician is the right person for a referral, not a diagnosis."
            })],
        )
        .label("Negation-Reframe Pattern");

    let result = suite.into_suite_result();
    let file_result = build(Path::new("draft.md"), &result, false);

    assert_failure_present(
        &file_result,
        ExpectedFailure {
            id: "negation-reframe",
            label: "Negation-Reframe Pattern",
            kind: "heuristics",
            severity: "error",
            message_contains: "Found negation-reframe rhetoric",
            checking: None,
            expected: Some(serde_json::json!({ "min": 0, "max": 0 })),
            observed: Some(serde_json::json!(1)),
            evidence_len: Some(1),
            rewrite_hint_contains: "not-X-it-is-Y pattern",
        },
        "heuristic evidence should survive with public fields only",
    );

    let evidence = file_result
        .failures
        .first()
        .and_then(|failure| failure.evidence.as_ref())
        .and_then(|items| items.first())
        .unwrap_or_else(|| panic!("evidence should be preserved"));
    assert!(
        evidence.get("section_index").is_none(),
        "section_index removed"
    );
    assert!(
        evidence.get("paragraph_index").is_none(),
        "paragraph_index removed"
    );
    assert!(
        evidence.get("sentence_index").is_none(),
        "sentence_index removed"
    );
    assert!(
        evidence.get("sentence_index_next").is_none(),
        "sentence_index_next removed"
    );
    assert!(
        evidence.get("pattern_type").is_none(),
        "pattern_type removed"
    );
    assert_eq!(
        evidence
            .get("matched_text")
            .and_then(serde_json::Value::as_str),
        Some("x, not y")
    );
}

#[test]
fn build_file_result_includes_checks_when_requested() {
    let mut suite = ExpectationSuite::new("test");
    let _result = suite
        .expect_value_to_be_between("word-count", 800, 650, 1000)
        .label("Word Count");

    let result = suite.into_suite_result();
    let file_result = build(Path::new("draft.md"), &result, true);

    assert_check_count(
        &file_result,
        1,
        "include_checks should expose check outputs",
    );
}
