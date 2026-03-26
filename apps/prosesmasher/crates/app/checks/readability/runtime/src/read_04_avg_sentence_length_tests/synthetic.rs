use prosesmasher_app_checks_readability_assertions::avg_sentence_length as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
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
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.readability.avg_sentence_length_max = Some(max);
    config
}

#[test]
fn average_within_limit_passes() {
    // 100 words / 5 sentences = 20 avg, max 25 → pass
    let doc = make_sentence_doc(100, 5);
    let config = config_with_avg_max(25);
    assertions::assert_passes(&doc, &config, "avg 20 <= max 25");
}

#[test]
fn average_over_limit_fails() {
    // 100 words / 3 sentences = 33 avg, max 25 → fail
    let doc = make_sentence_doc(100, 3);
    let config = config_with_avg_max(25);
    assertions::assert_average_failure(&doc, &config, 33, "avg 33 > max 25 should fail");
}

#[test]
fn zero_sentences_skips() {
    let doc = make_sentence_doc(50, 0);
    let config = config_with_avg_max(25);
    assertions::assert_skips(&doc, &config, "zero sentences skips");
}

#[test]
fn no_threshold_skips() {
    let doc = make_sentence_doc(100, 5);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "no threshold uses default pass");
}

#[test]
fn exact_boundary_passes() {
    // 75 words / 3 sentences = 25 avg, max 25 → pass (at_most is inclusive)
    let doc = make_sentence_doc(75, 3);
    let config = config_with_avg_max(25);
    assertions::assert_passes(&doc, &config, "exact boundary should pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn truncation_at_boundary_passes() {
    // 101 words / 4 sentences = 25 (integer division truncates 25.25 → 25)
    // With max=25, should pass because truncated value equals max.
    let doc = make_sentence_doc(101, 4);
    let config = config_with_avg_max(25);
    assertions::assert_passes(
        &doc,
        &config,
        "101/4 = 25 (truncated from 25.25) should pass with max=25",
    );
}

#[test]
fn truncation_above_boundary_fails() {
    // 104 words / 4 sentences = 26 (exact), max=25 → fail
    let doc = make_sentence_doc(104, 4);
    let config = config_with_avg_max(25);
    assertions::assert_fails(&doc, &config, "104/4 = 26 should fail with max=25");
}
