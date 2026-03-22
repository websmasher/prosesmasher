use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_patterns() -> CheckConfig {
    CheckConfig::default()
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
    let vr = result.results.get("summative-closer");
    assert!(vr.is_some(), "summative-closer result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("and that's what makes"), "matched closer");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("And that's what makes this approach so powerful."), "sentence evidence");
    }
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
fn default_config_runs() {
    let doc = make_doc(
        "And that's what makes this approach so powerful.",
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SummativeCloserCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "default summative patterns should run"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::SummativeCloserCheck;
    assert_eq!(check.id(), "summative-closer");
    assert_eq!(check.label(), "Summative Closer");
    assert!(check.supported_locales().is_none());
}

#[test]
fn summative_closer_in_middle_section_detected() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "First section with normal text.",
            "And that's what makes this approach so powerful.",
            "Final section with normal text.",
        ],
        Locale::En,
    );
    let config = config_with_patterns();
    let mut suite = ExpectationSuite::new("test");
    super::SummativeCloserCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "summative closer in section 2 of 3 should fail"
    );
}
