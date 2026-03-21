//! Forbidden terms check — flags configured forbidden terms found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that none of the configured forbidden terms appear in the document.
///
/// Terms are matched case-insensitively. Matches in code blocks and list
/// items are ignored.
#[derive(Debug)]
pub struct ForbiddenTermsCheck;

impl Check for ForbiddenTermsCheck {
    fn id(&self) -> &'static str {
        "forbidden-terms"
    }

    fn label(&self) -> &'static str {
        "Forbidden Terms"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.forbidden_terms.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let terms = low_expectations::text::build_term_set(&config.terms.forbidden_terms);
        let _result = suite
            .expect_terms_absent("forbidden-terms", &all_words, &terms)
            .label("Forbidden Terms")
            .checking("style compliance");
    }
}

#[cfg(test)]
#[path = "forbidden_terms_tests.rs"]
mod tests;
