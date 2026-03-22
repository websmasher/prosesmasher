use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    Word,
};

fn doc_with_repeated_word(word: &str, count: usize) -> Document {
    let words: Vec<Word> = (0..count)
        .map(|_| Word {
            text: word.to_owned(),
            syllable_count: 2,
        })
        .collect();
    let text = vec![word; count].join(" ");

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
            total_words: count,
            ..DocumentMetadata::default()
        },
    }
}

fn config_with_repetition_max(max: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.heuristics.word_repetition.max = max;
    config
}

#[test]
fn word_exceeds_max_fails() {
    let doc = doc_with_repeated_word("actually", 7);
    let config = config_with_repetition_max(5);
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "actually x7 with max=5 should fail"
    );
    let vr = result.results.get("word-repetition-actually");
    assert!(vr.is_some(), "word repetition result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("word"))
            .and_then(serde_json::Value::as_str), Some("actually"), "repeated word");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("count"))
            .and_then(serde_json::Value::as_i64), Some(7), "repetition count");
    }
}

#[test]
fn word_within_max_passes() {
    let doc = doc_with_repeated_word("actually", 3);
    let config = config_with_repetition_max(5);
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "actually x3 with max=5 should pass"
    );
}

#[test]
fn short_words_ignored() {
    let doc = doc_with_repeated_word("the", 10);
    let config = config_with_repetition_max(5);
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "words < 4 chars should be ignored"
    );
}

#[test]
fn stop_words_ignored() {
    let doc = doc_with_repeated_word("that", 10);
    let mut config = config_with_repetition_max(5);
    config.quality.heuristics.word_repetition.excluded_terms.add = vec!["that".to_owned()];
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "stop words should be ignored"
    );
}

#[test]
fn no_threshold_skips() {
    let doc = doc_with_repeated_word("actually", 4);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "default quality config should run word repetition with built-in max"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::WordRepetitionCheck;
    assert_eq!(check.id(), "word-repetition", "id");
    assert_eq!(check.label(), "Word Repetition", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}

#[test]
fn multi_section_aggregation_exceeds_max() {
    // "actually" appears 3 times in section 1 and 3 times in section 2 (6 total).
    // With max=5, aggregated count should fail.
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "actually actually actually filler words here",
            "actually actually actually other words here",
        ],
        Locale::En,
    );
    let config = config_with_repetition_max(5);
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert!(
        result.statistics.unsuccessful_expectations >= 1,
        "actually x6 across sections with max=5 should fail"
    );
}
