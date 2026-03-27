//! Authority-padding check — flags repeated vague authority/credibility scaffolding.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, sentence_evidence, strip_leading_prefixes,
    strip_quoted_segments,
};

#[derive(Debug)]
pub struct AuthorityPaddingCheck;

impl Check for AuthorityPaddingCheck {
    fn id(&self) -> &'static str {
        "authority-padding"
    }

    fn label(&self) -> &'static str {
        "Authority Padding"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.authority_padding.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config.quality.heuristics.authority_padding.max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_authority_padding_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "authority-padding",
                observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("Authority Padding")
            .checking("repeated vague authority or credibility scaffolding per document");
    }
}

const LEADING_PREFIXES: &[&str] = &["however, ", "but ", "and ", "so "];
const EVIDENCE_SUBJECTS: &[&str] = &["the evidence"];
const RESEARCH_SUBJECTS: &[&str] = &["the research", "what the research"];
const RESEARCHER_SUBJECTS: &[&str] = &["researchers"];
const EVIDENCE_PREDICATES: &[&str] = &[
    "is strongest",
    "is not subtle",
    "points",
];
const RESEARCH_PREDICATES: &[&str] = &[
    "is not mysterious",
    "points",
    "backs",
    "does show is",
];
const RESEARCHER_PREDICATES: &[&str] = &["keep finding"];
const PRESTIGE_SUFFIXES: &[&str] = &["'s work is famous for a reason", "’s work is famous for a reason"];

fn collect_authority_padding_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
            match_authority_padding(sentence).map(|(pattern_kind, matched_text)| {
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

fn match_authority_padding(sentence: &str) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));

    if ends_with_any(&stripped, PRESTIGE_SUFFIXES) {
        return Some(("prestige-frame", "work is famous for a reason"));
    }

    if matches_subject_predicate_family(&stripped, EVIDENCE_SUBJECTS, EVIDENCE_PREDICATES)
        || stripped.starts_with("the strongest recent evidence points")
    {
        return Some(("evidence-frame", "the evidence"));
    }

    if matches_subject_predicate_family(&stripped, RESEARCH_SUBJECTS, RESEARCH_PREDICATES)
        || stripped.starts_with("the broader research backs")
        || matches_subject_predicate_family(
            &stripped,
            RESEARCHER_SUBJECTS,
            RESEARCHER_PREDICATES,
        )
    {
        return Some(("research-frame", "the research"));
    }

    None
}

fn matches_subject_predicate_family(text: &str, subjects: &[&str], predicates: &[&str]) -> bool {
    subjects.iter().any(|subject| {
        predicates.iter().any(|predicate| {
            let prefix = format!("{subject} {predicate}");
            text.starts_with(&prefix)
        })
    })
}

fn ends_with_any(text: &str, suffixes: &[&str]) -> bool {
    suffixes.iter().any(|suffix| text.contains(suffix))
}

#[cfg(test)]
#[path = "slop_12_authority_padding_tests/mod.rs"]
mod tests;
