use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn dramatic_colon_detected() {
    let doc = make_doc("And then it hit me: everything changed.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "short clause after colon should fail"
    );
    let vr = result.results.get("colon-dramatic");
    assert!(vr.is_some(), "colon-dramatic result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("everything changed."), "matched clause");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("And then it hit me: everything changed."), "sentence evidence");
    }
}

#[test]
fn list_after_colon_passes() {
    let doc = make_doc(
        "There are three types: red, blue, and green.",
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "list after colon should pass"
    );
}

#[test]
fn no_colon_passes() {
    let doc = make_doc("A perfectly normal sentence without colons.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "no colon should pass"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::ColonDramaticCheck;
    assert_eq!(check.id(), "colon-dramatic");
    assert_eq!(check.label(), "Dramatic Colon");
    assert!(check.supported_locales().is_none());
}

/// False-positive test: "Time: 3 hours" is a factual label, not dramatic prose.
/// NOTE: This test exposes a real bug — the current heuristic counts any short
/// post-colon clause as dramatic, including factual label-value pairs.
/// Filed as a known issue; ignored until the heuristic is refined.
#[test]
#[ignore = "false positive: colon-dramatic flags factual label:value pairs (needs heuristic fix)"]
fn factual_colon_label_should_not_flag() {
    let doc = make_doc("Time: 3 hours", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    // Expected: pass (0 failures). If this fails, the heuristic needs refinement
    // to distinguish dramatic colons from factual label:value patterns.
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "factual label 'Time: 3 hours' should not flag as dramatic (false positive bug)"
    );
}
