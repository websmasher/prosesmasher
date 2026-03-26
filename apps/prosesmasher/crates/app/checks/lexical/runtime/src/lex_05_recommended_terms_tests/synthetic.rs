use crate::test_helpers::make_doc;
use prosesmasher_app_checks_lexical_assertions::recommended_terms as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale, TermPool};

fn config_with_pool(terms: &[&str], min_count: usize, inflections: bool) -> CheckConfig {
    let mut config = CheckConfig::default();
    config.quality.lexical.recommended_terms = Some(TermPool {
        terms: terms.iter().map(|t| (*t).to_owned()).collect(),
        min_count,
        allow_inflections: inflections,
    });
    config
}

#[test]
fn enough_terms_present_passes() {
    // Pool: ownership, borrowing, lifetimes, traits, async. Min 3.
    // Doc contains: ownership, borrowing, traits → 3 matches ≥ 3 → pass
    let doc = make_doc("Rust ownership and borrowing with traits today", Locale::En);
    let config = config_with_pool(
        &["ownership", "borrowing", "lifetimes", "traits", "async"],
        3,
        false,
    );
    assertions::assert_match_count(&doc, &config, 3, 1, 0, "3/5 ≥ 3 → pass");
}

#[test]
fn not_enough_terms_fails() {
    // Doc contains: ownership → 1 match < 3 → fail
    let doc = make_doc("Rust ownership is important.", Locale::En);
    let config = config_with_pool(
        &["ownership", "borrowing", "lifetimes", "traits", "async"],
        3,
        false,
    );
    assertions::assert_match_count(&doc, &config, 1, 0, 1, "1/5 < 3 → fail");
}

#[test]
fn inflections_match_word_variants() {
    // "screen" should match "screens" with allow_inflections=true
    let doc = make_doc("Multiple screens and borrowing concepts.", Locale::En);
    let config = config_with_pool(&["screen", "borrow"], 2, true);
    assertions::assert_match_count(
        &doc,
        &config,
        2,
        1,
        0,
        "screens→screen and borrowing→borrow via stem → 2 matches ≥ 2 → pass",
    );
}

#[test]
fn inflections_disabled_no_stem_match() {
    // "screen" should NOT match "screens" with allow_inflections=false
    let doc = make_doc("Multiple screens here.", Locale::En);
    let config = config_with_pool(&["screen"], 1, false);
    assertions::assert_match_count(
        &doc,
        &config,
        0,
        0,
        1,
        "exact match: 'screen' ≠ 'screens' → 0 matches < 1 → fail",
    );
}

#[test]
fn no_pool_configured_skips() {
    let doc = make_doc("Some text.", Locale::En);
    let config = CheckConfig::default(); // no recommended_terms
    assertions::assert_skips(&doc, &config, "no pool → skip");
}

#[test]
fn empty_pool_skips() {
    let config = config_with_pool(&[], 0, false);
    let doc = make_doc("Some text.", Locale::En);
    assertions::assert_skips(&doc, &config, "empty pool → skip");
}

#[test]
fn min_count_zero_always_passes() {
    let doc = make_doc("Nothing relevant here.", Locale::En);
    let config = config_with_pool(&["ownership", "borrowing"], 0, false);
    assertions::assert_match_count(&doc, &config, 0, 1, 0, "min 0 → always pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
