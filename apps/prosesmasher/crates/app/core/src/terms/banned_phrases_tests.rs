use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_phrases(phrases: &[&str]) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        terms: TermLists {
            banned_phrases: phrases.iter().map(|p| (*p).to_owned()).collect(),
            ..TermLists::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn phrase_absent_passes() {
    let doc = make_doc("We need this feature.", Locale::En);
    let config = config_with_phrases(&["let's dive in", "at the end of the day"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedPhrasesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "no banned phrase → pass");
}

#[test]
fn phrase_present_fails() {
    let doc = make_doc("so let's dive in here", Locale::En);
    let config = config_with_phrases(&["let's dive in"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedPhrasesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "banned phrase found → fail");
}

#[test]
fn empty_phrases_skips() {
    let doc = make_doc("so let's dive in here", Locale::En);
    let config = config_with_phrases(&[]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedPhrasesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty phrases → no expectation");
}

#[test]
fn phrase_case_insensitive() {
    let doc = make_doc("So Let's Dive In here", Locale::En);
    let config = config_with_phrases(&["let's dive in"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedPhrasesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "case-insensitive phrase → fail");
}

#[test]
fn check_id_and_label() {
    let check = super::BannedPhrasesCheck;
    assert_eq!(check.id(), "banned-phrases", "id");
    assert_eq!(check.label(), "Banned Phrases", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
