//! Sentence case check — validates that headings use sentence case, not Title Case.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Checks that headings use sentence case. This is a style heuristic that targets
/// heading nodes; it is not a document-structure rule.
#[derive(Debug)]
pub struct SentenceCaseCheck;

impl Check for SentenceCaseCheck {
    fn id(&self) -> &'static str {
        "sentence-case"
    }

    fn label(&self) -> &'static str {
        "Sentence Case"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En, Locale::Es, Locale::Pt, Locale::Fr, Locale::Id])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.sentence_case.enabled {
            return;
        }

        for (section_index, section) in doc.sections.iter().enumerate() {
            let Some(heading) = &section.heading else {
                continue;
            };

            let words: Vec<&str> = heading.text.split_whitespace().collect();

            // Skip first word (always capitalized). Skip all-caps words (acronyms).
            let capitalized_non_first: usize = words
                .iter()
                .skip(1)
                .filter(|w| is_title_cased(w))
                .count();

            let is_title_case = capitalized_non_first >= 3;

            let col = format!("sentence-case-{}", heading.text);
            let evidence = if is_title_case {
                vec![json!({
                    "section_index": section_index,
                    "heading_level": heading.level,
                    "heading_text": heading.text,
                    "capitalized_non_first_words": capitalized_non_first,
                    "sentence_case_expected": true,
                })]
            } else {
                Vec::new()
            };
            let _result = suite
                .record_custom_values(
                    &col,
                    !is_title_case,
                    json!({
                        "rule": "sentence case",
                        "max_capitalized_non_first_words": 2,
                    }),
                    json!(heading.text),
                    &evidence,
                )
                .label("Sentence Case")
                .checking(&format!("heading: \"{}\"", heading.text));
        }
    }
}

/// Returns true if a word starts with an uppercase letter and is NOT all-caps (acronym).
fn is_title_cased(word: &str) -> bool {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_uppercase() {
        return false;
    }

    // If all remaining chars are uppercase (or non-alphabetic), it's an acronym — skip it
    chars.any(char::is_lowercase)
}

#[cfg(test)]
#[path = "sentence_case_tests.rs"]
mod tests;
