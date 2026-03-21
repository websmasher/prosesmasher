//! Summative-closer check — flags sections ending with summative phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};

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
        if config.terms.summative_patterns.is_empty() {
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
            let matched = config.terms.summative_patterns.iter().any(|phrase| {
                lower.starts_with(&phrase.to_lowercase())
            });
            if matched {
                count = count.saturating_add(1);
            }
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("summative-closer", count_i64, 0, 0)
            .label("Summative Closer")
            .checking("section closers matching summative patterns");
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
#[path = "summative_closer_tests.rs"]
mod tests;
