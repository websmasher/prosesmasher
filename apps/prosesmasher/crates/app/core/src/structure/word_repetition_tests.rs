use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    TermLists, Thresholds, Word,
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
    CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            word_repetition_max: Some(max),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    }
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
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            word_repetition_max: Some(5),
            ..Thresholds::default()
        },
        terms: TermLists {
            stop_words: vec!["that".to_owned()],
            ..TermLists::default()
        },
    };
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
    let doc = doc_with_repeated_word("actually", 100);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::WordRepetitionCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no threshold → no expectations"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::WordRepetitionCheck;
    assert_eq!(check.id(), "word-repetition", "id");
    assert_eq!(check.label(), "Word Repetition", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
