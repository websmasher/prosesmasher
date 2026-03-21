use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_patterns() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            summative_patterns: vec![
                "and that's what makes".to_owned(),
                "that's why this".to_owned(),
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn summative_closer_detected() {
    let doc = make_doc(
        "And that's what makes this approach so powerful.",
        Locale::En,
    );
    let config = config_with_patterns();
    let mut suite = ExpectationSuite::new("test");
    super::SummativeCloserCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "summative closer should fail"
    );
}

#[test]
fn normal_closer_passes() {
    let doc = make_doc("The data backs this up.", Locale::En);
    let config = config_with_patterns();
    let mut suite = ExpectationSuite::new("test");
    super::SummativeCloserCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "normal closer should pass"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc(
        "And that's what makes this approach so powerful.",
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SummativeCloserCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty patterns should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::SummativeCloserCheck;
    assert_eq!(check.id(), "summative-closer");
    assert_eq!(check.label(), "Summative Closer");
    assert!(check.supported_locales().is_none());
}
