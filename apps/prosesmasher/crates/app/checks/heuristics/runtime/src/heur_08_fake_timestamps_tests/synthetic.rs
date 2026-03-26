use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn timestamp_detected() {
    let doc = make_doc("At 5:47 PM I realized everything changed.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FakeTimestampCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "fake timestamp should fail"
    );
    let vr = result.results.get("fake-timestamps");
    assert!(vr.is_some(), "fake-timestamps result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("matched_text"))
                .and_then(serde_json::Value::as_str),
            Some("5:47 PM"),
            "matched timestamp"
        );
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("sentence"))
                .and_then(serde_json::Value::as_str),
            Some("At 5:47 PM I realized everything changed."),
            "sentence evidence"
        );
    }
}

#[test]
fn no_timestamp_passes() {
    let doc = make_doc("Later that evening I realized.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FakeTimestampCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "no timestamp should pass"
    );
}

#[test]
fn am_timestamp_detected() {
    let doc = make_doc("It was 2:30 AM when the alarm rang.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FakeTimestampCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "AM timestamp should fail"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::FakeTimestampCheck;
    assert_eq!(check.id(), "fake-timestamps");
    assert_eq!(check.label(), "Fake Timestamps");
    assert_eq!(check.supported_locales(), Some([Locale::En].as_slice()));
}

#[test]
fn twenty_four_hour_format_not_flagged() {
    let doc = make_doc("The meeting starts at 14:30 in the main room.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FakeTimestampCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "24-hour time without AM/PM should pass"
    );
}
