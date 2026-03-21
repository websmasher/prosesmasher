//! Em-dash check — flags em-dash characters (U+2014) in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document contains zero em-dash characters.
///
/// Em-dashes are a strong signal of AI-generated text and are
/// flagged regardless of context.
#[derive(Debug)]
pub struct EmDashCheck;

impl Check for EmDashCheck {
    fn id(&self) -> &'static str {
        "em-dashes"
    }

    fn label(&self) -> &'static str {
        "No Em-Dashes"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut count: usize = 0;
        for section in &doc.sections {
            count_em_dashes_in_blocks(&section.blocks, &mut count);
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("em-dashes", count_i64, 0, 0)
            .label("No Em-Dashes")
            .checking("em-dash characters (U+2014)");
    }
}

fn count_em_dashes_in_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    *count = count.saturating_add(
                        sentence.text.chars().filter(|c| *c == '\u{2014}').count(),
                    );
                }
            }
            Block::BlockQuote(inner) => count_em_dashes_in_blocks(inner, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

#[cfg(test)]
#[path = "em_dashes_tests.rs"]
mod tests;
