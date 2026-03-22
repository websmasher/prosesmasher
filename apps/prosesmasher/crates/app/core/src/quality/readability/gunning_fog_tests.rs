use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence,
    Word,
};

/// Build a doc where each word has a specified syllable count.
/// `syllable_counts` controls per-word syllable counts.
fn make_fog_doc(syllable_counts: &[usize], total_sentences: usize) -> Document {
    let words: Vec<Word> = syllable_counts
        .iter()
        .enumerate()
        .map(|(i, &sc)| Word {
            text: format!("w{i}"),
            syllable_count: sc,
        })
        .collect();
    let total_words = words.len();
    let total_syllables: usize = syllable_counts.iter().copied().sum();
    let text = words
        .iter()
        .map(|w| w.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence { text, words }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences,
            total_syllables,
            ..Default::default()
        },
    }
}

fn config_with_fog_max(max: f64) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality.heuristics.readability.gunning_fog_max = Some(max);
    config
}

#[test]
fn simple_text_passes() {
    // 20 words, 2 sentences, 0 complex (all 1-syllable)
    // fog = 0.4 × (20/2 + 100×0/20) = 0.4 × 10 = 4.0
    let syllables = vec![1; 20];
    let doc = make_fog_doc(&syllables, 2);
    let config = config_with_fog_max(12.0);
    let mut suite = ExpectationSuite::new("test");
    super::GunningFogCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "simple text should pass");
    assert_eq!(result.statistics.unsuccessful_expectations, 0);
}

#[test]
fn complex_text_fails() {
    // 20 words, 2 sentences, 10 complex (3+ syllables)
    // fog = 0.4 × (20/2 + 100×10/20) = 0.4 × (10 + 50) = 0.4 × 60 = 24.0
    let mut syllables = vec![1; 10];
    syllables.extend(vec![3; 10]); // 10 complex words
    let doc = make_fog_doc(&syllables, 2);
    let config = config_with_fog_max(12.0);
    let mut suite = ExpectationSuite::new("test");
    super::GunningFogCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "complex text should fail (fog 24 > max 12)"
    );
    let vr = result.results.get("gunning-fog");
    assert!(vr.is_some(), "gunning-fog result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(evidence.and_then(|e| e.first())
            .and_then(|item| item.get("complex_word_count"))
            .and_then(serde_json::Value::as_i64), Some(10), "complex word count");
    }
}

#[test]
fn zero_words_skips() {
    let doc = make_fog_doc(&[], 0);
    let config = config_with_fog_max(12.0);
    let mut suite = ExpectationSuite::new("test");
    super::GunningFogCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn zero_sentences_skips() {
    let syllables = vec![1; 10];
    let doc = make_fog_doc(&syllables, 0);
    let config = config_with_fog_max(12.0);
    let mut suite = ExpectationSuite::new("test");
    super::GunningFogCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0);
}

#[test]
fn no_threshold_skips() {
    let syllables = vec![1; 20];
    let doc = make_fog_doc(&syllables, 2);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::GunningFogCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1);
}

#[test]
fn check_id_and_label() {
    let check = super::GunningFogCheck;
    assert_eq!(check.id(), "gunning-fog");
    assert_eq!(check.label(), "Gunning Fog Index");
    assert!(check.supported_locales().is_none());
}
