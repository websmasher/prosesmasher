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

    if stripped.contains("'s work is famous for a reason")
        || stripped.contains("’s work is famous for a reason")
    {
        return Some(("prestige-frame", "work is famous for a reason"));
    }

    if starts_with_any(
        &stripped,
        &[
            "the evidence is strongest",
            "the strongest recent evidence points",
            "the evidence points",
            "the evidence is not subtle",
        ],
    ) {
        return Some(("evidence-frame", "the evidence"));
    }

    if starts_with_any(
        &stripped,
        &[
            "the research is not mysterious",
            "the research points",
            "the broader research backs",
            "what the research does show is",
            "researchers keep finding",
        ],
    ) {
        return Some(("research-frame", "the research"));
    }

    None
}

fn starts_with_any<'a>(text: &str, candidates: &'a [&'a str]) -> bool {
    candidates
        .iter()
        .any(|candidate| text.starts_with(candidate))
}

#[cfg(test)]
#[path = "slop_12_authority_padding_tests/mod.rs"]
mod tests;
