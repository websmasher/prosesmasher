use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    TermLists, Word,
};

fn make_sentences(texts: &[&str]) -> Vec<Sentence> {
    texts
        .iter()
        .map(|t| {
            let words: Vec<Word> = t
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
    let word_count: usize = sentences.iter().map(|s| s.words.len()).sum();
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

fn config_with_signals() -> CheckConfig {
    CheckConfig {
        terms: TermLists {
            negation_signals: vec!["not".to_owned(), "isn't".to_owned(), "aren't".to_owned()],
            reframe_signals: vec!["it's".to_owned(), "this is".to_owned(), "that's".to_owned()],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn negation_reframe_detected() {
    let doc = make_doc_with_sentences(
        &["This isn't defiance.", "It's developmental."],
        Locale::En,
    );
    let config = config_with_signals();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "negation + reframe pair should fail"
    );
    let vr = result.results.get("negation-reframe");
    assert!(vr.is_some(), "negation-reframe result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("matched_text"))
            .and_then(serde_json::Value::as_str), Some("isn't -> it's"), "matched pattern");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("This isn't defiance."), "first sentence");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("next_sentence"))
            .and_then(serde_json::Value::as_str), Some("It's developmental."), "second sentence");
    }
}

#[test]
fn no_pattern_passes() {
    let doc = make_doc_with_sentences(
        &[
            "It works more like a philosophy than a tool.",
            "The approach is unconventional.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "no negation-reframe pair should pass"
    );
}

#[test]
fn empty_config_skips() {
    let doc = make_doc_with_sentences(
        &["This isn't defiance.", "It's developmental."],
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "empty signals should skip"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::NegationReframeCheck;
    assert_eq!(check.id(), "negation-reframe");
    assert_eq!(check.label(), "Negation-Reframe Pattern");
    assert!(check.supported_locales().is_none());
}

#[test]
fn negation_reframe_inside_blockquote_detected() {
    let sentences = make_sentences(&["This isn't defiance.", "It's developmental."]);
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })])],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_with_signals();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1,
        "negation-reframe inside blockquote must be detected");
}

#[test]
fn code_block_ignored() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::CodeBlock("This isn't code. It's fine.".to_owned())],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_with_signals();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    // The check always emits one expectation (match_count between 0-0).
    // Code block content is correctly skipped, so match_count=0 → passes.
    assert_eq!(result.statistics.successful_expectations, 1,
        "code block content ignored → 0 matches → pass");
    assert_eq!(result.statistics.unsuccessful_expectations, 0,
        "no failures from code block");
}
