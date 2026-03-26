//! Universalizing-claims check — flags repeated broad human-generalization framing.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, sentence_evidence, strip_leading_prefixes,
    strip_quoted_segments,
};

#[derive(Debug)]
pub struct UniversalizingClaimsCheck;

impl Check for UniversalizingClaimsCheck {
    fn id(&self) -> &'static str {
        "universalizing-claims"
    }

    fn label(&self) -> &'static str {
        "Universalizing Claims"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.universalizing_claims.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config.quality.heuristics.universalizing_claims.max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_universalizing_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "universalizing-claims",
                observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("Universalizing Claims")
            .checking("repeated broad human-generalization claims per document");
    }
}

const LEADING_PREFIXES: &[&str] = &[
    "however, ",
    "but ",
    "and ",
    "so ",
    "ultimately, ",
    "after all, ",
    "in the end, ",
];

const SUBJECT_PATTERNS: &[&[&str]] = &[
    &["everyone"],
    &["everybody"],
    &["we", "all"],
    &["most", "people"],
    &["most", "of", "us"],
    &["for", "most", "people"],
    &["no", "one"],
    &["nobody"],
];

const DESIRE_VERBS: &[&str] = &[
    "want",
    "wants",
    "need",
    "needs",
    "hope",
    "hopes",
    "deserve",
    "deserves",
    "crave",
    "craves",
];

const CERTAINTY_VERBS: &[&str] = &["know", "knows"];

fn collect_universalizing_evidence(doc: &Document) -> Vec<Value> {
    collect_sentence_evidence(doc, |sentence, section_index, paragraph_index, sentence_index| {
        match_universalizing_sentence(sentence).map(|(pattern_kind, matched_text)| {
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

fn match_universalizing_sentence(sentence: &str) -> Option<(&'static str, String)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));
    let tokens = token_words(&stripped);
    let (subject_index, subject) = match_subject(&tokens)?;
    if subject_index != 0 {
        return None;
    }

    let verb_window = tokens
        .iter()
        .skip(subject.len())
        .take(4)
        .copied()
        .collect::<Vec<_>>();

    if let Some(verb) = verb_window
        .iter()
        .copied()
        .find(|candidate| DESIRE_VERBS.contains(candidate))
    {
        return Some(("collective-desire", format!("{} {verb}", subject.join(" "))));
    }

    verb_window
        .iter()
        .copied()
        .find(|candidate| CERTAINTY_VERBS.contains(candidate))
        .map(|verb| ("collective-certainty", format!("{} {verb}", subject.join(" "))))
}

fn match_subject<'a>(tokens: &'a [&'a str]) -> Option<(usize, &'static [&'static str])> {
    SUBJECT_PATTERNS.iter().find_map(|subject| {
        tokens
            .windows(subject.len())
            .position(|window| window == *subject)
            .map(|index| (index, *subject))
    })
}

fn token_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "slop_07_universalizing_claims_tests/mod.rs"]
mod tests;
