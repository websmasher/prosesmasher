//! Jargon-faker check — flags sentences containing fake tech jargon phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Detects fake jargon phrases in prose
/// (e.g., "debugging your", "optimizing for", "iterating on your").
#[derive(Debug)]
pub struct JargonFakerCheck;

impl Check for JargonFakerCheck {
    fn id(&self) -> &'static str {
        "jargon-faker"
    }

    fn label(&self) -> &'static str {
        "Jargon Faker"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.jargon_faker.enabled {
            return;
        }
        let jargon_faker_phrases = super::resolve_jargon_faker_phrases(config);
        if jargon_faker_phrases.is_empty() {
            return;
        }
        let evidence = super::collect_sentence_phrase_evidence(
            doc,
            &jargon_faker_phrases,
            super::sentence_contains,
        );
        let _result = suite
            .record_custom_values(
                "jargon-faker",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": jargon_faker_phrases }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Jargon Faker")
            .checking("sentences containing fake tech jargon");
    }
}

#[cfg(test)]
#[path = "jargon_faker_tests.rs"]
mod tests;
