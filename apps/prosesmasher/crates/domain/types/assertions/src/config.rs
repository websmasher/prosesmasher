pub fn assert_english_default_quality(
    has_actually: bool,
    has_delve: bool,
    prohibited_substrings_defaults: bool,
    has_utilize_use: bool,
    has_the: bool,
    has_that: bool,
    context: &str,
) {
    assert!(
        has_actually,
        "{context}: expected built-in prohibited term `actually`"
    );
    assert!(
        has_delve,
        "{context}: expected built-in prohibited term `delve`"
    );
    assert!(
        !prohibited_substrings_defaults,
        "{context}: prohibited_substrings.defaults should be false"
    );
    assert!(
        has_utilize_use,
        "{context}: expected built-in simplicity pair utilize->use"
    );
    assert!(
        has_the,
        "{context}: expected built-in repetition exclusion `the`"
    );
    assert!(
        has_that,
        "{context}: expected built-in repetition exclusion `that`"
    );
}
