//! Gendered terms check — flags configured gendered terms found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that none of the configured gendered terms appear in the document.
///
/// Terms are matched case-insensitively. Matches in code blocks and list
/// items are ignored.
#[derive(Debug)]
pub struct GenderedTermsCheck;

impl Check for GenderedTermsCheck {
    fn id(&self) -> &'static str {
        "gendered-terms"
    }

    fn label(&self) -> &'static str {
        "Gendered Terms"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.gendered_terms.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let terms = low_expectations::text::build_term_set(&config.terms.gendered_terms);
        let _result = suite
            .expect_terms_absent("gendered-terms", &all_words, &terms)
            .label("Gendered Terms")
            .checking("inclusive language");
    }
}

#[cfg(test)]
#[path = "gendered_terms_tests.rs"]
mod tests;
