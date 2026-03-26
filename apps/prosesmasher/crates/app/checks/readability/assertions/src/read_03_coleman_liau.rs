use prosesmasher_app_checks_readability_runtime::ColemanLiauCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_i64;

crate::define_rule_assertions!(ColemanLiauCheck, "coleman-liau", "Coleman-Liau Index");

pub fn assert_score_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_score_x100: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_i64(&result, "coleman-liau", "score_x100", expected_score_x100);
}
