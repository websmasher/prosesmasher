//! Summative-closer check — flags sections ending with summative phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Detects sections ending with summative phrases
/// (e.g., "And that's what makes this approach so powerful.").
#[derive(Debug)]
pub struct SummativeCloserCheck;

impl Check for SummativeCloserCheck {
    fn id(&self) -> &'static str {
        "summative-closer"
    }

    fn label(&self) -> &'static str {
        "Summative Closer"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.summative_closer.enabled {
            return;
        }
        let summative_patterns = super::resolve_summative_patterns(config);
        if summative_patterns.is_empty() {
            return;
        }
        let evidence = super::collect_section_sentence_evidence(
            doc,
            &summative_patterns,
            super::section_last_sentence,
            super::sentence_starts_with,
        );
        let _result = suite
            .record_custom_values(
                "summative-closer",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": summative_patterns }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Summative Closer")
            .checking("section closers matching summative patterns");
    }
}

#[cfg(test)]
#[path = "summative_closer_tests/mod.rs"]
mod tests;
