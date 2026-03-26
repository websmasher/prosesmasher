//! LLM-openers check — flags section openers that match known LLM patterns.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Detects sections that open with common LLM phrases
/// (e.g., "The interesting part is that...").
#[derive(Debug)]
pub struct LlmOpenersCheck;

impl Check for LlmOpenersCheck {
    fn id(&self) -> &'static str {
        "llm-openers"
    }

    fn label(&self) -> &'static str {
        "LLM Openers"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.llm_openers.enabled {
            return;
        }
        let llm_openers = super::resolve_llm_openers(config);
        if llm_openers.is_empty() {
            return;
        }
        let evidence = super::collect_section_sentence_evidence(
            doc,
            &llm_openers,
            super::section_first_sentence,
            super::sentence_starts_with,
        );
        let _result = suite
            .record_custom_values(
                "llm-openers",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": llm_openers }),
                json!(evidence.len()),
                &evidence,
            )
            .label("LLM Openers")
            .checking("section openers matching LLM patterns");
    }
}

#[cfg(test)]
#[path = "heur_10_llm_openers_tests/mod.rs"]
mod tests;
