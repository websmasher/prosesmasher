//! Race terms check — flags configured race-related terms found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that none of the configured race-related terms appear in the document.
///
/// Terms are matched case-insensitively. Matches in code blocks and list
/// items are ignored.
#[derive(Debug)]
pub struct RaceTermsCheck;

impl Check for RaceTermsCheck {
    fn id(&self) -> &'static str {
        "race-terms"
    }

    fn label(&self) -> &'static str {
        "Race Terms"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.race_terms.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let terms = low_expectations::text::build_term_set(&config.terms.race_terms);
        let _result = suite
            .expect_terms_absent("race-terms", &all_words, &terms)
            .label("Race Terms")
            .checking("inclusive language");
    }
}

#[cfg(test)]
#[path = "race_terms_tests.rs"]
mod tests;
