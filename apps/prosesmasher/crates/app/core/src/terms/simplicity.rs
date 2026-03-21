//! Simplicity check — flags complex words that have simpler alternatives.

use std::collections::BTreeSet;

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that none of the configured complex words appear in the document.
///
/// Uses the `simplicity_pairs` config to build a set of complex words.
/// Words are matched case-insensitively. Matches in code blocks and list
/// items are ignored.
#[derive(Debug)]
pub struct SimplicityCheck;

impl Check for SimplicityCheck {
    fn id(&self) -> &'static str {
        "simplicity"
    }

    fn label(&self) -> &'static str {
        "Simplicity"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.simplicity_pairs.is_empty() {
            return;
        }

        let complex_set: BTreeSet<String> = config
            .terms
            .simplicity_pairs
            .iter()
            .map(|pair| pair.complex.to_lowercase())
            .collect();

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let _result = suite
            .expect_terms_absent("simplicity", &all_words, &complex_set)
            .label("Simplicity")
            .checking("plain language");
    }
}

#[cfg(test)]
#[path = "simplicity_tests.rs"]
mod tests;
