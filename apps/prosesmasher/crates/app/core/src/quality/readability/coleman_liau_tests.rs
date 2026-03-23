use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    Word,
};

/// Build a doc with words of known letter counts.
/// Each word is made of `letters_per_word` alphabetic characters.
fn make_coleman_doc(
    word_count: usize,
    letters_per_word: usize,
    total_sentences: usize,
) -> Document {
    let word_text: String = "a".repeat(letters_per_word);
    let words: Vec<Word> = (0..word_count)
        .map(|_| Word {
            text: word_text.clone(),
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
            total_words: word_count,
            total_sentences,
            total_syllables: word_count, // not used by Coleman-Liau
            ..Default::default()
        },
    }
}

fn config_with_cl_max(max: f64) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.readability.coleman_liau_max = Some(max);
    config
}

#[test]
fn short_words_low_grade_passes() {
    // 100 words, 4 letters each, 10 sentences
    // L = 400/100 × 100 = 400
    // S = 10/100 × 100 = 10
    // score = 0.0588×400 - 0.296×10 - 15.8 = 23.52 - 2.96 - 15.8 = 4.76
    let doc = make_coleman_doc(100, 4, 10);
    let config = config_with_cl_max(10.0);
    let mut suite = ExpectationSuite::new("test");
    super::ColemanLiauCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "grade ~4.76 should pass (max 10)");
    assert_eq!(result.statistics.unsuccessful_expectations, 0);
}

#[test]
fn long_words_high_grade_fails() {
    // 100 words, 8 letters each, 5 sentences
    // L = 800/100 × 100 = 800
    // S = 5/100 × 100 = 5
    // score = 0.0588×800 - 0.296×5 - 15.8 = 47.04 - 1.48 - 15.8 = 29.76
    let doc = make_coleman_doc(100, 8, 5);
    let config = config_with_cl_max(10.0);
    let mut suite = ExpectationSuite::new("test");
    super::ColemanLiauCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "grade ~29.76 should fail (max 10)"
    );
    let vr = result.results.get("coleman-liau");
    assert!(vr.is_some(), "coleman-liau result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("score_x100"))
            .and_then(serde_json::Value::as_i64), Some(2976), "score x100");
    }
}

#[test]
fn zero_words_skips() {
    let doc = make_coleman_doc(0, 4, 0);
    let config = config_with_cl_max(10.0);
    let mut suite = ExpectationSuite::new("test");
    super::ColemanLiauCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn no_threshold_skips() {
    let doc = make_coleman_doc(100, 4, 10);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColemanLiauCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1);
}

#[test]
fn check_id_and_label() {
    let check = super::ColemanLiauCheck;
    assert_eq!(check.id(), "coleman-liau");
    assert_eq!(check.label(), "Coleman-Liau Index");
    assert!(check.supported_locales().is_none());
}

#[test]
fn zero_sentences_with_words_skips() {
    // Words present but zero sentences → guard should skip (no expectation).
    let doc = make_coleman_doc(50, 4, 0);
    let config = config_with_cl_max(10.0);
    let mut suite = ExpectationSuite::new("test");
    super::ColemanLiauCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "zero sentences with words present → should skip"
    );
}
