//! Paragraph length check — validates that paragraphs don't exceed max sentence count.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that each paragraph's sentence count stays within the configured max.
#[derive(Debug)]
pub struct ParagraphLengthCheck;

impl Check for ParagraphLengthCheck {
    fn id(&self) -> &'static str {
        "paragraph-length"
    }

    fn label(&self) -> &'static str {
        "Paragraph Length"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(max_sentences) = config.thresholds.max_paragraph_sentences else {
            return;
        };

        let max_i64 = i64::try_from(max_sentences).unwrap_or(i64::MAX);
        let mut para_index: usize = 0;

        for section in &doc.sections {
            for block in &section.blocks {
                check_block(block, &mut para_index, max_i64, suite);
            }
        }
    }
}

fn check_block(
    block: &Block,
    para_index: &mut usize,
    max_i64: i64,
    suite: &mut ExpectationSuite,
) {
    match block {
        Block::Paragraph(p) => {
            let sentence_count = i64::try_from(p.sentences.len()).unwrap_or(i64::MAX);
            let col = format!("paragraph-length-{para_index}");
            let _result = suite
                .expect_value_to_be_between(&col, sentence_count, 0, max_i64)
                .label("Paragraph Length")
                .checking(&format!("paragraph {para_index} sentence count"));
            *para_index = para_index.saturating_add(1);
        }
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                check_block(inner, para_index, max_i64, suite);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[cfg(test)]
#[path = "paragraph_length_tests.rs"]
mod tests;
