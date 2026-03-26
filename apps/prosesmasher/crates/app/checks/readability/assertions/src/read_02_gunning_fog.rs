use prosesmasher_app_checks_readability_runtime::GunningFogCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_i64;

crate::define_rule_assertions!(GunningFogCheck, "gunning-fog", "Gunning Fog Index");

pub fn assert_complex_word_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_complex_words: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_i64(
        &result,
        "gunning-fog",
        "complex_word_count",
        expected_complex_words,
    );
}
