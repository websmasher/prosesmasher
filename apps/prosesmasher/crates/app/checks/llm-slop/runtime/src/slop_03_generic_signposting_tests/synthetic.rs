use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::generic_signposting as assertions;
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
fn repeated_important_to_signposts_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "It's important to note that eczema triggers vary by person.",
            "It's important to remember that treatment plans vary too.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "important-to",
        "it's important to note",
        "repeated important-to signposts should fail",
    );
}

#[test]
fn mixed_signpost_families_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "That being said, outcomes vary by case.",
            "It's always best to consult a qualified healthcare professional.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "transition",
        "that being said",
        "mixed generic signposts should fail once they repeat in the document",
    );
}

#[test]
fn repeated_signposting_inside_blockquote_fails() {
    let doc = make_multi_sentence_blockquote_doc(
        &[
            "Please note that this is general information.",
            "As such, it should not replace individual medical guidance.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "note-signpost",
        "please note that",
        "blockquote signposting should still count",
    );
}

#[test]
fn one_signpost_passes() {
    let doc = make_doc(
        "It's important to note that eczema triggers vary by person.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "a single signpost should stay under the default threshold",
    );
}

#[test]
fn ordinary_consultation_without_signpost_passes() {
    let doc = make_doc(
        "Consult a qualified healthcare professional for personalized advice.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "plain consultation advice should not count as generic signposting",
    );
}

#[test]
fn quoted_signposts_pass() {
    let doc = make_doc(
        "The phrase \"it's important to note\" is one of the most recycled transitions in bad AI copy.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "quoted discussion of the signpost should pass",
    );
}

#[test]
fn code_block_signposts_pass() {
    let doc = make_doc_code_only(
        "That being said, it's always best to consult a qualified healthcare professional.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc(
        "It is important to note that eczema triggers vary by person.",
        Locale::Fr,
    );
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc(
        "It is important to note that eczema triggers vary by person. It is important to remember that treatment plans vary too.",
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.generic_signposting.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled generic-signposting should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &[
            "It is important to note that eczema triggers vary by person.",
            "It is important to remember that treatment plans vary too.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.generic_signposting.max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two signposts should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
