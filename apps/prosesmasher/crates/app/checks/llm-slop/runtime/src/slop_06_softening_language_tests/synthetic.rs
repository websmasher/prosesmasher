use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::softening_language as assertions;
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
fn repeated_variability_disclaimers_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "What may trigger one person's eczema may not necessarily trigger another's.",
            "Some people with eczema may be sensitive to dairy.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_softening_failure(
        &doc,
        &config,
        "variability-softening",
        "may + may not necessarily",
        "repeated variability-heavy softening should fail",
    );
}

#[test]
fn modal_and_qualifier_stack_fails() {
    let doc = make_multi_sentence_doc(
        &[
            "Certain foods may commonly trigger eczema flare-ups.",
            "These reactions may typically vary by season.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_softening_failure(
        &doc,
        &config,
        "hedged-claim",
        "may + commonly",
        "stacked modal qualifiers should fail",
    );
}

#[test]
fn blockquote_softening_still_counts() {
    let doc = make_multi_sentence_blockquote_doc(
        &[
            "Some studies suggest that certain foods may help.",
            "More research is needed, and some people may respond differently.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_softening_failure(
        &doc,
        &config,
        "quantified-softening",
        "may + some studies",
        "blockquote softening stacks should still count",
    );
}

#[test]
fn one_hedged_sentence_passes() {
    let doc = make_doc("Some people may respond differently.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "one softening sentence should stay under the default threshold",
    );
}

#[test]
fn quoted_softening_discussion_passes() {
    let doc = make_doc(
        "The phrase \"some people may\" is a classic low-commitment crutch in canned copy.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted discussion should pass");
}

#[test]
fn code_block_softening_passes() {
    let doc = make_doc_code_only(
        "Some studies suggest that certain foods may help.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn ordinary_capability_sentences_pass() {
    let doc = make_multi_sentence_doc(
        &[
            "The parser can read markdown files.",
            "The tool can emit JSON output.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "plain capability statements should not count as softening",
    );
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("Some people may respond differently.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_multi_sentence_doc(
        &[
            "What may trigger one person's eczema may not necessarily trigger another's.",
            "Some people with eczema may be sensitive to dairy.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.softening_language.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled softening-language should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &[
            "What may trigger one person's eczema may not necessarily trigger another's.",
            "Some people with eczema may be sensitive to dairy.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.softening_language.max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two softening hits should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
