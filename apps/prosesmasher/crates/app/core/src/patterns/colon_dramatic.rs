//! Colon-dramatic check — flags short dramatic clauses after colons.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Detects dramatic colon usage where a short clause follows a colon
/// (e.g., "And then it hit me: everything changed.").
#[derive(Debug)]
pub struct ColonDramaticCheck;

impl Check for ColonDramaticCheck {
    fn id(&self) -> &'static str {
        "colon-dramatic"
    }

    fn label(&self) -> &'static str {
        "Dramatic Colon"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut count: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, &mut count);
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("colon-dramatic", count_i64, 0, 0)
            .label("Dramatic Colon")
            .checking("short dramatic clauses after colons");
    }
}

fn check_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    if is_dramatic_colon(&sentence.text) {
                        *count = count.saturating_add(1);
                    }
                }
            }
            Block::BlockQuote(inner) => check_blocks(inner, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

/// Check if a sentence has a dramatic colon: text after colon is < 6 words
/// and contains no commas (which would indicate a list).
fn is_dramatic_colon(text: &str) -> bool {
    let Some(colon_pos) = text.find(':') else {
        return false;
    };
    let Some(after_colon_raw) = text.get(colon_pos.saturating_add(1)..) else {
        return false;
    };
    let after_colon = after_colon_raw.trim();
    if after_colon.is_empty() {
        return false;
    }
    // If there are commas after the colon, it's likely a list
    if after_colon.contains(',') {
        return false;
    }
    let word_count = after_colon.split_whitespace().count();
    word_count < 6
}

#[cfg(test)]
#[path = "colon_dramatic_tests.rs"]
mod tests;
