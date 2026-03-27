//! Response-wrapper check — flags canned assistant capability/limitation wrappers.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, strip_leading_prefixes, strip_quoted_segments,
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
            .checking(
                "canned assistant capability, limitation, or diagnosis/advice wrapper language",
            );
    }
}

const LEADING_PREFIXES: &[&str] = &["however, ", "but ", "that being said, ", "as such, "];
const FIRST_PERSON_SUBJECTS: &[&str] = &["i"];
const CAPABILITY_AUXILIARIES: &[&str] = &["can"];
const LIMITATION_AUXILIARIES: &[&str] = &["cannot", "can't", "do not", "don't"];
const ABILITY_LIMITATION_PREFIXES: &[&str] =
    &["do not have the ability to", "don't have the ability to"];
const CAPABILITY_ACTIONS: &[&str] = &["provide", "offer", "share", "give"];
const INFORMATION_OBJECTS: &[&str] = &[
    "general information",
    "general guidance",
    "general suggestions",
    "some general suggestions",
    "general advice",
];
const LIMITATION_ACTIONS: &[&str] = &["provide", "give", "offer"];
const DIAGNOSIS_ACTIONS: &[&str] = &["diagnose"];
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
];
const MEDICAL_EXPERTISE_OBJECTS: &[&str] = &["medical expertise"];
const DIAGNOSIS_LIMITATION_OBJECTS: &[&str] = &[
    "provide a diagnosis",
    "diagnosis",
    "diagnose",
    "treatment plan",
];

fn collect_response_wrapper_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
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
        },
    )
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
    if matches_subject_aux_action_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        CAPABILITY_AUXILIARIES,
        CAPABILITY_ACTIONS,
        INFORMATION_OBJECTS,
    ) {
        return Some("capability+info-object");
    }

    if matches_subject_aux_action_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        LIMITATION_AUXILIARIES,
        LIMITATION_ACTIONS,
        INFORMATION_LIMITATION_OBJECTS,
    ) {
        return Some("limitation+info-object");
    }

    None
}

fn match_advice_limitation(normalized: &str) -> Option<&'static str> {
    if matches_subject_aux_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        LIMITATION_AUXILIARIES,
        MEDICAL_EXPERTISE_OBJECTS,
    ) {
        return Some("limitation+expertise-object");
    }

    if matches_subject_aux_action_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        LIMITATION_AUXILIARIES,
        LIMITATION_ACTIONS,
        ADVICE_LIMITATION_OBJECTS,
    ) {
        return Some("limitation+advice-object");
    }

    None
}

fn match_diagnosis_limitation(normalized: &str) -> Option<&'static str> {
    if matches_subject_prefix_action_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        ABILITY_LIMITATION_PREFIXES,
        &["provide"],
        DIAGNOSIS_LIMITATION_OBJECTS,
    ) {
        return Some("ability-limitation+diagnosis-object");
    }

    if matches_subject_aux_action_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        LIMITATION_AUXILIARIES,
        LIMITATION_ACTIONS,
        DIAGNOSIS_LIMITATION_OBJECTS,
    ) || matches_subject_aux_action_objects(
        normalized,
        FIRST_PERSON_SUBJECTS,
        LIMITATION_AUXILIARIES,
        DIAGNOSIS_ACTIONS,
        DIAGNOSIS_LIMITATION_OBJECTS,
    ) {
        return Some("limitation+diagnosis-object");
    }

    None
}

fn matches_subject_aux_action_objects(
    text: &str,
    subjects: &[&str],
    auxiliaries: &[&str],
    actions: &[&str],
    objects: &[&str],
) -> bool {
    subjects.iter().any(|subject| {
        auxiliaries.iter().any(|auxiliary| {
            actions.iter().any(|action| {
                let prefix = format!("{subject} {auxiliary} {action} ");
                text.starts_with(&prefix) && objects.iter().any(|object| text.contains(object))
            })
        })
    })
}

fn matches_subject_aux_objects(
    text: &str,
    subjects: &[&str],
    auxiliaries: &[&str],
    objects: &[&str],
) -> bool {
    subjects.iter().any(|subject| {
        auxiliaries.iter().any(|auxiliary| {
            let prefix = format!("{subject} {auxiliary} ");
            text.starts_with(&prefix) && objects.iter().any(|object| text.contains(object))
        })
    })
}

fn matches_subject_prefix_action_objects(
    text: &str,
    subjects: &[&str],
    prefixes: &[&str],
    actions: &[&str],
    objects: &[&str],
) -> bool {
    subjects.iter().any(|subject| {
        prefixes.iter().any(|prefix| {
            actions.iter().any(|action| {
                let full_prefix = format!("{subject} {prefix} {action} ");
                text.starts_with(&full_prefix) && objects.iter().any(|object| text.contains(object))
            })
        })
    })
}

#[cfg(test)]
#[path = "slop_02_response_wrapper_tests/mod.rs"]
mod tests;
