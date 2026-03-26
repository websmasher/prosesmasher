use prosesmasher_app_checks_llm_slop_runtime::SofteningLanguageCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    SofteningLanguageCheck,
    "softening-language",
    "Softening Language",
    Some(&[Locale::En])
);

pub fn assert_softening_failure(
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
        "softening-language",
        "pattern_kind",
        expected_pattern_kind,
    );
    assert_first_evidence_str(
        &result,
        "softening-language",
        "matched_text",
        expected_match,
    );
}
