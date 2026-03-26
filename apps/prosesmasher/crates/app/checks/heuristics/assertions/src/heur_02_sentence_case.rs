use prosesmasher_app_checks_heuristics_runtime::SentenceCaseCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_u64;

crate::define_rule_assertions!(
    SentenceCaseCheck,
    "sentence-case",
    "Sentence Case",
    Some(&[
        prosesmasher_domain_types::Locale::En,
        prosesmasher_domain_types::Locale::Es,
        prosesmasher_domain_types::Locale::Pt,
        prosesmasher_domain_types::Locale::Fr,
        prosesmasher_domain_types::Locale::Id
    ])
);

pub fn assert_heading_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_heading: &str,
    expected_capitalized_non_first_words: u64,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    let (check_id, validation) = result
        .results
        .iter()
        .find(|(id, _)| id.starts_with("sentence-case-"))
        .unwrap_or_else(|| panic!("missing sentence-case validation result"));
    assert_eq!(
        validation
            .result
            .partial_unexpected_list
            .as_ref()
            .and_then(|items| items.first())
            .and_then(|item| item.get("heading_text"))
            .and_then(|value| value.as_str()),
        Some(expected_heading),
        "{message}"
    );
    assert_first_evidence_u64(
        &result,
        check_id,
        "capitalized_non_first_words",
        expected_capitalized_non_first_words,
    );
}
