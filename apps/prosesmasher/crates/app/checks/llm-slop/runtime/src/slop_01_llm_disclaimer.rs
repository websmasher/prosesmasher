//! LLM-disclaimer check — flags explicit model/disclaimer leakage.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{collect_sentence_evidence, normalize, strip_leading_prefixes};

#[derive(Debug)]
pub struct LlmDisclaimerCheck;

impl Check for LlmDisclaimerCheck {
    fn id(&self) -> &'static str {
        "llm-disclaimer"
    }

    fn label(&self) -> &'static str {
        "LLM Disclaimer"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.llm_disclaimer.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_llm_disclaimer_evidence(doc);
        let _result = suite
            .record_custom_values(
                "llm-disclaimer",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("LLM Disclaimer")
            .checking("explicit LLM identity, knowledge-cutoff, or real-time-access disclaimers");
    }
}

const START_PATTERNS: &[&str] = &[
    "as a language model",
    "as an ai language model",
    "as a language model ai",
    "as a large language model",
    "as of my knowledge cutoff",
    "as of my last knowledge update",
    "my knowledge cutoff date is",
    "i do not have access to real-time",
    "i don't have access to real-time",
    "i do not have real-time",
    "i don't have real-time",
];

const CONTAINS_PATTERNS: &[&str] = &[
    "as i am an ai language model",
    "as i am a language model",
    "i am an ai language model",
    "i am a language model",
    "knowledge cutoff date is",
    "my knowledge is based on data up to",
    "my training data up until",
    "do not have access to real-time information",
    "don't have access to real-time information",
    "do not have access to real-time data",
    "don't have access to real-time data",
];

fn collect_llm_disclaimer_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(doc, |sentence, section_index, paragraph_index, sentence_index| {
        match_disclaimer_pattern(sentence).map(|pattern| {
            json!({
                "section_index": section_index,
                "paragraph_index": paragraph_index,
                "sentence_index": sentence_index,
                "matched_text": pattern,
                "sentence": sentence,
            })
        })
    })
}

fn match_disclaimer_pattern(sentence: &str) -> Option<&'static str> {
    let normalized = normalize(sentence);

    START_PATTERNS
        .iter()
        .find(|pattern| starts_with_disclaimer(&normalized, pattern))
        .copied()
        .or_else(|| {
            CONTAINS_PATTERNS
                .iter()
                .find(|pattern| normalized.contains(**pattern))
                .copied()
        })
}

fn starts_with_disclaimer(normalized: &str, pattern: &str) -> bool {
    let stripped = strip_leading_prefixes(normalized, &["however, ", "but "]);
    stripped == pattern
        || stripped
            .strip_prefix(pattern)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with([' ', ',', '.', ':', ';']))
}

#[cfg(test)]
#[path = "slop_01_llm_disclaimer_tests/mod.rs"]
mod tests;
