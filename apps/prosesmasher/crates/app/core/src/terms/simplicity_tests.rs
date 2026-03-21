use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, SimplePair, TermLists};

/// A (complex, simple) word pair for test configuration.
type WordPair<'a> = (&'a str, &'a str);

fn config_with_pairs(pairs: &[WordPair<'_>]) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        terms: TermLists {
            simplicity_pairs: pairs
                .iter()
                .map(|(complex, simple)| SimplePair {
                    complex: (*complex).to_owned(),
                    simple: (*simple).to_owned(),
                })
                .collect(),
            ..TermLists::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn simple_text_passes() {
    let doc = make_doc("We use this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use"), ("implement", "do")]);
    let mut suite = ExpectationSuite::new("test");
    super::SimplicityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "simple words → pass");
}

#[test]
fn complex_word_fails() {
    let doc = make_doc("We utilize this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use")]);
    let mut suite = ExpectationSuite::new("test");
    super::SimplicityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "complex word found → fail");
}

#[test]
fn empty_pairs_skips() {
    let doc = make_doc("We utilize this tool.", Locale::En);
    let config = config_with_pairs(&[]);
    let mut suite = ExpectationSuite::new("test");
    super::SimplicityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty pairs → no expectation");
}

#[test]
fn case_insensitive_match() {
    let doc = make_doc("We UTILIZE this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use")]);
    let mut suite = ExpectationSuite::new("test");
    super::SimplicityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "case-insensitive → fail");
}

#[test]
fn check_id_and_label() {
    let check = super::SimplicityCheck;
    assert_eq!(check.id(), "simplicity", "id");
    assert_eq!(check.label(), "Simplicity", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
