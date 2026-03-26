use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::llm_vocabulary as assertions;
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
fn repeated_same_term_fails() {
    let doc = make_multi_sentence_doc(
        &[
            "We delve into the tradeoffs first.",
            "Then we delve deeper into the performance profile.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_vocabulary_failure(
        &doc,
        &config,
        "delve",
        "repeated stock vocabulary should fail",
    );
}

#[test]
fn mixed_terms_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "This guide offers a comprehensive overview.",
            "Moreover, the rollout plan stays vague.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_vocabulary_failure(
        &doc,
        &config,
        "comprehensive",
        "mixed stock vocabulary should fail once it repeats",
    );
}

#[test]
fn repeated_terms_inside_blockquote_fail() {
    let doc = make_multi_sentence_blockquote_doc(
        &["A vibrant roadmap helps.", "A vibrant narrative does not."],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_vocabulary_failure(
        &doc,
        &config,
        "vibrant",
        "blockquote prose should still count",
    );
}

#[test]
fn one_term_passes() {
    let doc = make_doc("This guide offers a comprehensive overview.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "one stock vocabulary hit should stay under the threshold",
    );
}

#[test]
fn quoted_discussion_passes() {
    let doc = make_doc(
        "The word \"delve\" is one of the easiest tells in canned AI prose.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted discussion should pass");
}

#[test]
fn code_block_terms_pass() {
    let doc = make_doc_code_only(
        "Moreover, we delve into a comprehensive abstraction.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("Moreover, this guide stays comprehensive.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_multi_sentence_doc(
        &[
            "This guide offers a comprehensive overview.",
            "Moreover, the rollout plan stays vague.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.llm_vocabulary.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled llm-vocabulary should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &[
            "This guide offers a comprehensive overview.",
            "Moreover, the rollout plan stays vague.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.llm_vocabulary.max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two vocabulary hits should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
