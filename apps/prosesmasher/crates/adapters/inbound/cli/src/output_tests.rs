use super::*;

#[test]
fn format_pass_contains_pass() {
    let line = format_line(true, "Some Check", "42");
    assert!(line.contains("PASS"), "should contain PASS — got: {line}");
}

#[test]
fn format_fail_contains_fail() {
    let line = format_line(false, "Some Check", "99");
    assert!(line.contains("FAIL"), "should contain FAIL — got: {line}");
}

#[test]
fn format_line_contains_label() {
    let line = format_line(true, "Word Count", "800");
    assert!(line.contains("Word Count"), "should contain label — got: {line}");
}

#[test]
fn format_line_contains_observed() {
    let line = format_line(false, "Em Dashes", "3");
    assert!(line.contains('3'), "should contain observed value — got: {line}");
}

#[test]
fn format_line_empty_observed() {
    let line = format_line(true, "Clean", "");
    assert!(line.contains("PASS"), "should still contain PASS — got: {line}");
    assert!(line.contains("Clean"), "should still contain label — got: {line}");
}

#[test]
fn build_file_result_json_serializable() {
    use low_expectations::ExpectationSuite;
    use std::path::Path;

    let mut suite = ExpectationSuite::new("test");
    let _r = suite
        .expect_value_to_be_between("word-count", 800, 650, 1000)
        .label("Word Count");
    let result = suite.into_suite_result();
    let file_result = build_file_result(Path::new("test.md"), &result);

    assert!(file_result.success, "should be success");
    assert_eq!(file_result.evaluated, 1, "1 check");
    assert_eq!(file_result.passed, 1, "1 passed");
    assert_eq!(file_result.failed, 0, "0 failed");
    assert_eq!(file_result.summary.evaluated, 1, "summary evaluated");
    assert_eq!(file_result.summary.passed, 1, "summary passed");
    assert_eq!(file_result.summary.failed, 0, "summary failed");
    assert!(!file_result.rewrite_needed, "no rewrite needed on success");
    assert!(file_result.rewrite_brief.is_empty(), "no rewrite brief on success");
    assert!(file_result.failures.is_empty(), "no failures on success");
    assert_eq!(file_result.file, "test.md", "file path");
    assert_eq!(file_result.checks.len(), 1, "1 check output");

    if let Some(check) = file_result.checks.first() {
        assert_eq!(check.id, "word-count", "check id");
        assert_eq!(check.label, "Word Count", "check label");
        assert!(check.success, "check success");
    }

    // Verify it serializes to valid JSON
    #[allow(clippy::disallowed_methods)]
    let json = serde_json::to_string(&file_result);
    assert!(json.is_ok(), "should serialize to JSON — got {json:?}");
    let json_str = json.unwrap_or_default();
    assert!(json_str.contains("word-count"), "JSON contains check id");
    assert!(json_str.contains("Word Count"), "JSON contains label");
    assert!(json_str.contains("rewrite_needed"), "JSON contains rewrite flag");
}

#[test]
fn build_file_result_includes_rewrite_guidance_for_failures() {
    use low_expectations::ExpectationSuite;
    use std::path::Path;

    let mut suite = ExpectationSuite::new("test");
    let _word_count_result = suite
        .expect_value_to_be_between("word-count", 1200, 650, 1000)
        .label("Word Count");
    let _em_dash_result = suite
        .expect_value_to_be_between("em-dashes", 2, 0, 0)
        .label("No Em-Dashes")
        .checking("em dash count");
    let result = suite.into_suite_result();
    let file_result = build_file_result(Path::new("draft.md"), &result);

    assert!(!file_result.success, "should be failure");
    assert!(file_result.rewrite_needed, "rewrite should be needed");
    assert_eq!(file_result.failures.len(), 2, "2 failed checks");
    assert_eq!(file_result.rewrite_brief.len(), 2, "2 rewrite instructions");
    assert!(file_result.rewrite_brief.iter().any(|s| s.contains("word-count range")),
        "word-count rewrite brief present");
    assert!(file_result.rewrite_brief.iter().any(|s| s.contains("Replace em dashes")),
        "em-dash rewrite brief present");

    let word_count = file_result.failures.iter().find(|f| f.id == "word-count");
    assert!(word_count.is_some(), "word-count failure present");
    if let Some(failure) = word_count {
        assert_eq!(failure.severity, "error", "word-count severity");
        assert!(failure.message.contains("outside the configured range"),
            "word-count message");
        assert!(failure.checking.is_none(), "no checking on bare test suite value");
        assert_eq!(failure.expected, Some(serde_json::json!({
            "min": 650,
            "max": 1000
        })), "word-count expected");
        assert!(failure.rewrite_hint.contains("word-count range"),
            "word-count hint");
        assert!(failure.evidence.is_none(), "no evidence for scalar range failure");
        assert_eq!(failure.observed, Some(serde_json::json!(1200)), "word-count observed");
    }

    let em_dashes = file_result.failures.iter().find(|f| f.id == "em-dashes");
    assert!(em_dashes.is_some(), "em-dashes failure present");
    if let Some(failure) = em_dashes {
        assert_eq!(failure.label, "No Em-Dashes", "failure label uses check label");
        assert_eq!(failure.severity, "error", "em-dashes severity");
        assert!(failure.message.contains("Found em dashes"), "em-dashes message");
        assert_eq!(failure.checking.as_deref(), Some("em dash count"), "em-dashes checking");
        assert_eq!(failure.expected, Some(serde_json::json!({
            "min": 0,
            "max": 0
        })), "em-dashes expected");
        assert!(failure.rewrite_hint.contains("Replace em dashes"),
            "em-dashes hint");
        assert!(failure.evidence.is_none(), "no evidence for scalar count failure");
        assert_eq!(failure.observed, Some(serde_json::json!(2)), "em-dashes observed");
    }
}

#[test]
fn build_file_result_sanitizes_readability_output() {
    use low_expectations::ExpectationSuite;
    use std::path::Path;

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
    let file_result = build_file_result(Path::new("draft.md"), &result);
    let failure = file_result.failures.first();
    assert!(failure.is_some(), "failure present");
    if let Some(failure) = failure {
        assert_eq!(failure.expected, Some(serde_json::json!({
            "formula": "fog",
            "maximum_score": 14.0
        })));
        assert_eq!(failure.observed, Some(serde_json::json!({
            "score": 14.67,
            "total_words": 100
        })));
        assert!(failure.evidence.is_none(), "duplicate scalar evidence removed");
    }
}

#[test]
fn build_file_result_strips_internal_index_fields_from_evidence() {
    use low_expectations::ExpectationSuite;
    use std::path::Path;

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
    let file_result = build_file_result(Path::new("draft.md"), &result);
    let failure = file_result.failures.first();
    assert!(failure.is_some(), "failure present");
    if let Some(failure) = failure {
        let evidence = failure.evidence.as_ref();
        assert!(evidence.is_some(), "evidence preserved");
        if let Some(evidence) = evidence.and_then(|items| items.first()) {
            assert!(evidence.get("section_index").is_none(), "section index removed");
            assert!(evidence.get("paragraph_index").is_none(), "paragraph index removed");
            assert!(evidence.get("sentence_index").is_none(), "sentence index removed");
            assert!(evidence.get("sentence_index_next").is_none(), "next index removed");
            assert!(evidence.get("pattern_type").is_none(), "pattern type removed");
            assert_eq!(
                evidence.get("matched_text").and_then(serde_json::Value::as_str),
                Some("x, not y")
            );
        }
    }
}
