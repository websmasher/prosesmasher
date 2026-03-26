use prosesmasher_app_checks_llm_slop_runtime::EmptyEmphasisCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    EmptyEmphasisCheck,
    "empty-emphasis",
    "Empty Emphasis",
    Some(&[Locale::En])
);

pub fn assert_emphasis_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "empty-emphasis", "matched_text", expected_match);
}
