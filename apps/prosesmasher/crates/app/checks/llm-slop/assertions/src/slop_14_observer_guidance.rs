use prosesmasher_app_checks_llm_slop_runtime::ObserverGuidanceCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    ObserverGuidanceCheck,
    "observer-guidance",
    "Observer Guidance",
    Some(&[Locale::En])
);

pub fn assert_observer_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_pattern_kind: &str,
    expected_match: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(
        &result,
        "observer-guidance",
        "pattern_kind",
        expected_pattern_kind,
    );
    assert_first_evidence_str(&result, "observer-guidance", "matched_text", expected_match);
}
