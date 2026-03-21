//! Banned words check — flags configured banned words found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that none of the configured banned words appear in the document.
///
/// Words are matched case-insensitively against the `banned_words` list
/// from the config. Matches in code blocks and list items are ignored.
#[derive(Debug)]
pub struct BannedWordsCheck;

impl Check for BannedWordsCheck {
    fn id(&self) -> &'static str {
        "banned-words"
    }

    fn label(&self) -> &'static str {
        "Banned Words"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.banned_words.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| collect_paragraph_words(b))
            .collect();

        let banned = low_expectations::text::build_term_set(&config.terms.banned_words);
        let _result = suite
            .expect_terms_absent("banned-words", &all_words, &banned)
            .label("Banned Words")
            .checking("AI writing tells");
    }
}

/// Collect word texts from paragraphs, including inside block quotes.
fn collect_paragraph_words(block: &Block) -> Vec<&str> {
    match block {
        Block::Paragraph(p) => p
            .sentences
            .iter()
            .flat_map(|s| s.words.iter().map(|w| w.text.as_str()))
            .collect(),
        Block::BlockQuote(inner) => inner
            .iter()
            .flat_map(|b| collect_paragraph_words(b))
            .collect(),
        Block::List(_) | Block::CodeBlock(_) => Vec::new(),
    }
}

#[cfg(test)]
#[path = "banned_words_tests.rs"]
mod tests;
