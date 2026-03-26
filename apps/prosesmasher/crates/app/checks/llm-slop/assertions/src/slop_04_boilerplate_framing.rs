use prosesmasher_app_checks_llm_slop_runtime::BoilerplateFramingCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    BoilerplateFramingCheck,
    "boilerplate-framing",
    "Boilerplate Framing",
    Some(&[Locale::En])
);

pub fn assert_framing_failure(
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
        "boilerplate-framing",
        "pattern_kind",
        expected_pattern_kind,
    );
    assert_first_evidence_str(
        &result,
        "boilerplate-framing",
        "matched_text",
        expected_match,
    );
}
