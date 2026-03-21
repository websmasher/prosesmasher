use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_phrases() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            humble_bragger_phrases: vec![
                "in my experience".to_owned(),
                "as someone who has".to_owned(),
                "having worked with".to_owned(),
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn humble_brag_detected() {
    let doc = make_doc(
        "In my experience working with startups, this is common.",
        Locale::En,
    );
    let config = config_with_phrases();
    let mut suite = ExpectationSuite::new("test");
    super::HumbleBraggerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "humble brag should fail"
    );
}

#[test]
fn normal_sentence_passes() {
    let doc = make_doc("Startups often struggle with funding.", Locale::En);
    let config = config_with_phrases();
    let mut suite = ExpectationSuite::new("test");
    super::HumbleBraggerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "normal sentence should pass"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc("In my experience this is common.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HumbleBraggerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty phrases should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::HumbleBraggerCheck;
    assert_eq!(check.id(), "humble-bragger");
    assert_eq!(check.label(), "Humble Bragger");
    assert!(check.supported_locales().is_none());
}
