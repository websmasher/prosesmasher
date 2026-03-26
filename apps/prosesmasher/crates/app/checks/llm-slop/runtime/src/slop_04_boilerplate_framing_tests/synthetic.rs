use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::boilerplate_framing as assertions;
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
    let total_words: usize = sentence_values.iter().map(|sentence| sentence.words.len()).sum();
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
    let total_words: usize = sentence_values.iter().map(|sentence| sentence.words.len()).sum();
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
fn repeated_enumeration_prefaces_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "Some examples of failed launches include cancelled dates and silent rewrites.",
            "Some common reasons include weak demand and unclear ownership.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_framing_failure(
        &doc,
        &config,
        "enumeration-preface",
        "some examples + include",
        "repeated vague list-preface framing should fail",
    );
}

#[test]
fn two_distinct_framing_moves_in_one_sentence_fail() {
    let doc = make_doc(
        "There are certain types of failures that show up repeatedly, and some examples of failures include missing ownership and vague contracts.",
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.boilerplate_framing.max_per_document = 0;
    assertions::assert_framing_failure(
        &doc,
        &config,
        "existence-frame",
        "there are certain/common",
        "two distinct framing moves in one sentence should both count",
    );
}

#[test]
fn mixed_preview_and_topic_frames_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "When it comes to debugging, most failures are ownership bugs.",
            "In the following sections, we explore the fixes and tradeoffs.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_framing_failure(
        &doc,
        &config,
        "topic-frame",
        "when it comes to",
        "mixed staging frames should fail once they repeat in the document",
    );
}

#[test]
fn repeated_framing_inside_blockquote_fails() {
    let doc = make_multi_sentence_blockquote_doc(
        &[
            "When it comes to eczema, triggers vary by person.",
            "In the following sections, we explore the most common patterns.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_framing_failure(
        &doc,
        &config,
        "topic-frame",
        "when it comes to",
        "blockquote framing should still count",
    );
}

#[test]
fn one_framing_sentence_passes() {
    let doc = make_doc(
        "When it comes to debugging, ownership mistakes dominate.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "a single framing sentence should stay under the default threshold",
    );
}

#[test]
fn specific_list_intro_without_vague_preface_passes() {
    let doc = make_doc(
        "Supported targets include Linux, macOS, and Windows.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "specific list introductions should not count as boilerplate framing",
    );
}

#[test]
fn quoted_framing_passes() {
    let doc = make_doc(
        "The phrase \"when it comes to\" is one of the laziest setup moves in canned copy.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted framing discussion should pass");
}

#[test]
fn code_block_framing_passes() {
    let doc = make_doc_code_only(
        "In the following sections, we explore the API surface.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("When it comes to eczema, triggers vary by person.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_multi_sentence_doc(
        &[
            "When it comes to debugging, ownership mistakes dominate.",
            "In the following sections, we explore the fixes.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.boilerplate_framing.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled boilerplate-framing should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &[
            "When it comes to debugging, ownership mistakes dominate.",
            "In the following sections, we explore the fixes.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.boilerplate_framing.max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two framing hits should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
