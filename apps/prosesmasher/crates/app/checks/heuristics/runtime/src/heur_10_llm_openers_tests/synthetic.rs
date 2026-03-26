use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_openers() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn llm_opener_detected() {
    let doc = make_doc("The interesting part is that nobody noticed.", Locale::En);
    let config = config_with_openers();
    let mut suite = ExpectationSuite::new("test");
    super::LlmOpenersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "LLM opener should fail"
    );
    let vr = result.results.get("llm-openers");
    assert!(vr.is_some(), "llm-openers result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("matched_text"))
                .and_then(serde_json::Value::as_str),
            Some("the interesting part is"),
            "matched opener"
        );
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("sentence"))
                .and_then(serde_json::Value::as_str),
            Some("The interesting part is that nobody noticed."),
            "sentence evidence"
        );
    }
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
fn default_config_runs() {
    let doc = make_doc("The interesting part is that nobody noticed.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::LlmOpenersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "default LLM opener patterns should run"
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
