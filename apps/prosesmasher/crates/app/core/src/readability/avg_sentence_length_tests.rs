use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    Thresholds, Word,
};

/// Build a doc with a specific word count and sentence count.
fn make_sentence_doc(total_words: usize, total_sentences: usize) -> Document {
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
            ..Default::default()
        },
    }
}

fn config_with_avg_max(max: usize) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            avg_sentence_length_max: Some(max),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn average_within_limit_passes() {
    // 100 words / 5 sentences = 20 avg, max 25 → pass
    let doc = make_sentence_doc(100, 5);
    let config = config_with_avg_max(25);
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "avg 20 <= max 25");
    assert_eq!(result.statistics.unsuccessful_expectations, 0);
}

#[test]
fn average_over_limit_fails() {
    // 100 words / 3 sentences = 33 avg, max 25 → fail
    let doc = make_sentence_doc(100, 3);
    let config = config_with_avg_max(25);
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "avg 33 > max 25 should fail"
    );
    let vr = result.results.get("avg-sentence-length");
    assert!(vr.is_some(), "avg-sentence-length result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("average_words_per_sentence"))
            .and_then(serde_json::Value::as_i64), Some(33), "average words per sentence");
    }
}

#[test]
fn zero_sentences_skips() {
    let doc = make_sentence_doc(50, 0);
    let config = config_with_avg_max(25);
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn no_threshold_skips() {
    let doc = make_sentence_doc(100, 5);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn exact_boundary_passes() {
    // 75 words / 3 sentences = 25 avg, max 25 → pass (at_most is inclusive)
    let doc = make_sentence_doc(75, 3);
    let config = config_with_avg_max(25);
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "exact boundary should pass");
}

#[test]
fn check_id_and_label() {
    let check = super::AvgSentenceLengthCheck;
    assert_eq!(check.id(), "avg-sentence-length");
    assert_eq!(check.label(), "Average Sentence Length");
    assert!(check.supported_locales().is_none());
}

#[test]
fn truncation_at_boundary_passes() {
    // 101 words / 4 sentences = 25 (integer division truncates 25.25 → 25)
    // With max=25, should pass because truncated value equals max.
    let doc = make_sentence_doc(101, 4);
    let config = config_with_avg_max(25);
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "101/4 = 25 (truncated from 25.25) should pass with max=25"
    );
    assert_eq!(result.statistics.unsuccessful_expectations, 0);
}

#[test]
fn truncation_above_boundary_fails() {
    // 104 words / 4 sentences = 26 (exact), max=25 → fail
    let doc = make_sentence_doc(104, 4);
    let config = config_with_avg_max(25);
    let mut suite = ExpectationSuite::new("test");
    super::AvgSentenceLengthCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "104/4 = 26 should fail with max=25"
    );
}
