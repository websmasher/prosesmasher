//! Paragraph length check — validates that paragraphs don't exceed max sentence count.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Checks that each paragraph's sentence count stays within the configured max.
#[derive(Debug)]
pub struct ParagraphLengthCheck;

impl Check for ParagraphLengthCheck {
    fn id(&self) -> &'static str {
        "paragraph-length"
    }

    fn label(&self) -> &'static str {
        "Paragraph Length"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.paragraph_length.enabled {
            return;
        }

        let max_sentences = config.quality.heuristics.paragraph_length.max_sentences;

        let max_i64 = i64::try_from(max_sentences).unwrap_or(i64::MAX);
        let mut para_index: usize = 0;

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                check_block(block, section_index, &mut para_index, max_i64, suite);
            }
        }
    }
}

fn check_block(
    block: &Block,
    section_index: usize,
    para_index: &mut usize,
    max_i64: i64,
    suite: &mut ExpectationSuite,
) {
    match block {
        Block::Paragraph(p) => {
            let sentence_count = i64::try_from(p.sentences.len()).unwrap_or(i64::MAX);
            let col = format!("paragraph-length-{para_index}");
            let paragraph_text = p
                .sentences
                .iter()
                .map(|sentence| sentence.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            let evidence = if sentence_count > max_i64 {
                vec![json!({
                    "section_index": section_index,
                    "paragraph_index": *para_index,
                    "paragraph_text": paragraph_text,
                    "sentence_count": sentence_count,
                    "max_allowed": max_i64,
                })]
            } else {
                Vec::new()
            };
            let _result = suite
                .record_custom_values(
                    &col,
                    sentence_count <= max_i64,
                    json!({ "max_sentences": max_i64 }),
                    json!(sentence_count),
                    &evidence,
                )
                .label("Paragraph Length")
                .checking(&format!("paragraph {para_index} sentence count"));
            *para_index = para_index.saturating_add(1);
        }
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                check_block(inner, section_index, para_index, max_i64, suite);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[cfg(test)]
#[path = "paragraph_length_tests.rs"]
mod tests;
