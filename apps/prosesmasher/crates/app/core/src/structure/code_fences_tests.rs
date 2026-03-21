use crate::check::Check;
use crate::test_helpers::{make_doc, make_doc_code_only};
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn doc_with_code_block_fails() {
    let doc = make_doc_code_only("fn main() {}", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::CodeFencesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "document with code block should fail"
    );
}

#[test]
fn doc_without_code_block_passes() {
    let doc = make_doc("This is a normal paragraph without code.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::CodeFencesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "document without code block should pass"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::CodeFencesCheck;
    assert_eq!(check.id(), "code-fences", "id");
    assert_eq!(check.label(), "Code Fences", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
