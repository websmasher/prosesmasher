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
