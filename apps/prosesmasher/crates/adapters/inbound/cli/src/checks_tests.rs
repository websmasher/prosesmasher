use super::*;
use std::collections::BTreeSet;

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_all_returns_32() {
    let checks = collect_checks(None).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 34, "total check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_terms_returns_7() {
    let checks = collect_checks(Some("terms")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 9, "terms check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_patterns_returns_13() {
    let checks = collect_checks(Some("patterns")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 13, "patterns check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_structure_returns_8() {
    let checks = collect_checks(Some("structure")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 8, "structure check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_readability_returns_4() {
    let checks = collect_checks(Some("readability")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 4, "readability check count");
}

#[test]
fn collect_unknown_group_errors() {
    let result = collect_checks(Some("unknown"));
    assert!(result.is_err(), "unknown group should error");
    if let Err(err) = result {
        assert!(err.contains("Unknown"), "error should contain 'Unknown' — got: {err}");
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
    let terms = collect_checks(Some("terms")).unwrap_or_else(|e| panic!("terms: {e}"));
    let patterns = collect_checks(Some("patterns")).unwrap_or_else(|e| panic!("patterns: {e}"));
    let structure = collect_checks(Some("structure")).unwrap_or_else(|e| panic!("structure: {e}"));
    let readability = collect_checks(Some("readability")).unwrap_or_else(|e| panic!("readability: {e}"));

    let sum = terms.len() + patterns.len() + structure.len() + readability.len();

    assert_eq!(all.len(), sum, "all == terms + patterns + structure + readability");
}

// ═══════════════════════════════════════════════════════════════
// filter_checks_by_id
// ═══════════════════════════════════════════════════════════════

#[test]
#[allow(clippy::panic)]
fn filter_single_check_by_id() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("{e}"));
    let filtered = filter_checks_by_id(all, "banned-words").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 1, "one check");
    assert_eq!(filtered.first().map(|c| c.id()), Some("banned-words"), "correct check");
}

#[test]
#[allow(clippy::panic)]
fn filter_multiple_checks_by_id() {
    let all = collect_checks(None).unwrap_or_else(|e| panic!("{e}"));
    let filtered = filter_checks_by_id(all, "banned-words,em-dashes,word-count")
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 3, "three checks");
    let ids: Vec<&str> = filtered.iter().map(|c| c.id()).collect();
    assert!(ids.contains(&"banned-words"), "has banned-words");
    assert!(ids.contains(&"em-dashes"), "has em-dashes");
    assert!(ids.contains(&"word-count"), "has word-count");
}

#[test]
fn filter_unknown_check_id_errors() {
    let all = collect_checks(None).unwrap_or_default();
    let result = filter_checks_by_id(all, "nonexistent-check");
    assert!(result.is_err(), "unknown check ID should error");
    if let Err(err) = result {
        assert!(err.contains("nonexistent-check"), "error should contain the bad ID — got: {err}");
        assert!(err.contains("Available"), "error should list available — got: {err}");
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
    let filtered = filter_checks_by_id(all, "banned-words , em-dashes")
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 2, "spaces around commas handled");
}

#[test]
#[allow(clippy::panic)]
fn filter_combined_with_group() {
    // --group terms --check banned-words → only banned-words from terms group
    let terms = collect_checks(Some("terms")).unwrap_or_else(|e| panic!("{e}"));
    let filtered = filter_checks_by_id(terms, "banned-words").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(filtered.len(), 1, "one check from group");
}

#[test]
fn filter_check_not_in_group_errors() {
    // --group terms --check em-dashes → em-dashes is a pattern check, not in terms
    let terms = collect_checks(Some("terms")).unwrap_or_default();
    let result = filter_checks_by_id(terms, "em-dashes");
    assert!(result.is_err(), "em-dashes not in terms group should error");
}
