use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_patterns() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            false_question_patterns: vec![
                "isn't that what we all".to_owned(),
                "isn't that the point".to_owned(),
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn false_question_detected() {
    let doc = make_doc("And isn't that what we all want?", Locale::En);
    let config = config_with_patterns();
    let mut suite = ExpectationSuite::new("test");
    super::FalseQuestionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "false question should fail"
    );
}

#[test]
fn genuine_question_passes() {
    let doc = make_doc("So who's going to build the alternative?", Locale::En);
    let config = config_with_patterns();
    let mut suite = ExpectationSuite::new("test");
    super::FalseQuestionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "genuine question should pass"
    );
}

#[test]
fn non_question_passes() {
    let doc = make_doc("That's what we all want.", Locale::En);
    let config = config_with_patterns();
    let mut suite = ExpectationSuite::new("test");
    super::FalseQuestionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "non-question should pass even with matching text"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc("And isn't that what we all want?", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FalseQuestionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty patterns should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::FalseQuestionCheck;
    assert_eq!(check.id(), "false-question");
    assert_eq!(check.label(), "False Question");
    assert!(check.supported_locales().is_none());
}
