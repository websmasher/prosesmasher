//! Humble-bragger check — flags sentences containing humble-bragging phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

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
        if config.terms.humble_bragger_phrases.is_empty() {
            return;
        }
        let mut count: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, config, &mut count);
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("humble-bragger", count_i64, 0, 0)
            .label("Humble Bragger")
            .checking("sentences containing humble-bragging phrases");
    }
}

fn check_blocks(blocks: &[Block], config: &CheckConfig, count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    let lower = sentence.text.to_lowercase();
                    let matched = config.terms.humble_bragger_phrases.iter().any(|phrase| {
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
#[path = "humble_bragger_tests.rs"]
mod tests;
