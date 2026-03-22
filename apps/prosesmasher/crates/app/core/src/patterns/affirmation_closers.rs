//! Affirmation-closers check — flags sections ending with affirmation phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Detects sections that end with affirmation phrases
/// (e.g., "...and that's the key.").
#[derive(Debug)]
pub struct AffirmationClosersCheck;

impl Check for AffirmationClosersCheck {
    fn id(&self) -> &'static str {
        "affirmation-closers"
    }

    fn label(&self) -> &'static str {
        "Affirmation Closers"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.affirmation_closers.is_empty() {
            return;
        }
        let evidence = super::collect_section_sentence_evidence(
            doc,
            &config.terms.affirmation_closers,
            super::section_last_sentence,
            super::sentence_ends_with,
        );
        let _result = suite
            .record_custom_values(
                "affirmation-closers",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": config.terms.affirmation_closers }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Affirmation Closers")
            .checking("section closers matching affirmation patterns");
    }
}

#[cfg(test)]
#[path = "affirmation_closers_tests.rs"]
mod tests;
