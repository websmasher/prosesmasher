//! Slogan-punchline check — flags short sloganized punchlines and paired moral curricula lines.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;
use crate::support::{
    collect_adjacent_sentence_evidence, collect_sentence_evidence, normalize, sentence_evidence,
    strip_leading_prefixes, strip_quoted_segments,
};

#[derive(Debug)]
pub struct SloganPunchlineCheck;

impl Check for SloganPunchlineCheck {
    fn id(&self) -> &'static str {
        "slogan-punchline"
    }

    fn label(&self) -> &'static str {
        "Slogan Punchline"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.slogan_punchline.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_slogan_punchline_evidence(doc);
        let _result = suite
            .record_custom_values(
                "slogan-punchline",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Slogan Punchline")
            .checking("short sloganized punchline lines");
    }
}

const LEADING_PREFIXES: &[&str] = &["and ", "but ", "so ", "because "];
const ENOUGH_FOR_SUFFIXES: &[&str] = &[" is enough for this", " is enough for that"];

fn collect_slogan_punchline_evidence(doc: &Document) -> Vec<serde_json::Value> {
    let mut evidence = collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
            match_single_sentence(sentence).map(|matched_text| {
                sentence_evidence(
                    section_index,
                    paragraph_index,
                    sentence_index,
                    &[("matched_text", matched_text), ("sentence", sentence)],
                )
            })
        },
    );

    evidence.extend(collect_adjacent_sentence_evidence(
        doc,
        |sentence, next_sentence, section_index, paragraph_index, sentence_index| {
            match_curriculum_pair(sentence, next_sentence).map(|matched_text| {
                sentence_evidence(
                    section_index,
                    paragraph_index,
                    sentence_index,
                    &[
                        ("matched_text", matched_text),
                        ("sentence", sentence),
                        ("next_sentence", next_sentence),
                    ],
                )
            })
        },
    ));

    evidence
}

fn match_single_sentence(sentence: &str) -> Option<&'static str> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));

    if matches_part_that_sticks(&stripped) {
        return Some("part-that-sticks");
    }
    if matches_part_most_people_miss(&stripped) {
        return Some("part-most-x-miss");
    }
    if matches_it_changes_everything(&stripped) {
        return Some("it-changes-everything");
    }

    None
}

fn match_curriculum_pair(sentence: &str, next_sentence: &str) -> Option<&'static str> {
    let normalized = normalize(sentence);
    let first_unquoted =
        strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));
    let next_normalized = normalize(next_sentence);
    let second_unquoted =
        strip_quoted_segments(strip_leading_prefixes(&next_normalized, LEADING_PREFIXES));
    let first = trim_terminal_punctuation(&first_unquoted);
    let second = trim_terminal_punctuation(&second_unquoted);

    for suffix in ENOUGH_FOR_SUFFIXES {
        let Some(subject) = first.strip_suffix(suffix) else {
            continue;
        };
        if subject.is_empty() || second != format!("{subject} is the curriculum") {
            continue;
        }
        return Some("x-is-enough-x-is-curriculum");
    }

    None
}

fn trim_terminal_punctuation(text: &str) -> &str {
    text.trim_end_matches(|ch: char| matches!(ch, '.' | '!' | '?'))
}

fn matches_part_that_sticks(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(
        tokens.as_slice(),
        [_, _, "is", "the", "part", "that", "sticks"] | [_, "is", "the", "part", "that", "sticks"]
    )
}

fn matches_part_most_people_miss(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(
        tokens.as_slice(),
        ["that", "is", "the", "part", "most", _, "miss"]
            | ["this", "is", "the", "part", "most", _, "miss"]
            | ["it", "is", "the", "part", "most", _, "miss"]
    )
}

fn matches_it_changes_everything(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(tokens.as_slice(), [.., "it", "changes", "everything"]) && tokens.len() <= 8
}

fn tokens(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "slop_10_slogan_punchline_tests/mod.rs"]
mod tests;
