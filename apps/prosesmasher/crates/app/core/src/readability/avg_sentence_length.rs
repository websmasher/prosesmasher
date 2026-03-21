//! Average sentence length check — flags overly long sentences on average.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document's average sentence length (in words) stays
/// at or below the configured maximum.
///
/// Formula: `total_words / total_sentences`
#[derive(Debug)]
pub struct AvgSentenceLengthCheck;

impl Check for AvgSentenceLengthCheck {
    fn id(&self) -> &'static str {
        "avg-sentence-length"
    }

    fn label(&self) -> &'static str {
        "Average Sentence Length"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(max) = config.thresholds.avg_sentence_length_max else {
            return;
        };

        let total_sentences = doc.metadata.total_sentences;
        if total_sentences == 0 {
            return;
        }

        let avg = doc.metadata.total_words.checked_div(total_sentences).unwrap_or(0);

        let avg_i64 = i64::try_from(avg).unwrap_or(i64::MAX);
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);

        let _result = suite
            .expect_value_to_be_at_most("avg-sentence-length", avg_i64, max_i64)
            .label("Average Sentence Length")
            .checking("words per sentence");
    }
}

#[cfg(test)]
#[path = "avg_sentence_length_tests.rs"]
mod tests;
