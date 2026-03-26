use prosesmasher_app_checks_llm_slop_runtime::ResponseWrapperCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    ResponseWrapperCheck,
    "response-wrapper",
    "Response Wrapper",
    Some(&[Locale::En])
);

pub fn assert_response_wrapper_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_pattern_kind: &str,
    expected_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(
        &result,
        "response-wrapper",
        "pattern_kind",
        expected_pattern_kind,
    );
    assert_first_evidence_str(&result, "response-wrapper", "sentence", expected_sentence);
}
