use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, TermLists};

fn config_with_openers() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            llm_openers: vec![
                "the interesting part is".to_owned(),
                "in the world of".to_owned(),
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn llm_opener_detected() {
    let doc = make_doc(
        "The interesting part is that nobody noticed.",
        Locale::En,
    );
    let config = config_with_openers();
    let mut suite = ExpectationSuite::new("test");
    super::LlmOpenersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "LLM opener should fail"
    );
}

#[test]
fn normal_opener_passes() {
    let doc = make_doc("Nobody noticed the change at first.", Locale::En);
    let config = config_with_openers();
    let mut suite = ExpectationSuite::new("test");
    super::LlmOpenersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "normal opener should pass"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc(
        "The interesting part is that nobody noticed.",
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::LlmOpenersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty openers should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::LlmOpenersCheck;
    assert_eq!(check.id(), "llm-openers");
    assert_eq!(check.label(), "LLM Openers");
    assert!(check.supported_locales().is_none());
}

#[test]
fn llm_opener_in_middle_section_detected() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "Nothing special here.",
            "The interesting part is that nobody noticed.",
            "Final section with normal text.",
        ],
        Locale::En,
    );
    let config = config_with_openers();
    let mut suite = ExpectationSuite::new("test");
    super::LlmOpenersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "LLM opener in section 2 of 3 should fail"
    );
}
