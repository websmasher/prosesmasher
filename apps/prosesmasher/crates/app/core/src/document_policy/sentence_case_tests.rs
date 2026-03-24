use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Heading, Locale,
    Paragraph, Section, Sentence,
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
    let mut suite = ExpectationSuite::new("test");
    super::SentenceCaseCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "title case heading should fail"
    );
    let vr = result.results.get("sentence-case-Why Saying Nothing Is Bad");
    assert!(vr.is_some(), "sentence-case result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("heading_text"))
            .and_then(serde_json::Value::as_str), Some("Why Saying Nothing Is Bad"), "heading text");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("capitalized_non_first_words"))
            .and_then(serde_json::Value::as_u64), Some(4), "capitalized words after first");
    }
}

#[test]
fn sentence_case_heading_passes() {
    let doc = doc_with_heading("Why saying nothing is bad");
    let config = config_enforcing_sentence_case();
    let mut suite = ExpectationSuite::new("test");
    super::SentenceCaseCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "sentence case heading should pass"
    );
}

#[test]
fn acronyms_not_counted() {
    let doc = doc_with_heading("Working with the AWS API");
    let config = config_enforcing_sentence_case();
    let mut suite = ExpectationSuite::new("test");
    super::SentenceCaseCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "all-caps acronyms should be skipped"
    );
}

#[test]
fn two_capitalized_words_passes() {
    // Only 2 non-first capitalized words — below the 3 threshold
    let doc = doc_with_heading("How Content Marketing works today");
    let config = config_enforcing_sentence_case();
    let mut suite = ExpectationSuite::new("test");
    super::SentenceCaseCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "2 capitalized words should pass"
    );
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
    let mut suite = ExpectationSuite::new("test");
    super::SentenceCaseCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no headings → no expectations"
    );
}

#[test]
fn disabled_check_skips() {
    let doc = doc_with_heading("Why Saying Nothing Is Bad");
    let config = config_disabling_sentence_case();
    let mut suite = ExpectationSuite::new("test");
    super::SentenceCaseCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "disabled heuristic should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::SentenceCaseCheck;
    assert_eq!(check.id(), "sentence-case", "id");
    assert_eq!(check.label(), "Sentence Case", "label");
    assert_eq!(
        check.supported_locales(),
        Some(
            [Locale::En, Locale::Es, Locale::Pt, Locale::Fr, Locale::Id].as_slice()
        ),
        "supported locales"
    );
}
