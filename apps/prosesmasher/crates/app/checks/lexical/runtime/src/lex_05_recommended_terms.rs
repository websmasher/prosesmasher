//! Recommended terms check — at least N from a pool must appear.
//!
//! Unlike required terms (all must appear), recommended terms use a
//! pool where only `min_count` need to be present. Supports optional
//! stem matching via `allow_inflections`.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that at least `min_count` terms from the recommended pool
/// appear in the document.
///
/// When `allow_inflections` is true, matching uses rough stemming
/// (e.g., "screens" matches "screen").
#[derive(Debug)]
pub struct RecommendedTermsCheck;

impl Check for RecommendedTermsCheck {
    fn id(&self) -> &'static str {
        "recommended-terms"
    }

    fn label(&self) -> &'static str {
        "Recommended Terms"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(ref pool) = config.quality.lexical.recommended_terms else {
            return;
        };
        if pool.terms.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let lower_words: Vec<String> = all_words.iter().map(|w| w.to_lowercase()).collect();

        let match_count = if pool.allow_inflections {
            count_stem_matches(&lower_words, &pool.terms)
        } else {
            count_exact_matches(&lower_words, &pool.terms)
        };

        let observed = i64::try_from(match_count).unwrap_or(i64::MAX);
        let min = i64::try_from(pool.min_count).unwrap_or(0);
        let _result = suite
            .expect_value_to_be_at_least("recommended-terms", observed, min)
            .label("Recommended Terms")
            .checking(&format!(
                "at least {} of {} pool terms present",
                pool.min_count,
                pool.terms.len()
            ));
    }
}

/// Count how many pool terms appear as exact matches (case-insensitive).
fn count_exact_matches(lower_words: &[String], terms: &[String]) -> usize {
    terms
        .iter()
        .filter(|term| {
            let lower_term = term.to_lowercase();
            lower_words.contains(&lower_term)
        })
        .count()
}

/// Count how many pool terms appear via stem matching.
///
/// A term matches if any document word starts with the rough stem
/// of the term. E.g., term "screen" with stem "screen" matches
/// "screens", "screening", "screened".
fn count_stem_matches(lower_words: &[String], terms: &[String]) -> usize {
    terms
        .iter()
        .filter(|term| {
            let stem = rough_stem(&term.to_lowercase());
            lower_words.iter().any(|w| w.starts_with(&stem))
        })
        .count()
}

/// Reduce a word to a rough stem by stripping common English suffixes.
fn rough_stem(word: &str) -> String {
    let w = word.trim();
    for suffix in &[
        "ation", "tion", "ment", "ness", "ing", "ies", "ied", "es", "ed", "er", "ly", "s",
    ] {
        if let Some(stripped) = w.strip_suffix(suffix)
            && stripped.len() >= 3
        {
            return stripped.to_owned();
        }
    }
    w.to_owned()
}

#[cfg(test)]
#[path = "lex_05_recommended_terms_tests/mod.rs"]
mod tests;
