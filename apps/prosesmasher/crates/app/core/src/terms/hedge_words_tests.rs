use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    TermLists, Thresholds, Word,
};

fn config_with_hedges(words: &[&str], max: Option<usize>) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        terms: TermLists {
            hedge_words: words.iter().map(|w| (*w).to_owned()).collect(),
            ..TermLists::default()
        },
        thresholds: Thresholds {
            max_hedges_per_sentence: max,
            ..Thresholds::default()
        },
    }
}

/// Build a document with multiple sentences in one paragraph.
fn make_doc_sentences(sentences: &[&str], locale: Locale) -> Document {
    let sents: Vec<Sentence> = sentences
        .iter()
        .map(|text| {
            let words: Vec<Word> = text
                .split_whitespace()
                .map(|w| Word {
                    text: w.to_owned(),
                    syllable_count: 1,
                })
                .collect();
            Sentence {
                text: (*text).to_owned(),
                words,
            }
        })
        .collect();

    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: sents,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_sentences: sentences.len(),
            ..Default::default()
        },
    }
}

#[test]
fn two_hedges_in_one_sentence_fails() {
    // "might" and "perhaps" are hedges, both in one sentence → fail
    let doc = make_doc_sentences(&["it might perhaps work"], Locale::En);
    let config = config_with_hedges(&["might", "perhaps", "maybe"], None); // default threshold 2
    let mut suite = ExpectationSuite::new("test");
    super::HedgeStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "2 hedges in 1 sentence with threshold 2 → fail"
    );
}

#[test]
fn one_hedge_per_sentence_passes() {
    // Each sentence has only 1 hedge → both pass
    let doc = make_doc_sentences(&["it might work", "perhaps later"], Locale::En);
    let config = config_with_hedges(&["might", "perhaps"], None);
    let mut suite = ExpectationSuite::new("test");
    super::HedgeStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 2,
        "1 hedge per sentence → both pass"
    );
    assert_eq!(result.statistics.unsuccessful_expectations, 0, "no failures");
}

#[test]
fn empty_hedge_list_skips() {
    let doc = make_doc_sentences(&["it might perhaps work"], Locale::En);
    let config = config_with_hedges(&[], None);
    let mut suite = ExpectationSuite::new("test");
    super::HedgeStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty hedge list → no expectations"
    );
}

#[test]
fn custom_threshold_respected() {
    // 3 hedges but threshold is 4 → should pass
    let doc = make_doc_sentences(&["it might perhaps maybe work"], Locale::En);
    let config = config_with_hedges(&["might", "perhaps", "maybe"], Some(4));
    let mut suite = ExpectationSuite::new("test");
    super::HedgeStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "3 hedges with threshold 4 → pass"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::HedgeStackingCheck;
    assert_eq!(check.id(), "hedge-stacking", "id");
    assert_eq!(check.label(), "Hedge Stacking", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}

#[test]
fn hedges_inside_blockquote_detected() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence {
                    text: "it might perhaps work".to_owned(),
                    words: "it might perhaps work".split_whitespace()
                        .map(|w| Word { text: w.to_owned(), syllable_count: 1 })
                        .collect(),
                }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })])],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_with_hedges(&["might", "perhaps"], None);
    let mut suite = ExpectationSuite::new("test");
    super::HedgeStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1,
        "hedges inside blockquote must be detected");
}

#[test]
fn hedges_in_code_block_not_detected() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::CodeBlock("it might perhaps work".to_owned())],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_with_hedges(&["might", "perhaps"], None);
    let mut suite = ExpectationSuite::new("test");
    super::HedgeStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0,
        "code block content must be ignored");
}
