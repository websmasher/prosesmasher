use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    Thresholds, Word,
};

fn make_sentence(text: &str) -> Sentence {
    let words: Vec<Word> = text
        .split_whitespace()
        .map(|w| Word {
            text: w.to_owned(),
            syllable_count: 1,
        })
        .collect();
    Sentence {
        text: text.to_owned(),
        words,
    }
}

fn doc_with_paragraph_sentences(sentence_count: usize) -> Document {
    let sentences: Vec<Sentence> = (0..sentence_count)
        .map(|i| make_sentence(&format!("This is sentence {i}.")))
        .collect();

    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata::default(),
    }
}

#[test]
fn paragraph_exceeds_max_fails() {
    let doc = doc_with_paragraph_sentences(6);
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            max_paragraph_sentences: Some(4),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
    let mut suite = ExpectationSuite::new("test");
    super::ParagraphLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "6 sentences with max=4 should fail"
    );
    let vr = result.results.get("paragraph-length-0");
    assert!(vr.is_some(), "paragraph-length result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("paragraph_text"))
            .and_then(serde_json::Value::as_str), Some("This is sentence 0. This is sentence 1. This is sentence 2. This is sentence 3. This is sentence 4. This is sentence 5."), "paragraph text");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence_count"))
            .and_then(serde_json::Value::as_i64), Some(6), "sentence count");
    }
}

#[test]
fn paragraph_within_max_passes() {
    let doc = doc_with_paragraph_sentences(3);
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            max_paragraph_sentences: Some(4),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
    let mut suite = ExpectationSuite::new("test");
    super::ParagraphLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "3 sentences with max=4 should pass"
    );
}

#[test]
fn paragraph_in_blockquote_checked() {
    let sentences: Vec<Sentence> = (0..6)
        .map(|i| make_sentence(&format!("Sentence {i}.")))
        .collect();

    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })])],
        }],
        metadata: DocumentMetadata::default(),
    };

    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            max_paragraph_sentences: Some(4),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
    let mut suite = ExpectationSuite::new("test");
    super::ParagraphLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "blockquote paragraph with 6 sentences should fail"
    );
}

#[test]
fn no_threshold_skips() {
    let doc = doc_with_paragraph_sentences(10);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ParagraphLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no threshold → no expectation"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::ParagraphLengthCheck;
    assert_eq!(check.id(), "paragraph-length", "id");
    assert_eq!(check.label(), "Paragraph Length", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
