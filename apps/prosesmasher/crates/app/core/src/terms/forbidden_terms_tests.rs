use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_forbidden(terms: &[&str]) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        terms: TermLists {
            forbidden_terms: terms.iter().map(|t| (*t).to_owned()).collect(),
            ..TermLists::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn clean_text_passes() {
    let doc = make_doc("The report is ready.", Locale::En);
    let config = config_with_forbidden(&["synergy", "paradigm"]);
    let mut suite = ExpectationSuite::new("test");
    super::ForbiddenTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "no forbidden terms → pass");
}

#[test]
fn forbidden_term_present_fails() {
    let doc = make_doc("We need more synergy here.", Locale::En);
    let config = config_with_forbidden(&["synergy"]);
    let mut suite = ExpectationSuite::new("test");
    super::ForbiddenTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "forbidden term found → fail");
}

#[test]
fn empty_list_skips() {
    let doc = make_doc("We need more synergy here.", Locale::En);
    let config = config_with_forbidden(&[]);
    let mut suite = ExpectationSuite::new("test");
    super::ForbiddenTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty list → no expectation");
}

#[test]
fn check_id_and_label() {
    let check = super::ForbiddenTermsCheck;
    assert_eq!(check.id(), "forbidden-terms", "id");
    assert_eq!(check.label(), "Forbidden Terms", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}

#[test]
fn forbidden_term_in_blockquote_detected() {
    let doc = crate::test_helpers::make_doc_in_blockquote(
        "We need more synergy here.",
        Locale::En,
    );
    let config = config_with_forbidden(&["synergy"]);
    let mut suite = ExpectationSuite::new("test");
    super::ForbiddenTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "forbidden term in blockquote → fail");
}

#[test]
fn forbidden_term_in_code_block_not_detected() {
    let doc = crate::test_helpers::make_doc_code_only(
        "We need more synergy here.",
        Locale::En,
    );
    let config = config_with_forbidden(&["synergy"]);
    let mut suite = ExpectationSuite::new("test");
    super::ForbiddenTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "forbidden term in code block → pass");
}
