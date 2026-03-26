use prosesmasher_app_checks_document_policy_runtime::HeadingHierarchyCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_failure_count;

crate::define_rule_assertions!(
    HeadingHierarchyCheck,
    "heading-hierarchy",
    "Heading Hierarchy"
);

pub fn assert_h1_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_heading: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_failure_count(&result, 1, message);
    assert_eq!(
        result.results.values().find_map(|validation| {
            validation
                .result
                .partial_unexpected_list
                .as_ref()
                .and_then(|items| items.first())
                .and_then(|item| item.get("heading_text"))
                .and_then(|value| value.as_str())
        }),
        Some(expected_heading),
        "{message}"
    );
}

pub fn assert_failure_total(
    doc: &Document,
    config: &CheckConfig,
    expected_failures: usize,
    message: &str,
) {
    let result = run(doc, config);
    assert_failure_count(&result, expected_failures, message);
}

pub fn assert_clean(doc: &Document, config: &CheckConfig, message: &str) {
    let result = run(doc, config);
    assert_failure_count(&result, 0, message);
}
