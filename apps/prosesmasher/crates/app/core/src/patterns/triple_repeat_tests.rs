use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn make_sentences(texts: &[&str]) -> Vec<Sentence> {
    texts
        .iter()
        .map(|t| {
            let words: Vec<Word> = t
                .split_whitespace()
                .map(|w| Word {
                    text: w.to_owned(),
                    syllable_count: 1,
                })
                .collect();
            Sentence {
                text: (*t).to_owned(),
                words,
            }
        })
        .collect()
}

fn make_doc_with_sentences(texts: &[&str], locale: Locale) -> Document {
    let sentences = make_sentences(texts);
    let word_count: usize = sentences.iter().map(|s| s.words.len()).sum();
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
            total_words: word_count,
            total_sentences: texts.len(),
            ..Default::default()
        },
    }
}

#[test]
fn triple_same_opener_fails() {
    let doc = make_doc_with_sentences(
        &["It's fast.", "It's reliable.", "It's revolutionary."],
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::TripleRepeatCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "triple repeat should fail"
    );
    let vr = result.results.get("triple-repeat");
    assert!(vr.is_some(), "triple-repeat result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("it's"), "matched opener");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("It's fast."), "first sentence");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("next_sentence"))
            .and_then(serde_json::Value::as_str), Some("It's reliable."), "second sentence");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("third_sentence"))
            .and_then(serde_json::Value::as_str), Some("It's revolutionary."), "third sentence");
    }
}

#[test]
fn different_openers_pass() {
    let doc = make_doc_with_sentences(
        &["It's fast.", "The engine purrs.", "Nothing breaks."],
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::TripleRepeatCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "different openers should pass"
    );
}

#[test]
fn fewer_than_three_sentences_passes() {
    let doc = make_doc_with_sentences(&["It's fast.", "It's reliable."], Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::TripleRepeatCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "only two sentences should pass"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::TripleRepeatCheck;
    assert_eq!(check.id(), "triple-repeat");
    assert_eq!(check.label(), "Triple Repeat Opener");
    assert!(check.supported_locales().is_none());
}
