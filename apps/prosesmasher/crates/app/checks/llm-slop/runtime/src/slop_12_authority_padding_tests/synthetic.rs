use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::authority_padding as assertions;
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
fn repeated_research_frames_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "The research is not mysterious here.",
            "The broader research backs the same idea from different angles.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_authority_failure(
        &doc,
        &config,
        "research-frame",
        "the research",
        "repeated vague research scaffolding should fail",
    );
}

#[test]
fn mixed_evidence_and_prestige_frames_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "The evidence is strongest on a few points.",
            "John Gottman's work is famous for a reason.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_authority_failure(
        &doc,
        &config,
        "evidence-frame",
        "the evidence",
        "mixed authority-padding frames should fail once they repeat",
    );
}

#[test]
fn what_the_research_does_show_counts() {
    let doc = make_multi_sentence_doc(
        &[
            "What the research does show is messier and more annoying.",
            "The strongest recent evidence points to interruption and multitasking.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_authority_failure(
        &doc,
        &config,
        "research-frame",
        "the research",
        "research-does-show framing should count as authority padding",
    );
}

#[test]
fn repeated_authority_padding_inside_blockquote_fails() {
    let doc = make_multi_sentence_blockquote_doc(
        &[
            "The evidence is not subtle.",
            "The evidence points to a few boring but effective truths.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_authority_failure(
        &doc,
        &config,
        "evidence-frame",
        "the evidence",
        "blockquote authority padding should still count",
    );
}

#[test]
fn single_authority_sentence_passes() {
    let doc = make_doc("The evidence is strongest on a few points.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "single authority-padding sentence should stay under the default threshold",
    );
}

#[test]
fn specific_review_found_sentence_passes() {
    let doc = make_doc(
        "The 2023 review found that nurse burnout was associated with lower patient safety.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "specific review attribution should not count as vague authority padding",
    );
}

#[test]
fn concrete_health_agency_fact_passes() {
    let doc = make_doc(
        "The CDC says social isolation and loneliness are widespread in the U.S.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "plain agency attribution should pass without vague authority scaffolding",
    );
}

#[test]
fn quoted_authority_phrase_passes() {
    let doc = make_doc(
        "Editors should cut lines like \"the evidence is not subtle\" when they add no precision.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "quoted authority-padding discussion should pass",
    );
}

#[test]
fn code_block_authority_padding_passes() {
    let doc = make_doc_code_only("The evidence is strongest on a few points.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("The evidence is strongest on a few points.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_multi_sentence_doc(
        &[
            "The evidence is strongest on a few points.",
            "The broader research backs the same idea from different angles.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.authority_padding.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled authority-padding should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &[
            "The evidence is strongest on a few points.",
            "The broader research backs the same idea from different angles.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.authority_padding.max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two authority-padding hits should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
