use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_phrases() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            jargon_faker_phrases: vec![
                "debug our".to_owned(),
                "debug your".to_owned(),
                "optimizing for".to_owned(),
                "iterating on your".to_owned(),
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn jargon_detected() {
    let doc = make_doc(
        "We need to debug our morning routine.",
        Locale::En,
    );
    let config = config_with_phrases();
    let mut suite = ExpectationSuite::new("test");
    super::JargonFakerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "fake jargon should fail"
    );
}

#[test]
fn normal_sentence_passes() {
    let doc = make_doc("We need to fix our morning routine.", Locale::En);
    let config = config_with_phrases();
    let mut suite = ExpectationSuite::new("test");
    super::JargonFakerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "normal sentence should pass"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc("We need to debug our morning routine.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::JargonFakerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty phrases should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::JargonFakerCheck;
    assert_eq!(check.id(), "jargon-faker");
    assert_eq!(check.label(), "Jargon Faker");
    assert!(check.supported_locales().is_none());
}
