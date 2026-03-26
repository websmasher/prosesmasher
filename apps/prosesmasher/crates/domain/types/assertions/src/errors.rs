use std::error::Error;
use std::fmt::Display;

pub fn assert_read_error_display(
    err: impl Display,
    expected_fragment: &str,
    expected_label: &str,
    context: &str,
) {
    let message = err.to_string();
    assert!(
        message.contains(expected_fragment),
        "{context}: expected `{expected_fragment}` in `{message}`"
    );
    assert!(
        message.contains(expected_label),
        "{context}: expected `{expected_label}` in `{message}`"
    );
}

pub fn assert_parse_error_display(
    err: impl Display,
    expected_fragment: &str,
    expected_label: &str,
    context: &str,
) {
    let message = err.to_string();
    assert!(
        message.contains(expected_fragment),
        "{context}: expected `{expected_fragment}` in `{message}`"
    );
    assert!(
        message.contains(expected_label),
        "{context}: expected `{expected_label}` in `{message}`"
    );
}

pub fn assert_config_error_display(
    err: impl Display,
    expected_fragment: &str,
    expected_label: &str,
    context: &str,
) {
    let message = err.to_string();
    assert!(
        message.contains(expected_fragment),
        "{context}: expected `{expected_fragment}` in `{message}`"
    );
    assert!(
        message.contains(expected_label),
        "{context}: expected `{expected_label}` in `{message}`"
    );
}

pub fn assert_error_source_is_none(err: &dyn Error, context: &str) {
    assert!(err.source().is_none(), "{context}: source() should be None");
}

pub fn assert_boxed_error_contains(err: impl Error + 'static, expected: &str, context: &str) {
    let boxed: Box<dyn Error> = Box::new(err);
    let message = boxed.to_string();
    assert!(
        message.contains(expected),
        "{context}: expected `{expected}` in `{message}`"
    );
}
