//! Contrastive-aphorism check — flags short sloganized contrasts and paired moral curricula lines.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;
use crate::support::{
    collect_adjacent_sentence_evidence, collect_sentence_evidence, normalize, sentence_evidence,
    strip_leading_prefixes, strip_quoted_segments,
};

#[derive(Debug)]
pub struct ContrastiveAphorismCheck;

impl Check for ContrastiveAphorismCheck {
    fn id(&self) -> &'static str {
        "contrastive-aphorism"
    }

    fn label(&self) -> &'static str {
        "Contrastive Aphorism"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.contrastive_aphorism.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_contrastive_aphorism_evidence(doc);
        let _result = suite
            .record_custom_values(
                "contrastive-aphorism",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Contrastive Aphorism")
            .checking("short contrastive aphorism lines");
    }
}

const LEADING_PREFIXES: &[&str] = &["and ", "but ", "so ", "because "];
const ENOUGH_FOR_SUFFIXES: &[&str] = &[" is enough for this", " is enough for that"];
const ABSTRACT_CONTRAST_NOUNS: &[&str] = &["revelations", "vibe", "virtues"];
const HUMAN_PLURAL_SUBJECTS: &[&str] = &["kids", "children", "people"];

fn collect_contrastive_aphorism_evidence(doc: &Document) -> Vec<serde_json::Value> {
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
    if matches_imperative_contrast_aphorism(&stripped) {
        return Some("imperative-contrast-aphorism");
    }
    if matches_reps_not_revelations_shape(&stripped) {
        return Some("reps-not-revelations");
    }
    if matches_treating_like_not_virtues_shape(&stripped) {
        return Some("treating-like-not-virtues");
    }
    if matches_watch_for_x_not_y(&stripped) {
        return Some("watch-for-pattern-not-week");
    }
    if matches_like_a_problem_not_a_problem(&stripped) {
        return Some("like-a-problem-not-a-problem");
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

fn matches_imperative_contrast_aphorism(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(
        tokens.as_slice(),
        ["bring", article_a, _, "not", article_b, contrast]
            if is_article(article_a)
                && is_article(article_b)
                && ABSTRACT_CONTRAST_NOUNS.contains(contrast)
    )
}

fn matches_reps_not_revelations_shape(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(
        tokens.as_slice(),
        [subject, "get", _, "in", _, "not", contrast]
            if HUMAN_PLURAL_SUBJECTS.contains(subject)
                && ABSTRACT_CONTRAST_NOUNS.contains(contrast)
    )
}

fn matches_treating_like_not_virtues_shape(text: &str) -> bool {
    let tokens = tokens(text);
    tokens.len() <= 10
        && tokens.starts_with(&["mostly", "by", "treating"])
        && tokens.contains(&"like")
        && matches!(tokens.last(), Some(contrast) if ABSTRACT_CONTRAST_NOUNS.contains(contrast))
        && tokens.iter().any(|token| *token == "not")
}

fn matches_watch_for_x_not_y(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(
        tokens.as_slice(),
        ["watch", "for", article, "pattern", "not", "one", "bad", _]
            if is_article(article)
    )
}

fn matches_like_a_problem_not_a_problem(text: &str) -> bool {
    let tokens = tokens(text);
    matches!(
        tokens.as_slice(),
        [_, _, _, "like", article_a, _, "problem", "not", article_b, _, "problem"]
            if is_article(article_a) && is_article(article_b)
    )
}

fn is_article(token: &str) -> bool {
    matches!(token, "a" | "an" | "the")
}

fn tokens(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "slop_10_contrastive_aphorism_tests/mod.rs"]
mod tests;
