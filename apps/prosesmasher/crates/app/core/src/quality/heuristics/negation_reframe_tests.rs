use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    Word,
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
    CheckConfig::default()
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
            .and_then(serde_json::Value::as_str), Some("not y -> x"), "matched pattern");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("sentence"))
            .and_then(serde_json::Value::as_str), Some("This isn't defiance."), "first sentence");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("next_sentence"))
            .and_then(serde_json::Value::as_str), Some("It's developmental."), "second sentence");
    }
}

#[test]
fn inline_corrective_detected() {
    let doc = make_doc_with_sentences(
        &["The goal is corrective contrast, not generic negation."],
        Locale::En,
    );
    let config = config_with_signals();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "inline x-not-y contrast should fail"
    );
    let vr = result.results.get("negation-reframe");
    assert!(vr.is_some(), "negation-reframe result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("pattern_type"))
            .and_then(serde_json::Value::as_str), Some("inline"), "pattern type");
    }
}

#[test]
fn action_negation_narration_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "I could not fix the banana.",
            "My second instinct was to explain that bananas sometimes break and this is fine and we can eat both pieces.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "action negation plus narration should pass"
    );
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "banana-style narrative pair must not fail"
    );
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
fn default_config_runs() {
    let doc = make_doc_with_sentences(
        &["This isn't defiance.", "It's developmental."],
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::NegationReframeCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "default negation/reframe patterns should run"
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
