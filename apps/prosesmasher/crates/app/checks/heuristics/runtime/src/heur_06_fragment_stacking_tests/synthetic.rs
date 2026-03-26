use crate::fragment_stacking::classify_fragment;
use prosesmasher_app_checks_heuristics_assertions::fragment_stacking as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn make_sentences(texts: &[&str]) -> Vec<Sentence> {
    texts
        .iter()
        .map(|t| {
            let words = t
                .split_whitespace()
                .map(|w| Word {
                    text: w.to_owned(),
                    syllable_count: 1,
                })
                .collect();
            Sentence {
                text: (*t).to_owned(),
                words,
            }
        })
        .collect()
}

fn make_doc_with_sentences(texts: &[&str], locale: Locale) -> Document {
    let sentences = make_sentences(texts);
    let word_count: usize = sentences.iter().map(Sentence::word_count).sum();
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words: word_count,
            total_sentences: texts.len(),
            ..Default::default()
        },
    }
}

fn make_sentence(text: &str) -> Sentence {
    Sentence {
        text: text.to_owned(),
        words: text
            .split_whitespace()
            .map(|w| Word {
                text: w.to_owned(),
                syllable_count: 1,
            })
            .collect(),
    }
}

#[test]
fn clipped_fragments_fail() {
    let doc = make_doc_with_sentences(
        &[
            "Completely unreasonable.",
            "Possibly manipulative.",
            "Most offensive of all, deeply inconvenient.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_fragment_failure_with_first_sentence(
        &doc,
        &config,
        "Completely unreasonable.",
        "clipped fragments should fail",
    );
}

#[test]
fn skipped_snack_style_fragments_merge_into_one_run() {
    let doc = make_doc_with_sentences(
        &[
            "Skipped snack.",
            "Too much noise.",
            "Weird nap.",
            "One errand too many.",
            "Me already fried.",
            "Then the cup.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_fragment_failure_count(
        &doc,
        &config,
        1,
        "skipped snack style fragments should merge into one run",
    );
}

#[test]
fn short_short_long_payoff_fails() {
    let doc = make_doc_with_sentences(
        &[
            "Not morally wrong.",
            "Not bad-parent wrong.",
            "More like systems-failing-one-by-one wrong.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "short short long payoff fails");
}

#[test]
fn classification_catches_not_not_more_like_shape() {
    assert_eq!(
        classify_fragment(&make_sentence("Not morally wrong.")),
        Some("noun-fragment")
    );
    assert_eq!(
        classify_fragment(&make_sentence("Not bad-parent wrong.")),
        Some("noun-fragment")
    );
    assert_eq!(
        classify_fragment(&make_sentence(
            "More like systems-failing-one-by-one wrong."
        )),
        Some("modifier-fragment")
    );
}

#[test]
fn short_narration_with_verbs_passes() {
    let doc = make_doc_with_sentences(&["Door slammed.", "Cat ran.", "Kids screamed."], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "short narration with verbs passes");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc_with_sentences(
        &["Skipped snack.", "Too much noise.", "Weird nap."],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.fragment_stacking.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled check skips");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc_with_sentences(
        &["Skipped snack.", "Too much noise.", "Weird nap."],
        Locale::De,
    );
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-english is skipped");
}
