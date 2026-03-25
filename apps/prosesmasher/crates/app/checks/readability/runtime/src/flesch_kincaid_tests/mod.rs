use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

/// Build a document with precise control over words, sentences, and syllables.
fn make_readability_doc(
    total_words: usize,
    total_sentences: usize,
    total_syllables: usize,
) -> Document {
    // Create dummy words — syllable counts are controlled via metadata.
    let words: Vec<Word> = (0..total_words)
        .map(|i| Word {
            text: format!("w{i}"),
            syllable_count: 1,
        })
        .collect();
    let text = words
        .iter()
        .map(|w| w.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence { text, words }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences,
            total_syllables,
            ..Default::default()
        },
    }
}

fn config_with_fk_min(min: f64) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.readability.flesch_kincaid_min = Some(min);
    config
}

#[test]
fn easy_text_passes() {
    // 100 words, 10 sentences, 120 syllables (1.2 syl/word)
    // score = 206.835 - 1.015×10 - 84.6×1.2 = 206.835 - 10.15 - 101.52 = 95.165
    let doc = make_readability_doc(100, 10, 120);
    let config = config_with_fk_min(60.0);
    let mut suite = ExpectationSuite::new("test");
    super::FleschKincaidCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "easy text should pass"
    );
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "no failures"
    );
}

#[test]
fn hard_text_fails() {
    // 100 words, 4 sentences, 200 syllables (2.0 syl/word)
    // score = 206.835 - 1.015×25 - 84.6×2.0 = 206.835 - 25.375 - 169.2 = 12.26
    let doc = make_readability_doc(100, 4, 200);
    let config = config_with_fk_min(60.0);
    let mut suite = ExpectationSuite::new("test");
    super::FleschKincaidCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "hard text should fail (score ~12 < min 60)"
    );
    let vr = result.results.get("flesch-kincaid");
    assert!(vr.is_some(), "flesch-kincaid result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("score_x100"))
                .and_then(serde_json::Value::as_i64),
            Some(1226),
            "score x100"
        );
    }
}

#[test]
fn empty_doc_skips() {
    let doc = make_readability_doc(0, 0, 0);
    let config = config_with_fk_min(60.0);
    let mut suite = ExpectationSuite::new("test");
    super::FleschKincaidCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "zero words/sentences → no expectation"
    );
}

#[test]
fn zero_sentences_skips() {
    let doc = make_readability_doc(50, 0, 50);
    let config = config_with_fk_min(60.0);
    let mut suite = ExpectationSuite::new("test");
    super::FleschKincaidCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn no_threshold_skips() {
    let doc = make_readability_doc(100, 10, 120);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FleschKincaidCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1);
}

#[test]
fn check_id_and_label() {
    let check = super::FleschKincaidCheck;
    assert_eq!(check.id(), "flesch-kincaid");
    assert_eq!(check.label(), "Flesch-Kincaid Reading Ease");
    assert!(check.supported_locales().is_none());
}

#[test]
fn zero_words_nonzero_sentences_skips() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![],
        metadata: DocumentMetadata {
            total_words: 0,
            total_sentences: 5,
            total_syllables: 0,
            ..Default::default()
        },
    };
    let mut config = CheckConfig::default();
    config.quality.readability.flesch_kincaid_min = Some(50.0);
    let mut suite = ExpectationSuite::new("test");
    super::FleschKincaidCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "zero words with nonzero sentences → skip (no div by zero)"
    );
}
