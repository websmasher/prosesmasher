use prosesmasher_app_checks_heuristics_assertions::sentence_case as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Heading, Locale, Paragraph, Section, Sentence,
};

fn doc_with_heading(text: &str) -> Document {
    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: Some(Heading {
                level: 2,
                text: text.to_owned(),
            }),
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence {
                    text: "Content.".to_owned(),
                    words: vec![],
                }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata::default(),
    }
}

fn config_enforcing_sentence_case() -> CheckConfig {
    CheckConfig::default()
}

fn config_disabling_sentence_case() -> CheckConfig {
    let mut config = CheckConfig::default();
    config.quality.heuristics.sentence_case.enabled = false;
    config
}

#[test]
fn title_case_heading_fails() {
    let doc = doc_with_heading("Why Saying Nothing Is Bad");
    let config = config_enforcing_sentence_case();
    assertions::assert_heading_failure(
        &doc,
        &config,
        "Why Saying Nothing Is Bad",
        4,
        "title case heading should fail",
    );
}

#[test]
fn sentence_case_heading_passes() {
    let doc = doc_with_heading("Why saying nothing is bad");
    let config = config_enforcing_sentence_case();
    assertions::assert_passes(&doc, &config, "sentence case heading should pass");
}

#[test]
fn acronyms_not_counted() {
    let doc = doc_with_heading("Working with the AWS API");
    let config = config_enforcing_sentence_case();
    assertions::assert_passes(&doc, &config, "all-caps acronyms should be skipped");
}

#[test]
fn two_capitalized_words_passes() {
    // Only 2 non-first capitalized words — below the 3 threshold
    let doc = doc_with_heading("How Content Marketing works today");
    let config = config_enforcing_sentence_case();
    assertions::assert_passes(&doc, &config, "2 capitalized words should pass");
}

#[test]
fn no_headings_passes() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_enforcing_sentence_case();
    assertions::assert_skips(&doc, &config, "no headings → no expectations");
}

#[test]
fn disabled_check_skips() {
    let doc = doc_with_heading("Why Saying Nothing Is Bad");
    let config = config_disabling_sentence_case();
    assertions::assert_skips(&doc, &config, "disabled heuristic should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
