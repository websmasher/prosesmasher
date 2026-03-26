use super::*;
use std::collections::BTreeSet;

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_all_returns_32() {
    let checks = collect_checks(None).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 37, "total check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_quality_returns_27() {
    let checks = collect_checks(Some("quality")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 32, "quality check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_lexical_returns_5() {
    let checks = collect_checks(Some("lexical")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 5, "lexical check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_heuristics_returns_16() {
    let checks =
        collect_checks(Some("heuristics")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 21, "heuristics check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_flow_returns_2() {
    let checks = collect_checks(Some("flow")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 2, "flow check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_readability_returns_4() {
    let checks =
        collect_checks(Some("readability")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 4, "readability check count");
}

#[test]
fn collect_unknown_group_errors() {
    let result = collect_checks(Some("unknown"));
    assert!(result.is_err(), "unknown group should error");
    if let Err(err) = result {
        assert!(
            err.contains("Unknown"),
            "error should contain 'Unknown' — got: {err}"
        );
    }
}

#[test]
fn collect_empty_string_errors() {
    let result = collect_checks(Some(""));
    assert!(result.is_err(), "empty group should error");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_all_ids_unique() {
    let checks = collect_checks(None).unwrap_or_else(|e| panic!("collect failed: {e}"));
    let mut seen = BTreeSet::new();
    for check in &checks {
        let id = check.id();
        assert!(seen.insert(id), "duplicate check ID: {id}");
    }
}

#[test]
#[allow(clippy::panic, clippy::arithmetic_side_effects)] // test assertion + safe addition
fn collect_all_equals_sum_of_groups() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("all: {e}"));
    let lexical = collect_checks(Some("lexical")).unwrap_or_else(|e| panic!("lexical: {e}"));
    let heuristics =
        collect_checks(Some("heuristics")).unwrap_or_else(|e| panic!("heuristics: {e}"));
    let flow = collect_checks(Some("flow")).unwrap_or_else(|e| panic!("flow: {e}"));
    let readability =
        collect_checks(Some("readability")).unwrap_or_else(|e| panic!("readability: {e}"));
    let document_policy =
        collect_checks(Some("document-policy")).unwrap_or_else(|e| panic!("document-policy: {e}"));

    let sum =
        lexical.len() + heuristics.len() + flow.len() + readability.len() + document_policy.len();

    assert_eq!(
        all.len(),
        sum,
        "all == lexical + heuristics + flow + readability + document-policy"
    );
}

#[test]
#[allow(clippy::panic)]
fn filter_single_check_by_id() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("{e}"));
    let filtered = filter_checks_by_id(all, "prohibited-terms").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 1, "one check");
    assert_eq!(
        filtered.first().map(|c| c.id()),
        Some("prohibited-terms"),
        "correct check"
    );
}

#[test]
#[allow(clippy::panic)]
fn filter_multiple_checks_by_id() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("{e}"));
    let filtered = filter_checks_by_id(all, "prohibited-terms,em-dashes,word-count")
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 3, "three checks");
    let ids: Vec<&str> = filtered.iter().map(|c| c.id()).collect();
    assert!(ids.contains(&"prohibited-terms"), "has prohibited-terms");
    assert!(ids.contains(&"em-dashes"), "has em-dashes");
    assert!(ids.contains(&"word-count"), "has word-count");
}

#[test]
fn filter_unknown_check_id_errors() {
    let all = collect_checks(None).unwrap_or_default();
    let result = filter_checks_by_id(all, "nonexistent-check");
    assert!(result.is_err(), "unknown check ID should error");
    if let Err(err) = result {
        assert!(
            err.contains("nonexistent-check"),
            "error should contain the bad ID — got: {err}"
        );
        assert!(
            err.contains("Available"),
            "error should list available — got: {err}"
        );
    }
}

#[test]
#[allow(clippy::panic)]
fn filter_empty_string_returns_all() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("{e}"));
    let count = all.len();
    let filtered = filter_checks_by_id(all, "").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), count, "empty filter returns all");
}

#[test]
#[allow(clippy::panic)]
fn filter_with_spaces_around_commas() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("{e}"));
    let filtered =
        filter_checks_by_id(all, "prohibited-terms , em-dashes").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 2, "spaces around commas handled");
}

#[test]
#[allow(clippy::panic)]
fn filter_combined_with_group() {
    let lexical = collect_checks(Some("lexical")).unwrap_or_else(|e| panic!("{e}"));
    let filtered =
        filter_checks_by_id(lexical, "prohibited-terms").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 1, "one check from group");
}

#[test]
fn filter_check_not_in_group_errors() {
    let lexical = collect_checks(Some("lexical")).unwrap_or_default();
    let result = filter_checks_by_id(lexical, "em-dashes");
    assert!(
        result.is_err(),
        "em-dashes not in lexical group should error"
    );
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_document_policy_returns_6() {
    let checks =
        collect_checks(Some("document-policy")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 5, "document-policy check count");
}

#[test]
#[allow(clippy::panic)]
fn list_checks_returns_catalog_metadata() {
    let entries = list_checks(Some("readability")).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(entries.len(), 4, "readability catalog count");
    let first = entries
        .first()
        .unwrap_or_else(|| panic!("missing first entry"));
    assert_eq!(first.group, "readability");
    assert_eq!(first.kind, "readability");
    assert!(
        first.default_enabled,
        "readability checks enabled by default"
    );
    assert!(
        !first.supported_locales.is_empty(),
        "supported locales present"
    );
}
