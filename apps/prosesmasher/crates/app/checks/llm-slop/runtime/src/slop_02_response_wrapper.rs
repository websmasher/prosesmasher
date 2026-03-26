//! Response-wrapper check — flags canned assistant capability/limitation wrappers.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, contains_any, normalize, strip_leading_prefixes,
    strip_quoted_segments,
};

#[derive(Debug)]
pub struct ResponseWrapperCheck;

impl Check for ResponseWrapperCheck {
    fn id(&self) -> &'static str {
        "response-wrapper"
    }

    fn label(&self) -> &'static str {
        "Response Wrapper"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.response_wrapper.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_response_wrapper_evidence(doc);
        let _result = suite
            .record_custom_values(
                "response-wrapper",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Response Wrapper")
            .checking("canned assistant capability, limitation, or diagnosis/advice wrapper language");
    }
}

const LEADING_PREFIXES: &[&str] = &["however, ", "but ", "that being said, ", "as such, "];

const GENERAL_INFO_CAPABILITY_PATTERNS: &[&str] = &[
    "i can provide",
    "i can offer",
    "i can share",
    "i can give",
];

const GENERAL_INFO_OBJECTS: &[&str] = &[
    "general information",
    "general guidance",
    "general suggestions",
    "some general suggestions",
    "general advice",
];

const LIMITATION_PATTERNS: &[&str] = &[
    "i cannot provide",
    "i can't provide",
    "i do not provide",
    "i don't provide",
    "i cannot give",
    "i can't give",
    "i do not give",
    "i don't give",
    "i cannot offer",
    "i can't offer",
    "i am not able to provide",
    "i'm not able to provide",
    "i cannot diagnose",
    "i can't diagnose",
    "i do not diagnose",
    "i don't diagnose",
];

const INFORMATION_LIMITATION_OBJECTS: &[&str] = &[
    "information",
    "up-to-date information",
    "most up-to-date information",
    "real-time information",
    "specific information",
];

const ADVICE_LIMITATION_OBJECTS: &[&str] = &[
    "medical advice",
    "specific medical advice",
    "specific advice",
    "personalized advice",
    "medical expertise",
];

const DIAGNOSIS_LIMITATION_OBJECTS: &[&str] = &[
    "provide a diagnosis",
    "diagnosis",
    "diagnose",
    "treatment plan",
];

const ABILITY_LIMITATION_PATTERNS: &[&str] = &[
    "i do not have the ability to provide",
    "i don't have the ability to provide",
    "ability to provide a diagnosis",
];

fn collect_response_wrapper_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(doc, |sentence, section_index, paragraph_index, sentence_index| {
        match_response_wrapper(sentence).map(|(pattern_kind, matched_signal)| {
            json!({
                "section_index": section_index,
                "paragraph_index": paragraph_index,
                "sentence_index": sentence_index,
                "pattern_kind": pattern_kind,
                "matched_signal": matched_signal,
                "sentence": sentence,
            })
        })
    })
}

fn match_response_wrapper(sentence: &str) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_leading_prefixes(&normalized, LEADING_PREFIXES);
    let stripped = strip_quoted_segments(stripped);

    if let Some(signal) = match_information_wrapper(&stripped) {
        return Some(("information-wrapper", signal));
    }
    if let Some(signal) = match_advice_limitation(&stripped) {
        return Some(("advice-limitation", signal));
    }
    match_diagnosis_limitation(&stripped).map(|signal| ("diagnosis-limitation", signal))
}

fn match_information_wrapper(normalized: &str) -> Option<&'static str> {
    if let Some(capability) = contains_any(normalized, GENERAL_INFO_CAPABILITY_PATTERNS) {
        if contains_any(normalized, GENERAL_INFO_OBJECTS).is_some() {
            return Some(capability);
        }
    }

    if let Some(limitation) = contains_any(normalized, LIMITATION_PATTERNS) {
        if contains_any(normalized, INFORMATION_LIMITATION_OBJECTS).is_some() {
            return Some(limitation);
        }
    }

    None
}

fn match_advice_limitation(normalized: &str) -> Option<&'static str> {
    if normalized.contains("i do not have medical expertise")
        || normalized.contains("i don't have medical expertise")
    {
        return Some("medical expertise");
    }

    if let Some(limitation) = contains_any(normalized, LIMITATION_PATTERNS) {
        if contains_any(normalized, ADVICE_LIMITATION_OBJECTS).is_some() {
            return Some(limitation);
        }
    }

    None
}

fn match_diagnosis_limitation(normalized: &str) -> Option<&'static str> {
    if let Some(ability) = contains_any(normalized, ABILITY_LIMITATION_PATTERNS) {
        if contains_any(normalized, DIAGNOSIS_LIMITATION_OBJECTS).is_some() {
            return Some(ability);
        }
    }

    if let Some(limitation) = contains_any(normalized, LIMITATION_PATTERNS) {
        if contains_any(normalized, DIAGNOSIS_LIMITATION_OBJECTS).is_some() {
            return Some(limitation);
        }
    }

    None
}

#[cfg(test)]
#[path = "slop_02_response_wrapper_tests/mod.rs"]
mod tests;
