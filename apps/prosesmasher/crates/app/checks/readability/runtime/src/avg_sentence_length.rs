//! Average sentence length check — flags overly long sentences on average.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

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
        if !config.quality.readability.enabled {
            return;
        }
        let Some(max) = config.quality.readability.avg_sentence_length_max else {
            return;
        };

        let total_sentences = doc.metadata.total_sentences;
        if total_sentences == 0 {
            return;
        }

        let avg = doc
            .metadata
            .total_words
            .checked_div(total_sentences)
            .unwrap_or(0);

        let avg_i64 = i64::try_from(avg).unwrap_or(i64::MAX);
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);

        let _result = suite
            .record_custom_values(
                "avg-sentence-length",
                avg_i64 <= max_i64,
                json!({
                    "max_words_per_sentence": max_i64,
                    "formula": "total_words / total_sentences",
                }),
                json!({
                    "average_words_per_sentence": avg_i64,
                    "total_words": doc.metadata.total_words,
                    "total_sentences": total_sentences,
                }),
                &[json!({
                    "average_words_per_sentence": avg_i64,
                    "total_words": doc.metadata.total_words,
                    "total_sentences": total_sentences,
                    "max_words_per_sentence": max_i64,
                })],
            )
            .label("Average Sentence Length")
            .checking("words per sentence");
    }
}

#[cfg(test)]
#[path = "avg_sentence_length_tests/mod.rs"]
mod tests;
