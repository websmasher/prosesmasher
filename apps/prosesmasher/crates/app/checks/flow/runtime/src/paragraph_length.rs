//! Paragraph length check — validates that paragraphs don't exceed max sentence count.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

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
        if !config.quality.flow.paragraph_length.enabled {
            return;
        }

        let max_sentences = config.quality.flow.paragraph_length.max_sentences;

        let max_i64 = i64::try_from(max_sentences).unwrap_or(i64::MAX);
        let mut para_index: usize = 0;
        let mut evidence: Vec<Value> = Vec::new();

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                check_block(
                    block,
                    section_index,
                    &mut para_index,
                    max_i64,
                    &mut evidence,
                );
            }
        }

        let _result = suite
            .record_custom_values(
                "paragraph-length",
                evidence.is_empty(),
                json!({ "max_sentences": max_i64 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Paragraph Length")
            .checking("paragraph sentence count");
    }
}

fn check_block(
    block: &Block,
    section_index: usize,
    para_index: &mut usize,
    max_i64: i64,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(p) => {
            let sentence_count = i64::try_from(p.sentences.len()).unwrap_or(i64::MAX);
            let paragraph_text = p
                .sentences
                .iter()
                .map(|sentence| sentence.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            if sentence_count > max_i64 {
                evidence.push(json!({
                    "section_index": section_index,
                    "paragraph_index": *para_index,
                    "paragraph_text": paragraph_text,
                    "sentence_count": sentence_count,
                    "max_allowed": max_i64,
                }));
            }
            *para_index = para_index.saturating_add(1);
        }
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                check_block(inner, section_index, para_index, max_i64, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[cfg(test)]
#[path = "paragraph_length_tests/mod.rs"]
mod tests;
