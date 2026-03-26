use prosesmasher_app_checks_flow_assertions::word_repetition as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
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

fn doc_with_paragraph_text(text: &str) -> Document {
    let words = text
        .split_whitespace()
        .map(|token| Word {
            text: token
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_owned(),
            syllable_count: 1,
        })
        .collect();

    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence {
                    text: text.to_owned(),
                    words,
                }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata::default(),
    }
}

fn config_with_repetition_max(max: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.flow.word_repetition.max = max;
    config
}

#[test]
fn word_exceeds_max_fails() {
    let doc = doc_with_repeated_word("actually", 7);
    let config = config_with_repetition_max(5);
    assertions::assert_word_repetition_failure(
        &doc,
        &config,
        "actually",
        7,
        "actually actually actually actually actually actually actually",
        "actually x7 with max=5 should fail",
    );
}

#[test]
fn word_within_max_passes() {
    let doc = doc_with_repeated_word("actually", 3);
    let config = config_with_repetition_max(5);
    assertions::assert_passes(&doc, &config, "actually x3 with max=5 should pass");
}

#[test]
fn short_words_ignored() {
    let doc = doc_with_repeated_word("the", 10);
    let config = config_with_repetition_max(5);
    assertions::assert_passes(
        &doc,
        &config,
        "words < 4 chars should produce one passing aggregate check",
    );
}

#[test]
fn stop_words_ignored() {
    let doc = doc_with_repeated_word("that", 10);
    let mut config = config_with_repetition_max(5);
    config.quality.flow.word_repetition.excluded_terms.add = vec!["that".to_owned()];
    assertions::assert_passes(
        &doc,
        &config,
        "excluded words should produce one passing aggregate check",
    );
}

#[test]
fn no_threshold_skips() {
    let doc = doc_with_repeated_word("actually", 4);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "default quality config should run word repetition with built-in max",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn multi_section_aggregation_exceeds_max() {
    // Paragraph-local repetition should not fail when counts only exceed the
    // threshold across the whole document.
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "actually actually actually filler words here",
            "actually actually actually other words here",
        ],
        Locale::En,
    );
    let config = config_with_repetition_max(5);
    assertions::assert_passes(&doc, &config, "spread-out repetition should not fail");
}

#[test]
fn paragraph_local_repetition_fails_even_if_document_is_multi_section() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "banana banana banana banana banana banana split",
            "actually actually actually other words here",
        ],
        Locale::En,
    );
    let config = config_with_repetition_max(5);
    assertions::assert_word_repetition_failure(
        &doc,
        &config,
        "banana",
        6,
        "banana banana banana banana banana banana split",
        "local repetition within one paragraph should fail",
    );
}

#[test]
fn mdx_component_like_paragraph_is_ignored() {
    let doc = doc_with_paragraph_text(
        "<BlogFAQ items={[ { question: \"One?\" }, { question: \"Two?\" }, { question: \"Three?\" }, { question: \"Four?\" }, { question: \"Five?\" }, { question: \"Six?\" } ]} />",
    );
    let config = config_with_repetition_max(5);
    assertions::assert_passes(
        &doc,
        &config,
        "component-like paragraphs should not count as prose repetition",
    );
}
