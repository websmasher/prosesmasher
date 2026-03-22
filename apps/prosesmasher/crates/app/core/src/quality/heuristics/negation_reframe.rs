//! Negation-reframe check — flags corrective "X, not Y" / "not Y. It's X." rhetoric.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph, Sentence};
use serde_json::{Value, json};

use crate::check::Check;

const ACTION_NEGATION_PHRASES: &[&str] = &[
    "could not",
    "did not",
    "do not",
    "does not",
    "cannot",
    "can't",
    "won't",
    "would not",
    "should not",
    "have not",
    "has not",
    "had not",
];

const COPULAR_NEGATION_STARTS: &[&str] = &[
    "this isn't ",
    "that isn't ",
    "it isn't ",
    "they aren't ",
    "you aren't ",
    "we aren't ",
    "this is not ",
    "that is not ",
    "it is not ",
    "they are not ",
    "you are not ",
    "we are not ",
    "this wasn't ",
    "that wasn't ",
    "it wasn't ",
    "they weren't ",
    "you weren't ",
    "we weren't ",
];

const AFFIRMATIVE_REFRAME_STARTS: &[&str] = &[
    "it's ",
    "it is ",
    "this is ",
    "that is ",
    "they're ",
    "they are ",
    "you're ",
    "you are ",
    "we're ",
    "we are ",
];

/// Detects corrective contrast rather than generic negation:
/// - inline "X, not Y"
/// - adjacent relabeling like "This isn't defiance. It's developmental."
#[derive(Debug)]
pub struct NegationReframeCheck;

impl Check for NegationReframeCheck {
    fn id(&self) -> &'static str {
        "negation-reframe"
    }

    fn label(&self) -> &'static str {
        "Negation-Reframe Pattern"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.negation_reframe.enabled {
            return;
        }
        if config.locale != Locale::En {
            return;
        }
        let evidence = collect_negation_reframe_evidence(doc);
        let _result = suite
            .record_custom_values(
                "negation-reframe",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Negation-Reframe Pattern")
            .checking("consecutive negation + reframe sentence pairs");
    }
}

fn collect_negation_reframe_evidence(
    doc: &Document,
) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_negation_reframe_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_negation_reframe_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            collect_negation_reframe_evidence_from_paragraph(
                paragraph,
                section_index,
                *paragraph_index,
                evidence,
            );
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_negation_reframe_evidence_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_negation_reframe_evidence_from_paragraph(
    para: &Paragraph,
    section_index: usize,
    paragraph_index: usize,
    evidence: &mut Vec<Value>,
) {
    for (sentence_index, sentence) in para.sentences.iter().enumerate() {
        if let Some(item) = inline_corrective_evidence(
            sentence,
            section_index,
            paragraph_index,
            sentence_index,
        ) {
            evidence.push(item);
        }
    }

    for (sentence_index, pair) in para.sentences.windows(2).enumerate() {
        let Some(a) = pair.first() else {
            continue;
        };
        let Some(b) = pair.get(1) else {
            continue;
        };
        if let Some(item) = adjacent_corrective_evidence(
            a,
            b,
            section_index,
            paragraph_index,
            sentence_index,
        ) {
            evidence.push(item);
        }
    }
}

fn inline_corrective_evidence(
    sentence: &Sentence,
    section_index: usize,
    paragraph_index: usize,
    sentence_index: usize,
) -> Option<Value> {
    let text = normalize_text(&sentence.text);
    if !looks_like_inline_corrective(&text, sentence.word_count()) {
        return None;
    }

    Some(json!({
        "section_index": section_index,
        "paragraph_index": paragraph_index,
        "sentence_index": sentence_index,
        "matched_text": "x, not y",
        "sentence": sentence.text,
        "pattern_type": "inline",
    }))
}

fn adjacent_corrective_evidence(
    a: &Sentence,
    b: &Sentence,
    section_index: usize,
    paragraph_index: usize,
    sentence_index: usize,
) -> Option<Value> {
    let a_text = normalize_text(&a.text);
    let b_text = normalize_text(&b.text);

    if !looks_like_negated_label_sentence(&a_text, a.word_count()) {
        return None;
    }
    if !looks_like_affirmative_relabel_sentence(&b_text, b.word_count()) {
        return None;
    }

    Some(json!({
        "section_index": section_index,
        "paragraph_index": paragraph_index,
        "sentence_index": sentence_index,
        "sentence_index_next": sentence_index.saturating_add(1),
        "matched_text": "not y -> x",
        "sentence": a.text,
        "next_sentence": b.text,
        "pattern_type": "adjacent",
    }))
}

fn looks_like_inline_corrective(text: &str, word_count: usize) -> bool {
    if word_count > 24 || !text.contains(" not ") {
        return false;
    }
    if contains_action_negation(text) {
        return false;
    }
    if text.contains(", not ") || text.contains(" but not ") {
        return has_copular_frame_before_not(text);
    }

    false
}

fn looks_like_negated_label_sentence(text: &str, word_count: usize) -> bool {
    if word_count > 12 || contains_action_negation(text) {
        return false;
    }

    text.starts_with("not ")
        || COPULAR_NEGATION_STARTS.iter().any(|prefix| text.starts_with(prefix))
}

fn looks_like_affirmative_relabel_sentence(text: &str, word_count: usize) -> bool {
    if word_count > 8 {
        return false;
    }

    AFFIRMATIVE_REFRAME_STARTS
        .iter()
        .any(|prefix| text.starts_with(prefix))
        || is_short_nominal_label(text, word_count)
}

fn contains_action_negation(text: &str) -> bool {
    ACTION_NEGATION_PHRASES
        .iter()
        .any(|phrase| text.contains(phrase))
}

fn has_copular_frame_before_not(text: &str) -> bool {
    let Some((before_not, _)) = text.split_once(" not ") else {
        return false;
    };

    [" is ", " are ", " was ", " were ", " be ", " being ", " been "]
        .iter()
        .any(|cue| before_not.contains(cue))
        || before_not.ends_with("'s")
        || before_not.ends_with("'re")
}

fn is_short_nominal_label(text: &str, word_count: usize) -> bool {
    if word_count == 0 || word_count > 3 {
        return false;
    }

    !text.contains(' ')
        || ![" is ", " are ", " was ", " were ", " have ", " has ", " had ", " do ", " does "]
            .iter()
            .any(|cue| text.contains(cue))
}

fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || ch.is_whitespace() || ch == '\'' || ch == ',' {
                ch
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "negation_reframe_tests.rs"]
mod tests;
