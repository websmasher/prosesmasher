//! Humble-bragger check — flags sentences containing humble-bragging phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Detects humble-bragging phrases in prose
/// (e.g., "In my experience", "As someone who has").
#[derive(Debug)]
pub struct HumbleBraggerCheck;

impl Check for HumbleBraggerCheck {
    fn id(&self) -> &'static str {
        "humble-bragger"
    }

    fn label(&self) -> &'static str {
        "Humble Bragger"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.humble_bragger.enabled {
            return;
        }
        let humble_bragger_phrases = super::resolve_humble_bragger_phrases(config);
        if humble_bragger_phrases.is_empty() {
            return;
        }
        let evidence = super::collect_sentence_phrase_evidence(
            doc,
            &humble_bragger_phrases,
            super::sentence_contains,
        );
        let _result = suite
            .record_custom_values(
                "humble-bragger",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": humble_bragger_phrases }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Humble Bragger")
            .checking("sentences containing humble-bragging phrases");
    }
}

#[cfg(test)]
#[path = "humble_bragger_tests/mod.rs"]
mod tests;
