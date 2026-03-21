use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_race_terms(terms: &[&str]) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        terms: TermLists {
            race_terms: terms.iter().map(|t| (*t).to_owned()).collect(),
            ..TermLists::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn clean_text_passes() {
    let doc = make_doc("The team reviewed the proposal.", Locale::En);
    let config = config_with_race_terms(&["blacklist", "whitelist"]);
    let mut suite = ExpectationSuite::new("test");
    super::RaceTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "no race terms → pass");
}

#[test]
fn race_term_present_fails() {
    let doc = make_doc("Add it to the blacklist today.", Locale::En);
    let config = config_with_race_terms(&["blacklist"]);
    let mut suite = ExpectationSuite::new("test");
    super::RaceTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "race term found → fail");
}

#[test]
fn empty_list_skips() {
    let doc = make_doc("Add it to the blacklist.", Locale::En);
    let config = config_with_race_terms(&[]);
    let mut suite = ExpectationSuite::new("test");
    super::RaceTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty list → no expectation");
}

#[test]
fn check_id_and_label() {
    let check = super::RaceTermsCheck;
    assert_eq!(check.id(), "race-terms", "id");
    assert_eq!(check.label(), "Race Terms", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}

#[test]
fn race_term_in_blockquote_detected() {
    let doc = crate::test_helpers::make_doc_in_blockquote(
        "Add it to the blacklist today.",
        Locale::En,
    );
    let config = config_with_race_terms(&["blacklist"]);
    let mut suite = ExpectationSuite::new("test");
    super::RaceTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "race term in blockquote → fail");
}

#[test]
fn race_term_in_code_block_not_detected() {
    let doc = crate::test_helpers::make_doc_code_only(
        "Add it to the blacklist today.",
        Locale::En,
    );
    let config = config_with_race_terms(&["blacklist"]);
    let mut suite = ExpectationSuite::new("test");
    super::RaceTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "race term in code block → pass");
}
