use super::*;
use std::collections::BTreeSet;

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_all_returns_32() {
    let checks = collect_checks(None).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 32, "total check count");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn collect_terms_returns_7() {
    let checks = collect_checks(Some("terms")).unwrap_or_else(|e| panic!("collect failed: {e}"));
    assert_eq!(checks.len(), 7, "terms check count");
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
