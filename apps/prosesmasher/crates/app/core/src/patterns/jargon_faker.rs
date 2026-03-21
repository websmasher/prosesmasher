//! Jargon-faker check — flags sentences containing fake tech jargon phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

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
        if config.terms.jargon_faker_phrases.is_empty() {
            return;
        }
        let mut count: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, config, &mut count);
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("jargon-faker", count_i64, 0, 0)
            .label("Jargon Faker")
            .checking("sentences containing fake tech jargon");
    }
}

fn check_blocks(blocks: &[Block], config: &CheckConfig, count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    let lower = sentence.text.to_lowercase();
                    let matched = config.terms.jargon_faker_phrases.iter().any(|phrase| {
                        lower.contains(&phrase.to_lowercase())
                    });
                    if matched {
                        *count = count.saturating_add(1);
                    }
                }
            }
            Block::BlockQuote(inner) => check_blocks(inner, config, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

#[cfg(test)]
#[path = "jargon_faker_tests.rs"]
mod tests;
