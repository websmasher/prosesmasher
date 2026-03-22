use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_required(terms: &[&str]) -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            required_terms: terms.iter().map(|t| (*t).to_owned()).collect(),
            ..TermLists::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn all_required_terms_present_passes() {
    let doc = make_doc("Rust ownership and borrowing are key concepts.", Locale::En);
    let config = config_with_required(&["ownership", "borrowing"]);
    let mut suite = ExpectationSuite::new("test");
    super::RequiredTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 0, "all present → no failures");
    assert_eq!(result.statistics.successful_expectations, 2, "2 terms checked");
}

#[test]
fn missing_required_term_fails() {
    let doc = make_doc("Rust ownership is a key concept.", Locale::En);
    let config = config_with_required(&["ownership", "borrowing"]);
    let mut suite = ExpectationSuite::new("test");
    super::RequiredTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "borrowing missing → 1 failure");
    assert_eq!(result.statistics.successful_expectations, 1, "ownership found → 1 pass");
}

#[test]
fn case_insensitive_match() {
    let doc = make_doc("RUST Ownership is great.", Locale::En);
    let config = config_with_required(&["rust", "ownership"]);
    let mut suite = ExpectationSuite::new("test");
    super::RequiredTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 0, "case insensitive match");
}

#[test]
fn empty_required_list_skips() {
    let doc = make_doc("Some text.", Locale::En);
    let config = config_with_required(&[]);
    let mut suite = ExpectationSuite::new("test");
    super::RequiredTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty list → skip");
}

#[test]
fn check_id_and_label() {
    let check = super::RequiredTermsCheck;
    assert_eq!(check.id(), "required-terms");
    assert_eq!(check.label(), "Required Terms");
}
