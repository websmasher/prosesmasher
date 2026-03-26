use prosesmasher_app_checks_heuristics_assertions::negation_reframe as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
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
    let doc = make_doc_with_sentences(&["This isn't defiance.", "It's developmental."], Locale::En);
    let config = config_with_signals();
    assertions::assert_negation_reframe_pair(
        &doc,
        &config,
        "not y -> x",
        "This isn't defiance.",
        "It's developmental.",
        "negation + reframe pair should fail",
    );
}

#[test]
fn inline_corrective_detected() {
    let doc = make_doc_with_sentences(
        &["The goal is corrective contrast, not generic negation."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x, not y",
        "inline x-not-y contrast should fail",
    );
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
    assertions::assert_passes(&doc, &config, "action negation plus narration should pass");
}

#[test]
fn infinitive_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Not to act on the anger.",
            "To notice you're assembling a bonfire and stop adding to it.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "not to x -> to y",
        "infinitive contrast detected",
    );
}

#[test]
fn meaning_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "That does not mean you're failing.",
            "It means your alarm system is miscalibrated.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "does not x -> it xs",
        "meaning contrast detected",
    );
}

#[test]
fn same_root_framing_contrast_detected() {
    let doc = make_doc_with_sentences(
        &["That does not reflect defiance.", "It reflects overload."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_fails(&doc, &config, "same-root framing contrast detected");
}

#[test]
fn same_root_non_framing_pair_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "That does not make the tantrums fun.",
            "It does make them easier to read.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "same-root non-framing pair does not trigger");
}

#[test]
fn technical_explanation_without_same_root_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "That does not mean the server is healthy.",
            "It needs a deeper health check.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "technical explanation without same root does not trigger",
    );
}

#[test]
fn internal_state_expression_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Kids who learn that crying gets no response don't stop having feelings.",
            "They stop showing them.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "don't x -> they y",
        "internal state expression contrast detected",
    );
}

#[test]
fn normal_behavioral_followup_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "Children don't stop at the corner.",
            "They turn left instead.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "normal behavioral followup does not trigger");
}

#[test]
fn narrative_frame_contrast_detected() {
    let doc = make_doc_with_sentences(
        &["It doesn't begin the story.", "It ends the buildup."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "doesn't begin x -> it ends y",
        "narrative frame contrast detected",
    );
}

#[test]
fn shared_progressive_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "I was not living with a tiny chaos agent who woke up each day searching for weak points in my character.",
            "I was living with a child who keeps hitting the edge of her capacity and does not know that is what is happening.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_fails(&doc, &config, "shared progressive corrective detected");
}

#[test]
fn explicit_make_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "That doesn't make the meltdowns fun.",
            "But it makes them something I can read.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "doesn't make x -> but it makes y",
        "explicit make contrast detected",
    );
}

#[test]
fn less_more_like_pair_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Less like a judge.",
            "More like someone who got there late and is trying to understand what happened before she arrived.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_fails(&doc, &config, "less more like pair detected");
}

#[test]
fn ordinary_begin_end_pair_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &["The meeting doesn't begin on time.", "It ends at five."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "ordinary begin end pair does not trigger");
}

#[test]
fn lifecycle_frame_reversal_detected() {
    let doc = make_doc_with_sentences(
        &[
            "And when I miss the signs I try to remember that the screaming doesn't begin the story.",
            "It ends the buildup.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "doesn't begin x -> it ends y",
        "lifecycle frame reversal detected",
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
    assertions::assert_passes(&doc, &config, "no negation-reframe pair should pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc_with_sentences(&["This isn't defiance.", "It's developmental."], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(
        &doc,
        &config,
        "default negation/reframe patterns should run",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
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
    assertions::assert_fails(
        &doc,
        &config,
        "negation-reframe inside blockquote must be detected",
    );
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
    assertions::assert_passes(
        &doc,
        &config,
        "code block content ignored → 0 matches → pass",
    );
}
