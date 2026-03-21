use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_banned(words: &[&str]) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        terms: TermLists {
            banned_words: words.iter().map(|w| (*w).to_owned()).collect(),
            ..TermLists::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn no_banned_words_passes() {
    let doc = make_doc("We need this feature.", Locale::En);
    let config = config_with_banned(&["actually", "leverage"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedWordsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "no banned words → pass");
}

#[test]
fn banned_word_present_fails() {
    let doc = make_doc("We actually need this.", Locale::En);
    let config = config_with_banned(&["actually", "leverage"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedWordsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "banned word found → fail");
}

#[test]
fn banned_word_case_insensitive() {
    let doc = make_doc("We ACTUALLY need this.", Locale::En);
    let config = config_with_banned(&["actually"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedWordsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "case-insensitive match → fail");
}

#[test]
fn empty_banned_list_skips_check() {
    let doc = make_doc("We actually need this.", Locale::En);
    let config = config_with_banned(&[]); // no banned words
    let mut suite = ExpectationSuite::new("test");
    super::BannedWordsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty banned list → no expectation");
}

#[test]
fn multiple_banned_words_in_same_text() {
    let doc = make_doc("We actually leverage this.", Locale::En);
    let config = config_with_banned(&["actually", "leverage"]);
    let mut suite = ExpectationSuite::new("test");
    super::BannedWordsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    // expect_terms_absent creates one expectation, fails if ANY term matches
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "multiple banned → fail");
}

#[test]
fn check_id_and_label() {
    let check = super::BannedWordsCheck;
    assert_eq!(check.id(), "banned-words", "id");
    assert_eq!(check.label(), "Banned Words", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
