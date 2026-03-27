//! Lesson-framing check — flags empty lesson/fix wrappers common in short-form AI advice.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, sentence_evidence, starts_with_any,
    strip_leading_prefixes, strip_quoted_segments,
};

#[derive(Debug)]
pub struct LessonFramingCheck;

impl Check for LessonFramingCheck {
    fn id(&self) -> &'static str {
        "lesson-framing"
    }

    fn label(&self) -> &'static str {
        "Lesson Framing"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.lesson_framing.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_lesson_framing_evidence(doc);
        let _result = suite
            .record_custom_values(
                "lesson-framing",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Lesson Framing")
            .checking("empty lesson/fix wrappers in advice prose");
    }
}

const LEADING_PREFIXES: &[&str] = &["and ", "but ", "so "];
const LESSON_SUMMARY_PATTERNS: &[&str] = &[
    "the biggest lesson was simple",
    "the practical lesson for me was simple",
    "the practical lesson was simple",
];
const FIX_PREFIX: &str = "the fix is ";
const FIX_CUES: &[&str] = &[
    "plain",
    "boring",
    "not heroic",
    "usually smaller than people want",
];

fn collect_lesson_framing_evidence(doc: &Document) -> Vec<serde_json::Value> {
    collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
            match_lesson_framing(sentence).map(|(pattern_kind, matched_text)| {
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

fn match_lesson_framing(sentence: &str) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));

    if let Some(matched) = starts_with_any(&stripped, LESSON_SUMMARY_PATTERNS) {
        return Some(("lesson-summary", matched));
    }
    if stripped.starts_with(FIX_PREFIX) {
        return FIX_CUES
            .iter()
            .find(|cue| stripped.contains(**cue))
            .copied()
            .map(|matched| ("fix-wrapper", matched));
    }
    None
}

#[cfg(test)]
#[path = "slop_13_lesson_framing_tests/mod.rs"]
mod tests;
