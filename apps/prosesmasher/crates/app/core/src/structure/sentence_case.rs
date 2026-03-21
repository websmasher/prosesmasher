//! Sentence case check — validates that headings use sentence case, not Title Case.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that headings use sentence case. Flags if 3+ non-first words are capitalized
/// (excluding all-caps acronyms).
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

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        for section in &doc.sections {
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
            let errors: Vec<String> = if is_title_case {
                vec![format!(
                    "title case detected: \"{}\" ({capitalized_non_first} capitalized words after first)",
                    heading.text
                )]
            } else {
                vec![]
            };
            let _result = suite
                .record_custom(
                    &col,
                    !is_title_case,
                    "sentence case heading",
                    &heading.text,
                    &errors,
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
