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
}
