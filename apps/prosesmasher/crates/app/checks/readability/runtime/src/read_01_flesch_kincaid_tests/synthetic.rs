use prosesmasher_app_checks_readability_assertions::flesch_kincaid as assertions;
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
    assertions::assert_passes(&doc, &config, "easy text should pass");
}

#[test]
fn hard_text_fails() {
    // 100 words, 4 sentences, 200 syllables (2.0 syl/word)
    // score = 206.835 - 1.015×25 - 84.6×2.0 = 206.835 - 25.375 - 169.2 = 12.26
    let doc = make_readability_doc(100, 4, 200);
    let config = config_with_fk_min(60.0);
    assertions::assert_score_failure(
        &doc,
        &config,
        1226,
        "hard text should fail (score ~12 < min 60)",
    );
}

#[test]
fn empty_doc_skips() {
    let doc = make_readability_doc(0, 0, 0);
    let config = config_with_fk_min(60.0);
    assertions::assert_skips(&doc, &config, "zero words/sentences → no expectation");
}

#[test]
fn zero_sentences_skips() {
    let doc = make_readability_doc(50, 0, 50);
    let config = config_with_fk_min(60.0);
    assertions::assert_skips(&doc, &config, "zero sentences skips");
}

#[test]
fn no_threshold_skips() {
    let doc = make_readability_doc(100, 10, 120);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "no threshold uses default pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
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
    assertions::assert_skips(
        &doc,
        &config,
        "zero words with nonzero sentences → skip (no div by zero)",
    );
}
