use prosesmasher_app_checks_llm_slop_runtime::BlameReframeCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    BlameReframeCheck,
    "blame-reframe",
    "Blame Reframe",
    Some(&[Locale::En])
);

pub fn assert_blame_reframe_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_match: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(&result, "blame-reframe", "matched_text", expected_match);
}
