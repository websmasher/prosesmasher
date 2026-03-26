use prosesmasher_app_checks_flow_assertions::paragraph_length as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
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
    let config = config_with_max_sentences(4);
    assertions::assert_paragraph_length_failure(
        &doc,
        &config,
        "This is sentence 0. This is sentence 1. This is sentence 2. This is sentence 3. This is sentence 4. This is sentence 5.",
        6,
        "6 sentences with max=4 should fail",
    );
}

#[test]
fn paragraph_within_max_passes() {
    let doc = doc_with_paragraph_sentences(3);
    let config = config_with_max_sentences(4);
    assertions::assert_passes(&doc, &config, "3 sentences with max=4 should pass");
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

    let config = config_with_max_sentences(4);
    assertions::assert_fails(
        &doc,
        &config,
        "blockquote paragraph with 6 sentences should fail",
    );
}

#[test]
fn no_threshold_skips() {
    let doc = doc_with_paragraph_sentences(3);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "default quality config should run paragraph-length with built-in max",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

fn config_with_max_sentences(max_sentences: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.flow.paragraph_length.max_sentences = max_sentences;
    config
}
