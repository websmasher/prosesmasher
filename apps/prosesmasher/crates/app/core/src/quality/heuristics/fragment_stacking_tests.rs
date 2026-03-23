use crate::check::Check;
use low_expectations::ExpectationSuite;
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
    let mut suite = ExpectationSuite::new("test");
    super::FragmentStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1);
    let vr = result.results.get("fragment-stacking");
    assert!(vr.is_some());
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some());
        assert_eq!(
            evidence
                .and_then(|items| items.first())
                .and_then(|item| item.get("sentences"))
                .and_then(serde_json::Value::as_array)
                .and_then(|items| items.first())
                .and_then(serde_json::Value::as_str),
            Some("Completely unreasonable.")
        );
    }
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
    let mut suite = ExpectationSuite::new("test");
    super::FragmentStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1);
    let vr = result.results.get("fragment-stacking");
    assert!(vr.is_some());
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert_eq!(evidence.map(Vec::len), Some(1));
    }
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
    let mut suite = ExpectationSuite::new("test");
    super::FragmentStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1);
}

#[test]
fn classification_catches_not_not_more_like_shape() {
    assert_eq!(
        super::classify_fragment(&make_sentence("Not morally wrong.")),
        Some("noun-fragment")
    );
    assert_eq!(
        super::classify_fragment(&make_sentence("Not bad-parent wrong.")),
        Some("noun-fragment")
    );
    assert_eq!(
        super::classify_fragment(&make_sentence("More like systems-failing-one-by-one wrong.")),
        Some("modifier-fragment")
    );
}

#[test]
fn short_narration_with_verbs_passes() {
    let doc = make_doc_with_sentences(
        &["Door slammed.", "Cat ran.", "Kids screamed."],
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FragmentStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1);
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc_with_sentences(
        &["Skipped snack.", "Too much noise.", "Weird nap."],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.fragment_stacking.enabled = false;
    let mut suite = ExpectationSuite::new("test");
    super::FragmentStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc_with_sentences(
        &["Skipped snack.", "Too much noise.", "Weird nap."],
        Locale::De,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::FragmentStackingCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}
