//! Smart-quotes check — flags curly quote characters in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document contains zero smart-quote characters.
///
/// Curly quotes (U+201C, U+201D, U+2018, U+2019) are a common signal
/// of AI-generated or improperly formatted text.
#[derive(Debug)]
pub struct SmartQuotesCheck;

impl Check for SmartQuotesCheck {
    fn id(&self) -> &'static str {
        "smart-quotes"
    }

    fn label(&self) -> &'static str {
        "No Smart Quotes"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut count: usize = 0;
        for section in &doc.sections {
            count_smart_quotes_in_blocks(&section.blocks, &mut count);
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("smart-quotes", count_i64, 0, 0)
            .label("No Smart Quotes")
            .checking("curly quote characters (U+201C, U+201D, U+2018, U+2019)");
    }
}

const SMART_QUOTE_CHARS: [char; 4] = ['\u{201C}', '\u{201D}', '\u{2018}', '\u{2019}'];

fn is_smart_quote(c: char) -> bool {
    SMART_QUOTE_CHARS.contains(&c)
}

fn count_smart_quotes_in_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    *count = count.saturating_add(
                        sentence.text.chars().filter(|c| is_smart_quote(*c)).count(),
                    );
                }
            }
            Block::BlockQuote(inner) => count_smart_quotes_in_blocks(inner, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

#[cfg(test)]
#[path = "smart_quotes_tests.rs"]
mod tests;
