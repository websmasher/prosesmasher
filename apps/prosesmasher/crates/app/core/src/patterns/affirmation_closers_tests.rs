use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_closers() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            affirmation_closers: vec![
                "and that's the key.".to_owned(),
                "that's what matters.".to_owned(),
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn affirmation_closer_detected() {
    let doc = make_doc(
        "We worked hard and that's the key.",
        Locale::En,
    );
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "affirmation closer should fail"
    );
}

#[test]
fn normal_closer_passes() {
    let doc = make_doc("The data supports this conclusion.", Locale::En);
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "normal closer should pass"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc("We worked hard and that's the key.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty closers should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::AffirmationClosersCheck;
    assert_eq!(check.id(), "affirmation-closers");
    assert_eq!(check.label(), "Affirmation Closers");
    assert!(check.supported_locales().is_none());
}
