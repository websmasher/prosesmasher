//! Flesch-Kincaid readability check — measures reading ease of prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Checks that the document's Flesch-Kincaid reading ease score meets the
/// configured minimum threshold.
///
/// Formula: `206.835 - 1.015 × (words/sentences) - 84.6 × (syllables/words)`
///
/// Higher scores indicate easier text. Typical thresholds:
/// - 60–70: plain English
/// - 30–50: academic / professional
#[derive(Debug)]
pub struct FleschKincaidCheck;

impl Check for FleschKincaidCheck {
    fn id(&self) -> &'static str {
        "flesch-kincaid"
    }

    fn label(&self) -> &'static str {
        "Flesch-Kincaid Reading Ease"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.readability.enabled {
            return;
        }
        let Some(min) = config.quality.readability.flesch_kincaid_min else {
            return;
        };

        let total_words = doc.metadata.total_words;
        let total_sentences = doc.metadata.total_sentences;
        let total_syllables = doc.metadata.total_syllables;

        if total_sentences == 0 || total_words == 0 {
            return;
        }

        let words_f = f64::from(u32::try_from(total_words).unwrap_or(u32::MAX));
        let sentences_f = f64::from(u32::try_from(total_sentences).unwrap_or(u32::MAX));
        let syllables_f = f64::from(u32::try_from(total_syllables).unwrap_or(u32::MAX));

        let score = 1.015f64.mul_add(
            -(words_f / sentences_f),
            84.6f64.mul_add(-(syllables_f / words_f), 206.835),
        );

        let score_100 = f64_to_i64_x100(score);
        let min_100 = f64_to_i64_x100(min);

        let _result = suite
            .record_custom_values(
                "flesch-kincaid",
                score_100 >= min_100,
                json!({
                    "minimum_score_x100": min_100,
                    "formula": "206.835 - 1.015 × (words/sentences) - 84.6 × (syllables/words)",
                }),
                json!({
                    "score_x100": score_100,
                    "score": score,
                    "total_words": total_words,
                    "total_sentences": total_sentences,
                    "total_syllables": total_syllables,
                }),
                &[json!({
                    "score_x100": score_100,
                    "score": score,
                    "total_words": total_words,
                    "total_sentences": total_sentences,
                    "total_syllables": total_syllables,
                    "minimum_score_x100": min_100,
                })],
            )
            .label("Flesch-Kincaid Reading Ease")
            .checking("reading ease score (×100)");
    }
}

/// Multiply by 100, round, and convert to i64 without using `as`.
fn f64_to_i64_x100(value: f64) -> i64 {
    // round() returns f64; convert via i32 which has From<i32> for i64.
    // For readability scores the range is well within i32 bounds.
    let rounded = (value * 100.0).round();
    // Clamp to i32 range then widen to i64.
    let clamped = rounded.clamp(f64::from(i32::MIN), f64::from(i32::MAX));
    // SAFETY: value is clamped to i32 range, so truncation is intentional.
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    let narrow = clamped as i32;
    i64::from(narrow)
}

#[cfg(test)]
#[path = "flesch_kincaid_tests/mod.rs"]
mod tests;
