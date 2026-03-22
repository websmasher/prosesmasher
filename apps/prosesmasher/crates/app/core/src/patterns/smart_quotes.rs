//! Smart-quotes check — flags curly quote characters in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

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
        let mut evidence = Vec::new();
        let mut paragraph_index: usize = 0;

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                collect_smart_quote_evidence(
                    block,
                    section_index,
                    &mut paragraph_index,
                    &mut evidence,
                );
            }
        }
        let count = evidence.len();
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "smart-quotes",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(count_i64),
                &evidence,
            )
            .label("No Smart Quotes")
            .checking("curly quote characters (U+201C, U+201D, U+2018, U+2019)");
    }
}

const SMART_QUOTE_CHARS: [char; 4] = ['\u{201C}', '\u{201D}', '\u{2018}', '\u{2019}'];

fn is_smart_quote(c: char) -> bool {
    SMART_QUOTE_CHARS.contains(&c)
}

fn collect_smart_quote_evidence(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                let matches: Vec<char> = sentence
                    .text
                    .chars()
                    .filter(|c| is_smart_quote(*c))
                    .collect();
                if !matches.is_empty() {
                    evidence.push(json!({
                        "section_index": section_index,
                        "paragraph_index": *paragraph_index,
                        "sentence_index": sentence_index,
                        "matched_text": matches.iter().collect::<String>(),
                        "match_count": matches.len(),
                        "sentence": sentence.text,
                    }));
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_smart_quote_evidence(
                    inner_block,
                    section_index,
                    paragraph_index,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[cfg(test)]
#[path = "smart_quotes_tests.rs"]
mod tests;
