use crate::test_helpers::make_doc;
use prosesmasher_app_checks_lexical_assertions::simplicity as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale, SimplePair};

/// A (complex, simple) word pair for test configuration.
type WordPair<'a> = (&'a str, &'a str);

fn config_with_pairs(pairs: &[WordPair<'_>]) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.lexical.simplicity_pairs.add = pairs
        .iter()
        .map(|(complex, simple)| SimplePair {
            complex: (*complex).to_owned(),
            simple: (*simple).to_owned(),
        })
        .collect();
    config
}

#[test]
fn simple_text_passes() {
    let doc = make_doc("We use this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use"), ("implement", "do")]);
    assertions::assert_complex_terms(&doc, &config, &[], 1, 0, "simple words → pass");
}

#[test]
fn complex_word_fails() {
    let doc = make_doc("We utilize this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use")]);
    assertions::assert_complex_terms(
        &doc,
        &config,
        &["utilize"],
        0,
        1,
        "complex word found → fail",
    );
}

#[test]
fn empty_pairs_skips() {
    let doc = make_doc("We utilize this tool.", Locale::En);
    let config = config_with_pairs(&[]);
    assertions::assert_skips(&doc, &config, "empty pairs → no expectation");
}

#[test]
fn case_insensitive_match() {
    let doc = make_doc("We UTILIZE this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use")]);
    assertions::assert_complex_terms(&doc, &config, &["utilize"], 0, 1, "case-insensitive → fail");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn complex_word_in_blockquote_detected() {
    let doc = crate::test_helpers::make_doc_in_blockquote("We utilize this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use")]);
    assertions::assert_complex_terms(
        &doc,
        &config,
        &["utilize"],
        0,
        1,
        "complex word in blockquote → fail",
    );
}

#[test]
fn complex_word_in_code_block_not_detected() {
    let doc = crate::test_helpers::make_doc_code_only("We utilize this tool.", Locale::En);
    let config = config_with_pairs(&[("utilize", "use")]);
    assertions::assert_complex_terms(
        &doc,
        &config,
        &[],
        1,
        0,
        "complex word in code block → pass",
    );
}
