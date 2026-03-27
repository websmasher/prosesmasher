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
const ABSTRACT_FRAME_CONSTRUCTIONS: &[(&str, &str, &str, &str)] = &[
    ("better", "move", "is", "the-better-move-is"),
    ("bigger", "win", "is", "the-bigger-win-is"),
    ("useful", "move", "is", "the-useful-move-is"),
    ("useful", "alternative", "is", "the-useful-alternative-is"),
    (
        "useful",
        "alternatives",
        "are",
        "the-useful-alternatives-are",
    ),
];
const POINT_ABSTRACT_VERBS: &[&str] = &[
    "become",
    "build",
    "interrupt",
    "keep",
    "make",
    "remember",
    "respect",
    "stop",
    "understand",
];
const WHAT_FRAME_VERBS: &[(&str, &'static str)] =
    &[("helps", "what-helps-is"), ("matters", "what-matters-is")];
const WHAT_FRAME_TAIL_STARTERS: &[&str] = &[
    "boring", "less", "making", "more", "never", "not", "rarely", "small", "smaller", "usually",
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
        "question-frame"
            | "answer-frame"
            | "sequence-frame"
            | "frame-signpost"
            | "abstract-evaluation-frame"
    )
}

fn match_signposting(sentence: &str) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));
    if let Some(matched) = match_abstract_evaluation_frame(&stripped) {
        return Some(("abstract-evaluation-frame", matched));
    }

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

fn match_abstract_evaluation_frame(text: &str) -> Option<&'static str> {
    let tokens = token_words(text);
    if let Some(matched) = matches_modified_abstract_frame(&tokens) {
        return Some(matched);
    }
    if let Some(matched) = matches_point_is_to_frame(&tokens) {
        return Some(matched);
    }
    matches_what_frame(&tokens)
}

fn matches_modified_abstract_frame(tokens: &[&str]) -> Option<&'static str> {
    match tokens {
        ["the", "result", "worth", "caring", "about", ..] => Some("the-result-worth-caring-about"),
        ["the", modifier, noun, linker, ..] => ABSTRACT_FRAME_CONSTRUCTIONS.iter().find_map(
            |(expected_modifier, expected_noun, expected_linker, matched_text)| {
                (*modifier == *expected_modifier
                    && *noun == *expected_noun
                    && *linker == *expected_linker)
                    .then_some(*matched_text)
            },
        ),
        _ => None,
    }
}

fn matches_point_is_to_frame(tokens: &[&str]) -> Option<&'static str> {
    match tokens {
        ["the", "point", "is", "to", verb, ..] if POINT_ABSTRACT_VERBS.contains(verb) => {
            Some("the-point-is-to")
        }
        _ => None,
    }
}

fn matches_what_frame(tokens: &[&str]) -> Option<&'static str> {
    match tokens {
        ["what", verb, "most", "is", tail, ..] if *verb == "matters" => WHAT_FRAME_TAIL_STARTERS
            .contains(tail)
            .then_some("what-matters-most-is"),
        ["what", verb, "is", tail, ..] => {
            WHAT_FRAME_VERBS.iter().find_map(|(candidate, matched)| {
                (*verb == *candidate && WHAT_FRAME_TAIL_STARTERS.contains(tail)).then_some(*matched)
            })
        }
        _ => None,
    }
}

fn token_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "slop_03_generic_signposting_tests/mod.rs"]
mod tests;
