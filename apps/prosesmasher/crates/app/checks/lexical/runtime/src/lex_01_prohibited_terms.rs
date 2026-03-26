//! Prohibited terms check — flags configured prohibited terms found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Checks that none of the configured prohibited terms appear in the document.
///
/// Terms may be single words or multi-word phrases. Matching is
/// case-insensitive. Matches in code blocks and list items are ignored.
#[derive(Debug)]
pub struct ProhibitedTermsCheck;

impl Check for ProhibitedTermsCheck {
    fn id(&self) -> &'static str {
        "prohibited-terms"
    }

    fn label(&self) -> &'static str {
        "Prohibited Terms"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let prohibited_terms = super::resolve_prohibited_terms(config);
        if prohibited_terms.is_empty() {
            return;
        }

        let evidence = super::collect_prohibited_term_evidence(doc, &prohibited_terms);
        let observed = super::unique_matched_texts(&evidence);
        let _result = suite
            .record_custom_values(
                "prohibited-terms",
                evidence.is_empty(),
                json!({ "absent": prohibited_terms }),
                json!(observed),
                &evidence,
            )
            .label("Prohibited Terms")
            .checking("prohibited terms in prose");
    }
}

#[cfg(test)]
#[path = "lex_01_prohibited_terms_tests/mod.rs"]
mod tests;
