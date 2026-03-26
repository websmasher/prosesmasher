use crate::test_helpers::make_doc;
use prosesmasher_app_checks_heuristics_assertions::em_dashes as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn no_em_dashes_passes() {
    let doc = make_doc("Hello, world. No special dashes here.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "no em-dashes → pass");
}

#[test]
fn spaced_em_dash_passes() {
    let doc = make_doc("Hello \u{2014} world.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "spaced em-dash should pass");
}

#[test]
fn closed_em_dash_fails() {
    let doc = make_doc("Hello\u{2014}world.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_match_evidence(&doc, &config, "Hello\u{2014}world.", 1, "1 em-dash → fail");
}

#[test]
fn multiple_closed_em_dashes_fail() {
    let doc = make_doc("First\u{2014}second\u{2014}third\u{2014}end.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_match_evidence(
        &doc,
        &config,
        "First\u{2014}second\u{2014}third\u{2014}end.",
        3,
        "multiple em-dashes → fail",
    );
}

#[test]
fn en_dash_does_not_trigger() {
    // U+2013 en-dash is different from U+2014 em-dash
    let doc = make_doc("Pages 10\u{2013}20 in the book.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "en-dash should not trigger em-dash check");
}

#[test]
fn regular_hyphen_does_not_trigger() {
    let doc = make_doc("A well-known fact.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "regular hyphen should not trigger");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn closed_em_dash_inside_blockquote_detected() {
    let doc = crate::test_helpers::make_doc_in_blockquote("Hello\u{2014}world.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_match_evidence(
        &doc,
        &config,
        "Hello\u{2014}world.",
        1,
        "em-dash inside blockquote must be detected",
    );
}

#[test]
fn em_dash_in_code_block_not_detected() {
    let doc = crate::test_helpers::make_doc_code_only("let dash = '\u{2014}';", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "em-dash inside code block must NOT be detected",
    );
}

#[test]
fn em_dash_across_multiple_sections() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &["No dashes here.", "Hello\u{2014}world.", "Clean text."],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_match_evidence(
        &doc,
        &config,
        "Hello\u{2014}world.",
        1,
        "em-dash in second section must be detected",
    );
}

#[test]
fn one_sided_spaced_em_dash_passes() {
    let doc = make_doc("Hello \u{2014}world.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "one-sided spaced em-dash should pass");
}
