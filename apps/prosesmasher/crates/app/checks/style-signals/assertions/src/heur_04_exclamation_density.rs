use prosesmasher_app_checks_style_signals_runtime::ExclamationDensityCheck;
use prosesmasher_app_checks_test_support::result_helpers::{
    assert_first_evidence_i64, assert_first_evidence_str,
};

crate::define_rule_assertions!(
    ExclamationDensityCheck,
    "exclamation-density",
    "Exclamation Density",
    None
);

pub fn assert_exclamation_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_paragraph: &str,
    expected_count: i64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(
        &result,
        "exclamation-density",
        "paragraph_text",
        expected_paragraph,
    );
    assert_first_evidence_i64(
        &result,
        "exclamation-density",
        "exclamation_count",
        expected_count,
    );
}
