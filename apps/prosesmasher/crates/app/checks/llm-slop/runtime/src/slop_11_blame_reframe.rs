//! Blame-reframe check — flags short moralized contrast lines that recast blame as growth or diagnosis.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, sentence_evidence, strip_leading_prefixes,
    strip_quoted_segments,
};

#[derive(Debug)]
pub struct BlameReframeCheck;

impl Check for BlameReframeCheck {
    fn id(&self) -> &'static str {
        "blame-reframe"
    }

    fn label(&self) -> &'static str {
        "Blame Reframe"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.blame_reframe.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_sentence_evidence(
            doc,
            |sentence, section_index, paragraph_index, sentence_index| {
                match_blame_reframe(sentence).map(|matched_text| {
                    sentence_evidence(
                        section_index,
                        paragraph_index,
                        sentence_index,
                        &[("matched_text", matched_text), ("sentence", sentence)],
                    )
                })
            },
        );

        let _result = suite
            .record_custom_values(
                "blame-reframe",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Blame Reframe")
            .checking("short blame-to-growth contrast lines");
    }
}

const LEADING_PREFIXES: &[&str] = &["and ", "but ", "so "];
const BLAME_NOUNS: &[&str] = &["malice", "shame", "guilt", "blame", "humiliation"];
const GROWTH_NOUNS: &[&str] = &["development", "skill-building", "learning", "practice"];
const CAUSAL_CUES: &[&str] = &["comes from ", "come from ", "stems from ", "stem from "];

fn match_blame_reframe(sentence: &str) -> Option<&'static str> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));

    if matches_source_not_blame(&stripped) {
        return Some("source-not-blame");
    }
    if matches_growth_instead_of_blame(&stripped) {
        return Some("growth-instead-of-blame");
    }
    None
}

fn matches_source_not_blame(text: &str) -> bool {
    if !BLAME_NOUNS.iter().any(|noun| {
        text.ends_with(&format!("not {noun}.")) || text.ends_with(&format!("not {noun}"))
    }) {
        return false;
    }

    CAUSAL_CUES.iter().any(|cue| {
        let Some(prefix) = text.split(cue).next() else {
            return false;
        };
        let Some(after) = text.split(cue).nth(1) else {
            return false;
        };
        prefix.split_whitespace().count() <= 4
            && after
                .split(", not ")
                .next()
                .is_some_and(|segment| GROWTH_NOUNS.iter().any(|noun| segment.contains(noun)))
    })
}

fn matches_growth_instead_of_blame(text: &str) -> bool {
    if !text.contains(" instead of ") || !text.contains(" as ") {
        return false;
    }

    let has_growth = GROWTH_NOUNS
        .iter()
        .any(|noun| text.contains(&format!("as {noun}")));
    let has_blame = BLAME_NOUNS
        .iter()
        .any(|noun| text.contains(&format!("instead of {noun}")));

    has_growth && has_blame
}

#[cfg(test)]
#[path = "slop_11_blame_reframe_tests/mod.rs"]
mod tests;
