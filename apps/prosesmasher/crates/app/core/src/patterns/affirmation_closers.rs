//! Affirmation-closers check — flags sections ending with affirmation phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};

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
        let mut count: usize = 0;
        for section in &doc.sections {
            let Some(para) = last_paragraph(&section.blocks) else {
                continue;
            };
            let Some(sentence) = para.sentences.last() else {
                continue;
            };
            let lower = sentence.text.to_lowercase();
            let matched = config.terms.affirmation_closers.iter().any(|phrase| {
                lower.ends_with(&phrase.to_lowercase())
            });
            if matched {
                count = count.saturating_add(1);
            }
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("affirmation-closers", count_i64, 0, 0)
            .label("Affirmation Closers")
            .checking("section closers matching affirmation patterns");
    }
}

fn last_paragraph(blocks: &[Block]) -> Option<&Paragraph> {
    for block in blocks.iter().rev() {
        match block {
            Block::Paragraph(p) => return Some(p),
            Block::BlockQuote(inner) => {
                if let Some(p) = last_paragraph(inner) {
                    return Some(p);
                }
            }
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
    None
}

#[cfg(test)]
#[path = "affirmation_closers_tests.rs"]
mod tests;
