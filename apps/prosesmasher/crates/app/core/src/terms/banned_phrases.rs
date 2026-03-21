//! Banned phrases check — flags configured multi-word phrases found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that none of the configured banned phrases appear in the document.
///
/// Phrases are matched via sliding-window over contiguous words,
/// case-insensitively. Matches in code blocks and list items are ignored.
#[derive(Debug)]
pub struct BannedPhrasesCheck;

impl Check for BannedPhrasesCheck {
    fn id(&self) -> &'static str {
        "banned-phrases"
    }

    fn label(&self) -> &'static str {
        "Banned Phrases"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.banned_phrases.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let phrases = low_expectations::text::build_phrase_list(&config.terms.banned_phrases);
        let _result = suite
            .expect_phrases_absent("banned-phrases", &all_words, &phrases)
            .label("Banned Phrases")
            .checking("AI writing tells");
    }
}

#[cfg(test)]
#[path = "banned_phrases_tests.rs"]
mod tests;
