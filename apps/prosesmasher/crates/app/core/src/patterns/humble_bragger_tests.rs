use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_phrases() -> CheckConfig {
    CheckConfig::default()
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
    let vr = result.results.get("humble-bragger");
    assert!(vr.is_some(), "humble-bragger result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("in my experience"), "matched phrase");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("In my experience working with startups, this is common."), "sentence evidence");
    }
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
fn default_config_runs() {
    let doc = make_doc("In my experience this is common.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HumbleBraggerCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "default humble-bragger patterns should run"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::HumbleBraggerCheck;
    assert_eq!(check.id(), "humble-bragger");
    assert_eq!(check.label(), "Humble Bragger");
    assert!(check.supported_locales().is_none());
}
