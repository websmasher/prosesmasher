//! Exclamation-density check — flags paragraphs with too many exclamation marks.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

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
        if !config.quality.heuristics.exclamation_density.enabled {
            return;
        }
        let max = config
            .quality
            .heuristics
            .exclamation_density
            .max_per_paragraph;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let mut para_index: usize = 0;
        let mut evidence: Vec<Value> = Vec::new();
        for (section_index, section) in doc.sections.iter().enumerate() {
            check_blocks(
                &section.blocks,
                section_index,
                &mut para_index,
                max_i64,
                &mut evidence,
            );
        }
        let _result = suite
            .record_custom_values(
                "exclamation-density",
                evidence.is_empty(),
                json!({ "max": max_i64 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Exclamation Density")
            .checking("paragraph exclamation count");
    }
}

fn check_blocks(
    blocks: &[Block],
    section_index: usize,
    para_index: &mut usize,
    max_i64: i64,
    evidence: &mut Vec<Value>,
) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                let mut count: usize = 0;
                for sentence in &p.sentences {
                    count =
                        count.saturating_add(sentence.text.chars().filter(|c| *c == '!').count());
                }
                let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
                let paragraph_text = p
                    .sentences
                    .iter()
                    .map(|sentence| sentence.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                if count_i64 > max_i64 {
                    evidence.push(json!({
                        "section_index": section_index,
                        "paragraph_index": *para_index,
                        "paragraph_text": paragraph_text,
                        "sentence_texts": p.sentences.iter().map(|sentence| sentence.text.clone()).collect::<Vec<_>>(),
                        "exclamation_count": count_i64,
                        "max_allowed": max_i64,
                    }));
                }
                *para_index = para_index.saturating_add(1);
            }
            Block::BlockQuote(inner) => {
                check_blocks(inner, section_index, para_index, max_i64, evidence)
            }
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

#[cfg(test)]
#[path = "exclamation_density_tests/mod.rs"]
mod tests;
