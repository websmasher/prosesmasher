use prosesmasher_app_checks_style_signals_runtime::FakeTimestampCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(
    FakeTimestampCheck,
    "fake-timestamps",
    "Fake Timestamps",
    Some(&[prosesmasher_domain_types::Locale::En])
);

pub fn assert_timestamp_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "fake-timestamps", "matched_text", expected_match);
    assert_first_evidence_str(&result, "fake-timestamps", "sentence", expected_sentence);
}
