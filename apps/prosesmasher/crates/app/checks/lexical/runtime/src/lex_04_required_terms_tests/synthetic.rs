use crate::test_helpers::make_doc;
use prosesmasher_app_checks_lexical_assertions::required_terms as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_required(terms: &[&str]) -> CheckConfig {
    let mut config = CheckConfig::default();
    config.quality.lexical.required_terms = terms.iter().map(|t| (*t).to_owned()).collect();
    config
}

#[test]
fn all_required_terms_present_passes() {
    let doc = make_doc("Rust ownership and borrowing are key concepts.", Locale::En);
    let config = config_with_required(&["ownership", "borrowing"]);
    assertions::assert_term_counts(&doc, &config, 2, 0, "all present → 2 terms checked");
}

#[test]
fn missing_required_term_fails() {
    let doc = make_doc("Rust ownership is a key concept.", Locale::En);
    let config = config_with_required(&["ownership", "borrowing"]);
    assertions::assert_term_counts(
        &doc,
        &config,
        1,
        1,
        "borrowing missing → 1 failure, ownership found → 1 pass",
    );
}

#[test]
fn case_insensitive_match() {
    let doc = make_doc("RUST Ownership is great.", Locale::En);
    let config = config_with_required(&["rust", "ownership"]);
    assertions::assert_term_counts(&doc, &config, 2, 0, "case insensitive match");
}

#[test]
fn empty_required_list_skips() {
    let doc = make_doc("Some text.", Locale::En);
    let config = config_with_required(&[]);
    assertions::assert_skips(&doc, &config, "empty list → skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
