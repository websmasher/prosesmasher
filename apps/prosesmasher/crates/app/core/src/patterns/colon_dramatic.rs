//! Colon-dramatic check — flags short dramatic clauses after colons.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

type ColonDetails<'a> = (&'a str, usize);

/// Detects dramatic colon usage where a short clause follows a colon
/// (e.g., "And then it hit me: everything changed.").
#[derive(Debug)]
pub struct ColonDramaticCheck;

impl Check for ColonDramaticCheck {
    fn id(&self) -> &'static str {
        "colon-dramatic"
    }

    fn label(&self) -> &'static str {
        "Dramatic Colon"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let evidence = collect_colon_dramatic_evidence(doc);
        let _result = suite
            .record_custom_values(
                "colon-dramatic",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Dramatic Colon")
            .checking("short dramatic clauses after colons");
    }
}

fn collect_colon_dramatic_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_colon_dramatic_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_colon_dramatic_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                let Some((after_colon, word_count)) = dramatic_colon_details(&sentence.text)
                else {
                    continue;
                };
                evidence.push(json!({
                    "section_index": section_index,
                    "paragraph_index": *paragraph_index,
                    "sentence_index": sentence_index,
                    "matched_text": after_colon,
                    "after_colon": after_colon,
                    "word_count": word_count,
                    "sentence": sentence.text,
                }));
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_colon_dramatic_evidence_from_block(
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

/// Check if a sentence has a dramatic colon: text after colon is < 6 words
/// and contains no commas (which would indicate a list).
fn dramatic_colon_details(text: &str) -> Option<ColonDetails<'_>> {
    let colon_pos = text.find(':')?;
    let after_colon_raw = text.get(colon_pos.saturating_add(1)..)?;
    let after_colon = after_colon_raw.trim();
    if after_colon.is_empty() {
        return None;
    }
    // If there are commas after the colon, it's likely a list
    if after_colon.contains(',') {
        return None;
    }
    let word_count = after_colon.split_whitespace().count();
    if word_count < 6 {
        Some((after_colon, word_count))
    } else {
        None
    }
}

#[cfg(test)]
#[path = "colon_dramatic_tests.rs"]
mod tests;
