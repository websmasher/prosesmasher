use prosesmasher_app_checks_readability_assertions::gunning_fog as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

/// Build a doc where each word has a specified syllable count.
/// `syllable_counts` controls per-word syllable counts.
fn make_fog_doc(syllable_counts: &[usize], total_sentences: usize) -> Document {
    let words: Vec<Word> = syllable_counts
        .iter()
        .enumerate()
        .map(|(i, &sc)| Word {
            text: format!("w{i}"),
            syllable_count: sc,
        })
        .collect();
    let total_words = words.len();
    let total_syllables: usize = syllable_counts.iter().copied().sum();
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

fn config_with_fog_max(max: f64) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.readability.gunning_fog_max = Some(max);
    config
}

#[test]
fn simple_text_passes() {
    // 20 words, 2 sentences, 0 complex (all 1-syllable)
    // fog = 0.4 × (20/2 + 100×0/20) = 0.4 × 10 = 4.0
    let syllables = vec![1; 20];
    let doc = make_fog_doc(&syllables, 2);
    let config = config_with_fog_max(12.0);
    assertions::assert_passes(&doc, &config, "simple text should pass");
}

#[test]
fn complex_text_fails() {
    // 20 words, 2 sentences, 10 complex (3+ syllables)
    // fog = 0.4 × (20/2 + 100×10/20) = 0.4 × (10 + 50) = 0.4 × 60 = 24.0
    let mut syllables = vec![1; 10];
    syllables.extend(vec![3; 10]); // 10 complex words
    let doc = make_fog_doc(&syllables, 2);
    let config = config_with_fog_max(12.0);
    assertions::assert_complex_word_failure(
        &doc,
        &config,
        10,
        "complex text should fail (fog 24 > max 12)",
    );
}

#[test]
fn zero_words_skips() {
    let doc = make_fog_doc(&[], 0);
    let config = config_with_fog_max(12.0);
    assertions::assert_skips(&doc, &config, "zero words skips");
}

#[test]
fn zero_sentences_skips() {
    let syllables = vec![1; 10];
    let doc = make_fog_doc(&syllables, 0);
    let config = config_with_fog_max(12.0);
    assertions::assert_skips(&doc, &config, "zero sentences skips");
}

#[test]
fn no_threshold_skips() {
    let syllables = vec![1; 20];
    let doc = make_fog_doc(&syllables, 2);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "no threshold uses default pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
