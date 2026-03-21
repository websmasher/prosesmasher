//! Exclamation-density check — flags paragraphs with too many exclamation marks.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that no paragraph exceeds the configured exclamation mark threshold.
#[derive(Debug)]
pub struct ExclamationDensityCheck;

impl Check for ExclamationDensityCheck {
    fn id(&self) -> &'static str {
        "exclamation-density"
    }

    fn label(&self) -> &'static str {
        "Exclamation Density"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(max) = config.thresholds.max_exclamations_per_paragraph else {
            return;
        };
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let mut para_idx: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, &mut para_idx, max_i64, suite);
        }
    }
}

fn check_blocks(
    blocks: &[Block],
    para_idx: &mut usize,
    max_i64: i64,
    suite: &mut ExpectationSuite,
) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                let mut count: usize = 0;
                for sentence in &p.sentences {
                    count = count.saturating_add(
                        sentence.text.chars().filter(|c| *c == '!').count(),
                    );
                }
                let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
                let _result = suite
                    .expect_value_to_be_at_most(
                        &format!("exclamation-density-para-{para_idx}"),
                        count_i64,
                        max_i64,
                    )
                    .label("Exclamation Density")
                    .checking(&format!("paragraph {para_idx} exclamation count"));
                *para_idx = para_idx.saturating_add(1);
            }
            Block::BlockQuote(inner) => check_blocks(inner, para_idx, max_i64, suite),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

#[cfg(test)]
#[path = "exclamation_density_tests.rs"]
mod tests;
