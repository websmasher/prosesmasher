use prosesmasher_app_checks_heuristics_assertions::triple_repeat as assertions;
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
    assertions::assert_triple_repeat_failure(
        &doc,
        &config,
        "it's",
        "It's fast.",
        "It's reliable.",
        "It's revolutionary.",
        "triple repeat should fail",
    );
}

#[test]
fn different_openers_pass() {
    let doc = make_doc_with_sentences(
        &["It's fast.", "The engine purrs.", "Nothing breaks."],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "different openers should pass");
}

#[test]
fn fewer_than_three_sentences_passes() {
    let doc = make_doc_with_sentences(&["It's fast.", "It's reliable."], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "only two sentences should pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
