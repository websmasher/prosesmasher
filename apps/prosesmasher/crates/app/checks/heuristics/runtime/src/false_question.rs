//! False-question check — flags rhetorical questions matching known patterns.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Detects false/rhetorical questions at section ends that match
/// known pattern phrases (e.g., "And isn't that what we all want?").
#[derive(Debug)]
pub struct FalseQuestionCheck;

impl Check for FalseQuestionCheck {
    fn id(&self) -> &'static str {
        "false-question"
    }

    fn label(&self) -> &'static str {
        "False Question"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.false_question.enabled {
            return;
        }
        let false_question_patterns = super::resolve_false_question_patterns(config);
        if false_question_patterns.is_empty() {
            return;
        }
        let evidence = super::collect_section_sentence_evidence(
            doc,
            &false_question_patterns,
            super::section_last_sentence,
            false_question_matcher,
        );
        let _result = suite
            .record_custom_values(
                "false-question",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": false_question_patterns }),
                json!(evidence.len()),
                &evidence,
            )
            .label("False Question")
            .checking("rhetorical questions matching known patterns");
    }
}

fn false_question_matcher(sentence: &str, phrase: &str) -> bool {
    sentence.ends_with('?') && sentence.contains(phrase)
}

#[cfg(test)]
#[path = "false_question_tests/mod.rs"]
mod tests;
