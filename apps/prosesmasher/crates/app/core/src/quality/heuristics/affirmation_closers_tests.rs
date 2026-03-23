use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn config_with_closers() -> CheckConfig {
    CheckConfig::default()
}

fn make_doc_with_sentences(texts: &[&str], locale: Locale) -> Document {
    let sentences: Vec<Sentence> = texts
        .iter()
        .map(|text| Sentence {
            text: (*text).to_owned(),
            words: text
                .split_whitespace()
                .map(|word| Word {
                    text: word.to_owned(),
                    syllable_count: 1,
                })
                .collect(),
        })
        .collect();
    let total_words = sentences.iter().map(|sentence| sentence.words.len()).sum();

    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences: texts.len(),
            ..Default::default()
        },
    }
}

#[test]
fn affirmation_closer_detected() {
    let doc = make_doc(
        "We worked hard and that's the key.",
        Locale::En,
    );
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "affirmation closer should fail"
    );
    let vr = result.results.get("affirmation-closers");
    assert!(vr.is_some(), "affirmation-closers result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("and that's the key."), "matched closer");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("We worked hard and that's the key."), "sentence evidence");
    }
}

#[test]
fn normal_closer_passes() {
    let doc = make_doc("The data supports this conclusion.", Locale::En);
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "normal closer should pass"
    );
}

#[test]
fn default_config_runs() {
    let doc = make_doc("We worked hard and that's the key.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "default affirmation closer patterns should run"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::AffirmationClosersCheck;
    assert_eq!(check.id(), "affirmation-closers");
    assert_eq!(check.label(), "Affirmation Closers");
    assert!(check.supported_locales().is_none());
}

#[test]
fn affirmation_closer_in_middle_section_detected() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "First section with normal text.",
            "We worked hard and that's the key.",
            "Final section with normal text.",
        ],
        Locale::En,
    );
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "affirmation closer in section 2 of 3 should fail"
    );
}

#[test]
fn thats_the_formula_detected_mid_section() {
    let doc = make_doc_with_sentences(&["Sixty seconds.", "That's the whole repair."], Locale::En);
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1);
    let vr = result.results.get("affirmation-closers");
    assert!(vr.is_some());
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("that's the ..."));
    }
}

#[test]
fn ordinary_thats_the_sentence_passes() {
    let doc = make_doc("That's the neighborhood school we use every day.", Locale::En);
    let config = config_with_closers();
    let mut suite = ExpectationSuite::new("test");
    super::AffirmationClosersCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1);
}
