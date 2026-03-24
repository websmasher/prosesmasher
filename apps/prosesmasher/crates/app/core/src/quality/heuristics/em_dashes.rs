//! Em-dash check — flags closed em-dash characters (U+2014) in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

/// Checks that the document contains zero closed em-dash characters.
///
/// A closed em dash has no surrounding whitespace, like `word—word`.
/// Spaced em dashes (`word — word`) are allowed.
#[derive(Debug)]
pub struct EmDashCheck;

impl Check for EmDashCheck {
    fn id(&self) -> &'static str {
        "em-dashes"
    }

    fn label(&self) -> &'static str {
        "No Closed Em-Dashes"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut evidence = Vec::new();
        let mut paragraph_index: usize = 0;

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                collect_em_dash_evidence(
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
                "em-dashes",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(count_i64),
                &evidence,
            )
            .label("No Closed Em-Dashes")
            .checking("closed em-dash characters (U+2014)");
    }
}

fn collect_em_dash_evidence(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                let match_count = count_closed_em_dashes(&sentence.text);
                if match_count > 0 {
                    evidence.push(json!({
                        "section_index": section_index,
                        "paragraph_index": *paragraph_index,
                        "sentence_index": sentence_index,
                        "matched_text": "\u{2014}",
                        "match_count": match_count,
                        "sentence": sentence.text,
                    }));
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_em_dash_evidence(inner_block, section_index, paragraph_index, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn count_closed_em_dashes(text: &str) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let mut count = 0_usize;

    for (index, current) in chars.iter().enumerate() {
        if *current != '\u{2014}' {
            continue;
        }

        let prev = index.checked_sub(1).and_then(|prev_index| chars.get(prev_index));
        let next = chars.get(index.saturating_add(1));

        if prev.is_some_and(|c| !c.is_whitespace()) && next.is_some_and(|c| !c.is_whitespace()) {
            count = count.saturating_add(1);
        }
    }

    count
}

#[cfg(test)]
#[path = "em_dashes_tests.rs"]
mod tests;
