use prosesmasher_app_checks_lexical_assertions::hedge_words as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn config_with_hedges(_words: &[&str], max: Option<usize>) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    if let Some(max) = max {
        config.quality.heuristics.hedge_stacking.max_per_sentence = max;
    }
    config
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
    assertions::assert_hedge_failure(
        &doc,
        &config,
        2,
        "2 hedges in 1 sentence with threshold 2 → fail",
    );
}

#[test]
fn one_hedge_per_sentence_passes() {
    // Each sentence has only 1 hedge → both pass
    let doc = make_doc_sentences(&["it might work", "perhaps later"], Locale::En);
    let config = config_with_hedges(&["might", "perhaps"], None);
    assertions::assert_passes(&doc, &config, "1 hedge per sentence → aggregate pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc_sentences(&["it might perhaps work"], Locale::En);
    let config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    assertions::assert_fails(&doc, &config, "default hedge lexicon should run");
}

#[test]
fn custom_threshold_respected() {
    // 3 hedges but threshold is 4 → should pass
    let doc = make_doc_sentences(&["it might perhaps maybe work"], Locale::En);
    let config = config_with_hedges(&["might", "perhaps", "maybe"], Some(4));
    assertions::assert_passes(&doc, &config, "3 hedges with threshold 4 → pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
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
                    words: "it might perhaps work"
                        .split_whitespace()
                        .map(|w| Word {
                            text: w.to_owned(),
                            syllable_count: 1,
                        })
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
    assertions::assert_fails(&doc, &config, "hedges inside blockquote must be detected");
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
    assertions::assert_passes(
        &doc,
        &config,
        "code block content must be ignored by aggregate check",
    );
}
