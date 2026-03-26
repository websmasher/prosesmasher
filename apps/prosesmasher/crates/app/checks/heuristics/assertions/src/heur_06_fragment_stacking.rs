use prosesmasher_app_checks_heuristics_runtime::FragmentStackingCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_evidence_len;

crate::define_rule_assertions!(
    FragmentStackingCheck,
    "fragment-stacking",
    "Fragment Stacking",
    None
);

pub fn assert_fragment_failure_with_first_sentence(
    doc: &Document,
    config: &CheckConfig,
    expected_first_sentence: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_eq!(
        result
            .results
            .get("fragment-stacking")
            .and_then(|validation| validation.result.partial_unexpected_list.as_ref())
            .and_then(|items| items.first())
            .and_then(|item| item.get("sentences"))
            .and_then(|value| value.as_array())
            .and_then(|items| items.first())
            .and_then(|value| value.as_str()),
        Some(expected_first_sentence),
        "{message}"
    );
}

pub fn assert_fragment_failure_count(
    doc: &Document,
    config: &CheckConfig,
    expected_runs: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_evidence_len(&result, "fragment-stacking", expected_runs);
}
