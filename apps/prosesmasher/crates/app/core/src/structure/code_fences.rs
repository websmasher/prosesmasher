//! Code fences check — flags documents that contain code blocks.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document contains no code blocks.
#[derive(Debug)]
pub struct CodeFencesCheck;

impl Check for CodeFencesCheck {
    fn id(&self) -> &'static str {
        "code-fences"
    }

    fn label(&self) -> &'static str {
        "Code Fences"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut count: usize = 0;

        for section in &doc.sections {
            for block in &section.blocks {
                count_code_blocks(block, &mut count);
            }
        }

        let observed = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("code-fences", observed, 0, 0)
            .label("Code Fences")
            .checking("code block count");
    }
}

fn count_code_blocks(block: &Block, count: &mut usize) {
    match block {
        Block::CodeBlock(_) => {
            *count = count.saturating_add(1);
        }
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                count_code_blocks(inner, count);
            }
        }
        Block::Paragraph(_) | Block::List(_) => {}
    }
}

#[cfg(test)]
#[path = "code_fences_tests.rs"]
mod tests;
