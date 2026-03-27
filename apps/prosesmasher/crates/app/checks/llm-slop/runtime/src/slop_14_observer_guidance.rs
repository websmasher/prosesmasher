//! Observer-guidance check — flags empty reader-observation scaffolding in short-form advice.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, sentence_evidence, starts_with_any,
    strip_leading_prefixes, strip_quoted_segments,
};

#[derive(Debug)]
pub struct ObserverGuidanceCheck;

impl Check for ObserverGuidanceCheck {
    fn id(&self) -> &'static str {
        "observer-guidance"
    }

    fn label(&self) -> &'static str {
        "Observer Guidance"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.observer_guidance.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_observer_guidance_evidence(doc);
        let _result = suite
            .record_custom_values(
                "observer-guidance",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Observer Guidance")
            .checking("reader-observation scaffolding lines");
    }
}

const LEADING_PREFIXES: &[&str] = &["and ", "but ", "so "];
const OBSERVER_PATTERNS: &[&str] = &[
    "you see it everywhere",
    "you see it after almost every",
    "you can watch it happen in real time",
    "you can tell the difference quickly",
];
const READER_ADDRESS_PATTERNS: &[&str] = &["if this hits home"];
const STUCK_PATTERNS: &[&str] = &["this is where people get stuck"];
const WHERE_BRIDGE_PATTERNS: &[&str] = &[
    "that is where the confusion slips in",
    "that is where a lot of work gets lost",
    "that is where the guilt starts",
    "that is where the real progress lives",
    "that is where a lot of the misunderstanding begins",
    "that is where culture becomes visible",
];

fn collect_observer_guidance_evidence(doc: &Document) -> Vec<serde_json::Value> {
    collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
            match_observer_guidance(sentence).map(|(pattern_kind, matched_text)| {
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

fn match_observer_guidance(sentence: &str) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));
    let trimmed = stripped.trim_end_matches(['.', '!', '?', ':']);

    if let Some(matched) =
        starts_with_any(trimmed, OBSERVER_PATTERNS).filter(|pattern| trimmed == *pattern)
    {
        return Some(("observer-frame", matched));
    }
    if let Some(matched) = starts_with_any(&stripped, READER_ADDRESS_PATTERNS) {
        return Some(("reader-address", matched));
    }
    if let Some(matched) =
        starts_with_any(trimmed, STUCK_PATTERNS).filter(|pattern| trimmed == *pattern)
    {
        return Some(("stuck-frame", matched));
    }
    starts_with_any(trimmed, WHERE_BRIDGE_PATTERNS)
        .filter(|pattern| trimmed == *pattern)
        .map(|matched| ("where-bridge", matched))
}

#[cfg(test)]
#[path = "slop_14_observer_guidance_tests/mod.rs"]
mod tests;
