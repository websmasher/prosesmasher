use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_max(max: usize) -> CheckConfig {
    let mut config = CheckConfig::default();
    config
        .quality
        .heuristics
        .exclamation_density
        .max_per_paragraph = max;
    config
}

#[test]
fn within_threshold_passes() {
    let doc = make_doc("Great! This is fine.", Locale::En);
    let config = config_with_max(2);
    let mut suite = ExpectationSuite::new("test");
    super::ExclamationDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "1 exclamation within max 2 should pass"
    );
}

#[test]
fn exceeds_threshold_fails() {
    let doc = make_doc("Wow! Amazing! Incredible!", Locale::En);
    let config = config_with_max(1);
    let mut suite = ExpectationSuite::new("test");
    super::ExclamationDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "3 exclamations with max 1 should fail"
    );
    let vr = result.results.get("exclamation-density");
    assert!(vr.is_some(), "paragraph result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("paragraph_text"))
                .and_then(serde_json::Value::as_str),
            Some("Wow! Amazing! Incredible!"),
            "paragraph text"
        );
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("exclamation_count"))
                .and_then(serde_json::Value::as_i64),
            Some(3),
            "exclamation count"
        );
    }
}

#[test]
fn default_config_runs() {
    let doc = make_doc("Wow! Amazing! Incredible!", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ExclamationDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "default threshold should run"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::ExclamationDensityCheck;
    assert_eq!(check.id(), "exclamation-density");
    assert_eq!(check.label(), "Exclamation Density");
    assert!(check.supported_locales().is_none());
}
