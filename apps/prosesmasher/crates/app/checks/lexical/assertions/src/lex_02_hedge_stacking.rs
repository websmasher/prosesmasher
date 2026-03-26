use prosesmasher_app_checks_lexical_runtime::HedgeStackingCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_i64;

crate::define_rule_assertions!(HedgeStackingCheck, "hedge-stacking", "Hedge Stacking");

pub fn assert_hedge_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_count: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_i64(&result, "hedge-stacking", "hedge_count", expected_count);
}
