//! Word repetition check — flags words that appear too frequently.

use std::collections::BTreeMap;

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};
use serde_json::json;

use crate::check::Check;

/// Checks that no single word exceeds the configured repetition threshold.
#[derive(Debug)]
pub struct WordRepetitionCheck;

impl Check for WordRepetitionCheck {
    fn id(&self) -> &'static str {
        "word-repetition"
    }

    fn label(&self) -> &'static str {
        "Word Repetition"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        let Some(max_repetition) = config.thresholds.word_repetition_max else {
            return;
        };

        let mut freq: BTreeMap<String, usize> = BTreeMap::new();

        for section in &doc.sections {
            for block in &section.blocks {
                collect_words(block, &mut freq);
            }
        }

        // Filter out stop words and words < 4 chars
        let stop_words = &config.terms.stop_words;

        let max_i64 = i64::try_from(max_repetition).unwrap_or(i64::MAX);

        for (word, count) in &freq {
            if word.len() < 4 {
                continue;
            }

            if stop_words.iter().any(|sw| sw == word) {
                continue;
            }

            let observed = i64::try_from(*count).unwrap_or(i64::MAX);
            let col = format!("word-repetition-{word}");
            let _result = suite
                .record_custom_values(
                    &col,
                    observed <= max_i64,
                    json!({ "max": max_i64, "word": word }),
                    json!({ "word": word, "count": observed }),
                    &[json!({
                        "word": word,
                        "count": observed,
                        "max": max_i64,
                    })],
                )
                .label("Word Repetition")
                .checking(&format!("frequency of \"{word}\""));
        }
    }
}

fn collect_words(block: &Block, freq: &mut BTreeMap<String, usize>) {
    match block {
        Block::Paragraph(p) => collect_paragraph_words(p, freq),
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                collect_words(inner, freq);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_paragraph_words(para: &Paragraph, freq: &mut BTreeMap<String, usize>) {
    for sentence in &para.sentences {
        for word in &sentence.words {
            let lowered = word.text.to_lowercase();
            let entry = freq.entry(lowered).or_insert(0);
            *entry = entry.saturating_add(1);
        }
    }
}

#[cfg(test)]
#[path = "word_repetition_tests.rs"]
mod tests;
