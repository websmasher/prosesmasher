//! Coleman-Liau index check — estimates the US grade level of prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document's Coleman-Liau index stays below the configured
/// maximum threshold.
///
/// Formula: `0.0588 × L - 0.296 × S - 15.8`
/// where L = letters per 100 words, S = sentences per 100 words.
#[derive(Debug)]
pub struct ColemanLiauCheck;

impl Check for ColemanLiauCheck {
    fn id(&self) -> &'static str {
        "coleman-liau"
    }

    fn label(&self) -> &'static str {
        "Coleman-Liau Index"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(max) = config.thresholds.coleman_liau_max else {
            return;
        };

        let total_words = doc.metadata.total_words;
        let total_sentences = doc.metadata.total_sentences;

        if total_words == 0 {
            return;
        }

        let mut total_letters: usize = 0;
        for section in &doc.sections {
            count_letters_in_blocks(&section.blocks, &mut total_letters);
        }

        let words_f = f64::from(u32::try_from(total_words).unwrap_or(u32::MAX));
        let sentences_f = f64::from(u32::try_from(total_sentences).unwrap_or(u32::MAX));
        let letters_f = f64::from(u32::try_from(total_letters).unwrap_or(u32::MAX));

        let l = letters_f / words_f * 100.0;
        let s = sentences_f / words_f * 100.0;

        let score = 0.0588f64.mul_add(l, 0.296f64.mul_add(-s, -15.8));

        let score_100 = f64_to_i64_x100(score);
        let max_100 = f64_to_i64_x100(max);

        let _result = suite
            .expect_value_to_be_at_most("coleman-liau", score_100, max_100)
            .label("Coleman-Liau Index")
            .checking("grade level index (×100)");
    }
}

/// Count alphabetic characters in all word texts (recursive for blockquotes).
fn count_letters_in_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    for word in &sentence.words {
                        *count = count.saturating_add(
                            word.text.chars().filter(|c| c.is_alphabetic()).count(),
                        );
                    }
                }
            }
            Block::BlockQuote(inner) => count_letters_in_blocks(inner, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

/// Multiply by 100, round, and convert to i64 without using `as`.
fn f64_to_i64_x100(value: f64) -> i64 {
    let rounded = (value * 100.0).round();
    let clamped = rounded.clamp(f64::from(i32::MIN), f64::from(i32::MAX));
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    let narrow = clamped as i32;
    i64::from(narrow)
}

#[cfg(test)]
#[path = "coleman_liau_tests.rs"]
mod tests;
