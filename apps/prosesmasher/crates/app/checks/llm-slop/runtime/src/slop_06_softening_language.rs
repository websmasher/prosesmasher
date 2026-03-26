//! Softening-language check — flags repeated stacked low-commitment phrasing.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{collect_sentence_evidence, normalize, sentence_evidence, strip_quoted_segments};

#[derive(Debug)]
pub struct SofteningLanguageCheck;

impl Check for SofteningLanguageCheck {
    fn id(&self) -> &'static str {
        "softening-language"
    }

    fn label(&self) -> &'static str {
        "Softening Language"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.softening_language.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config.quality.heuristics.softening_language.max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_softening_language_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "softening-language",
                observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("Softening Language")
            .checking("repeated stacked low-commitment phrasing per document");
    }
}

const MODAL_PATTERNS: &[&str] = &[" may ", " might ", " could "];
const QUALIFIER_PATTERNS: &[&str] = &[
    " generally ",
    " typically ",
    " commonly ",
    " often ",
    " potentially ",
    " relatively ",
    " usually ",
];
const VARIABILITY_PATTERNS: &[&str] = &[
    "in some individuals",
    "in some people",
    "in some children",
    "in some cases",
    "from person to person",
    "may not necessarily",
    "not necessarily",
    "not all individuals",
    "not all people",
];
const REPORTING_PATTERNS: &[&str] = &[
    "is believed",
    "are believed",
    "generally considered",
    "has been reported",
    "have been reported",
    "research suggests",
    "studies suggest",
    "studies have suggested",
    "more research is needed",
];
const QUANTIFIER_LEADS: &[&str] = &["some", "certain", "various", "many"];
const QUANTIFIER_TARGETS: &[&str] = &[
    "people",
    "individuals",
    "children",
    "experts",
    "research",
    "studies",
    "foods",
    "factors",
    "types",
    "cases",
    "patients",
];

fn collect_softening_language_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(doc, |sentence, section_index, paragraph_index, sentence_index| {
        match_softening_sentence(sentence).map(|(pattern_kind, matched_text)| {
            sentence_evidence(
                section_index,
                paragraph_index,
                sentence_index,
                &[
                    ("pattern_kind", pattern_kind),
                    ("matched_text", &matched_text),
                    ("sentence", sentence),
                ],
            )
        })
    })
}

fn match_softening_sentence(sentence: &str) -> Option<(&'static str, String)> {
    let normalized = format!(" {} ", strip_quoted_segments(&normalize(sentence)));

    let modal = contains_pattern(&normalized, MODAL_PATTERNS).map(|matched| ("modal", matched));
    let qualifier =
        contains_pattern(&normalized, QUALIFIER_PATTERNS).map(|matched| ("qualifier", matched));
    let variability =
        contains_pattern(&normalized, VARIABILITY_PATTERNS).map(|matched| ("variability", matched));
    let reporting =
        contains_pattern(&normalized, REPORTING_PATTERNS).map(|matched| ("reporting", matched));
    let quantifier = find_quantifier_pair(&normalized).map(|matched| ("quantifier", matched));

    let signal_count = usize::from(modal.is_some())
        + usize::from(qualifier.is_some())
        + usize::from(variability.is_some())
        + usize::from(reporting.is_some())
        + usize::from(quantifier.is_some());

    if signal_count < 2 {
        return None;
    }

    if let (Some((_, modal_match)), Some((_, variability_match))) = (modal, variability) {
        return Some((
            "variability-softening",
            format!("{modal_match} + {variability_match}"),
        ));
    }

    if let (Some((_, modal_match)), Some((_, qualifier_match))) = (modal, qualifier) {
        return Some((
            "hedged-claim",
            format!("{modal_match} + {qualifier_match}"),
        ));
    }

    if let (Some((_, modal_match)), Some((_, quantifier_match))) = (modal, quantifier.as_ref()) {
        return Some((
            "quantified-softening",
            format!("{modal_match} + {quantifier_match}"),
        ));
    }

    if let (Some((_, reporting_match)), Some((_, modal_match))) = (reporting, modal) {
        return Some((
            "tentative-reporting",
            format!("{reporting_match} + {modal_match}"),
        ));
    }

    if let (Some((_, reporting_match)), Some((_, quantifier_match))) =
        (reporting, quantifier.as_ref())
    {
        return Some((
            "tentative-reporting",
            format!("{reporting_match} + {quantifier_match}"),
        ));
    }

    if let (Some((_, variability_match)), Some((_, quantifier_match))) =
        (variability, quantifier.as_ref())
    {
        return Some((
            "variability-softening",
            format!("{variability_match} + {quantifier_match}"),
        ));
    }

    None
}

fn contains_pattern<'a>(text: &str, patterns: &'a [&str]) -> Option<&'a str> {
    patterns
        .iter()
        .find(|pattern| text.contains(**pattern))
        .map(|pattern| pattern.trim())
}

fn find_quantifier_pair(text: &str) -> Option<String> {
    let tokens = token_words(text);
    for window in tokens.windows(3) {
        let lead = window.first().copied().unwrap_or_default();
        if !QUANTIFIER_LEADS.contains(&lead) {
            continue;
        }
        if let Some(target) = window.iter().skip(1).find(|token| QUANTIFIER_TARGETS.contains(token))
        {
            return Some(format!("{lead} {target}"));
        }
    }
    None
}

fn token_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "slop_06_softening_language_tests/mod.rs"]
mod tests;
