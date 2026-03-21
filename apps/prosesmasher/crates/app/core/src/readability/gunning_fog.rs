//! Gunning Fog index check — estimates the years of education needed.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document's Gunning Fog index stays below the configured
/// maximum threshold.
///
/// Formula: `0.4 × ((words/sentences) + 100 × (complex_words/words))`
///
/// A "complex word" has 3 or more syllables. Lower scores indicate simpler text.
#[derive(Debug)]
pub struct GunningFogCheck;

impl Check for GunningFogCheck {
    fn id(&self) -> &'static str {
        "gunning-fog"
    }

    fn label(&self) -> &'static str {
        "Gunning Fog Index"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(max) = config.thresholds.gunning_fog_max else {
            return;
        };

        let total_words = doc.metadata.total_words;
        let total_sentences = doc.metadata.total_sentences;

        if total_sentences == 0 || total_words == 0 {
            return;
        }

        let mut complex_count: usize = 0;
        for section in &doc.sections {
            count_complex_in_blocks(&section.blocks, &mut complex_count);
        }

        let words_f = f64::from(u32::try_from(total_words).unwrap_or(u32::MAX));
        let sentences_f = f64::from(u32::try_from(total_sentences).unwrap_or(u32::MAX));
        let complex_f = f64::from(u32::try_from(complex_count).unwrap_or(u32::MAX));

        let score = 0.4 * 100.0f64.mul_add(complex_f / words_f, words_f / sentences_f);

        let score_100 = f64_to_i64_x100(score);
        let max_100 = f64_to_i64_x100(max);

        let _result = suite
            .expect_value_to_be_at_most("gunning-fog", score_100, max_100)
            .label("Gunning Fog Index")
            .checking("fog index (×100)");
    }
}

/// Count words with 3+ syllables in all blocks (recursive for blockquotes).
fn count_complex_in_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    for word in &sentence.words {
                        if word.syllable_count >= 3 {
                            *count = count.saturating_add(1);
                        }
                    }
                }
            }
            Block::BlockQuote(inner) => count_complex_in_blocks(inner, count),
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
#[path = "gunning_fog_tests.rs"]
mod tests;
