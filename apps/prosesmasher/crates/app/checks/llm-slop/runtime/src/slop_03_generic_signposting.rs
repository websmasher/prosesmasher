//! Generic-signposting check — flags repeated stock rhetorical scaffolding.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, contains_any, normalize, sentence_evidence, strip_leading_prefixes,
    strip_quoted_segments,
};

#[derive(Debug)]
pub struct GenericSignpostingCheck;

impl Check for GenericSignpostingCheck {
    fn id(&self) -> &'static str {
        "generic-signposting"
    }

    fn label(&self) -> &'static str {
        "Generic Signposting"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.generic_signposting.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config
            .quality
            .heuristics
            .generic_signposting
            .max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_generic_signposting_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let has_strong_meta_frame = evidence.iter().any(is_strong_meta_evidence);
        let _result = suite
            .record_custom_values(
                "generic-signposting",
                !has_strong_meta_frame && observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("Generic Signposting")
            .checking("repeated stock signposting phrases per document");
    }
}

const LEADING_PREFIXES: &[&str] = &["however, ", "but ", "and ", "so "];

const IMPORTANT_TO_PATTERNS: &[&str] = &[
    "it's important to note",
    "it is important to note",
    "it's important to remember",
    "it is important to remember",
];

const TRANSITION_PATTERNS: &[&str] = &["that being said", "as such"];

const CONSULTATION_PATTERNS: &[&str] = &[
    "it's always best to consult",
    "it is always best to consult",
    "it's best to consult",
    "it is best to consult",
    "it's recommended to consult",
    "it is recommended to consult",
];

const NOTE_PATTERNS: &[&str] = &["please note that", "please note"];
const QUESTION_PATTERNS: &[&str] = &[
    "the useful question is",
    "the useful move is",
    "the practical move is",
    "the real question is",
    "the better question is",
];
const ANSWER_PATTERNS: &[&str] = &[
    "the answer is simple",
    "the answer is straightforward",
    "the practical answer is",
    "the short answer is",
    "the better conclusion is",
    "the useful conclusion is simple",
];
const FRAME_PATTERNS: &[&str] = &[
    "the short version",
    "the practical version is",
    "the useful frame",
    "the useful version is",
    "the point is plain enough",
];
const SEQUENCE_PATTERNS: &[&str] = &[
    "a simple sequence works well",
    "a simple pattern works well",
    "a simple rule works well",
];

fn collect_generic_signposting_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
            match_signposting(sentence).map(|(pattern_kind, matched_text)| {
                sentence_evidence(
                    section_index,
                    paragraph_index,
                    sentence_index,
                    &[
                        ("pattern_kind", pattern_kind),
                        ("matched_text", matched_text),
                        ("sentence", sentence),
                    ],
                )
            })
        },
    )
}

fn is_strong_meta_evidence(evidence: &Value) -> bool {
    let Some(pattern_kind) = evidence.get("pattern_kind").and_then(Value::as_str) else {
        return false;
    };
    matches!(
        pattern_kind,
        "question-frame" | "answer-frame" | "sequence-frame" | "frame-signpost"
    )
}

fn match_signposting(sentence: &str) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));

    if let Some(matched) = contains_any(&stripped, IMPORTANT_TO_PATTERNS) {
        return Some(("important-to", matched));
    }
    if let Some(matched) = contains_any(&stripped, TRANSITION_PATTERNS) {
        return Some(("transition", matched));
    }
    if let Some(matched) = contains_any(&stripped, CONSULTATION_PATTERNS) {
        return Some(("consultation-signpost", matched));
    }
    if let Some(matched) = contains_any(&stripped, NOTE_PATTERNS) {
        return Some(("note-signpost", matched));
    }
    if let Some(matched) = contains_any(&stripped, QUESTION_PATTERNS) {
        return Some(("question-frame", matched));
    }
    if let Some(matched) = contains_any(&stripped, ANSWER_PATTERNS) {
        return Some(("answer-frame", matched));
    }
    if let Some(matched) = contains_any(&stripped, FRAME_PATTERNS) {
        return Some(("frame-signpost", matched));
    }
    contains_any(&stripped, SEQUENCE_PATTERNS).map(|matched| ("sequence-frame", matched))
}

#[cfg(test)]
#[path = "slop_03_generic_signposting_tests/mod.rs"]
mod tests;
