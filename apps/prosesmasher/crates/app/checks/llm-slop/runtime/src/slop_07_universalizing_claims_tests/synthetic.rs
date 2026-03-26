use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::universalizing_claims as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn make_sentence(text: &str) -> Sentence {
    Sentence {
        text: text.to_owned(),
        words: text
            .split_whitespace()
            .map(|word| Word {
                text: word.to_owned(),
                syllable_count: 1,
            })
            .collect(),
    }
}

fn make_multi_sentence_doc(sentences: &[&str], locale: Locale) -> Document {
    let sentence_values: Vec<Sentence> = sentences.iter().map(|text| make_sentence(text)).collect();
    let total_words: usize = sentence_values
        .iter()
        .map(|sentence| sentence.words.len())
        .sum();
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: sentence_values,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences: sentences.len(),
            ..Default::default()
        },
    }
}

fn make_multi_sentence_blockquote_doc(sentences: &[&str], locale: Locale) -> Document {
    let sentence_values: Vec<Sentence> = sentences.iter().map(|text| make_sentence(text)).collect();
    let total_words: usize = sentence_values
        .iter()
        .map(|sentence| sentence.words.len())
        .sum();
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(Paragraph {
                sentences: sentence_values,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })])],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences: sentences.len(),
            ..Default::default()
        },
    }
}

#[test]
fn repeated_collective_desire_claims_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "Everyone wants certainty when things fall apart.",
            "We all want the pain to end quickly.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_universalizing_failure(
        &doc,
        &config,
        "collective-desire",
        "everyone wants",
        "repeated broad desire claims should fail",
    );
}

#[test]
fn mixed_certainty_and_desire_claims_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "After all, most people know what panic feels like.",
            "Nobody wants to sit in uncertainty forever.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_universalizing_failure(
        &doc,
        &config,
        "collective-certainty",
        "most people know",
        "mixed universalizing claim families should fail once they repeat",
    );
}

#[test]
fn repeated_claims_inside_blockquote_fail() {
    let doc = make_multi_sentence_blockquote_doc(
        &[
            "Everyone needs relief when pain spikes.",
            "No one wants to feel trapped in their own head.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_universalizing_failure(
        &doc,
        &config,
        "collective-desire",
        "everyone needs",
        "blockquote universalizing claims should still count",
    );
}

#[test]
fn one_claim_passes_under_default_threshold() {
    let doc = make_doc("Everyone wants stability during a crisis.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "a single universalizing claim should stay under the default threshold",
    );
}

#[test]
fn narrative_everyone_usage_passes() {
    let doc = make_doc(
        "The blast left everyone shaking. Later, when everyone's nervous system reset, the room finally went quiet.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "literal narrative everyone-usage should not count as a universalizing claim",
    );
}

#[test]
fn not_everyone_claim_passes() {
    let doc = make_doc(
        "Not everyone remembers their dreams, and some people never do.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "qualified non-universal claims should not count",
    );
}

#[test]
fn quoted_claim_passes() {
    let doc = make_doc(
        "The phrase \"everyone wants certainty\" is one of the most canned tricks in bad copy.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "quoted discussion of the universalizing line should pass",
    );
}

#[test]
fn code_block_claim_passes() {
    let doc = make_doc_code_only(
        "Everyone wants certainty. We all want the pain to end.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("Everyone wants certainty.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_multi_sentence_doc(
        &["Everyone wants certainty.", "We all want the pain to end."],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.universalizing_claims.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled universalizing-claims should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &["Everyone wants certainty.", "We all want the pain to end."],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config
        .quality
        .heuristics
        .universalizing_claims
        .max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two hits should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
