//! Triple-repeat check — flags three consecutive sentences starting with the same word.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};
use serde_json::{Value, json};

use crate::check::Check;

/// Detects paragraphs where three consecutive sentences begin with the same word.
#[derive(Debug)]
pub struct TripleRepeatCheck;

impl Check for TripleRepeatCheck {
    fn id(&self) -> &'static str {
        "triple-repeat"
    }

    fn label(&self) -> &'static str {
        "Triple Repeat Opener"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let evidence = collect_triple_repeat_evidence(doc);
        let _result = suite
            .record_custom_values(
                "triple-repeat",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Triple Repeat Opener")
            .checking("three consecutive sentences starting with the same word");
    }
}

fn collect_triple_repeat_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_triple_repeat_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_triple_repeat_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            collect_triple_repeat_evidence_from_paragraph(
                paragraph,
                section_index,
                *paragraph_index,
                evidence,
            );
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_triple_repeat_evidence_from_block(
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

fn collect_triple_repeat_evidence_from_paragraph(
    para: &Paragraph,
    section_index: usize,
    paragraph_index: usize,
    evidence: &mut Vec<Value>,
) {
    if para.sentences.len() < 3 {
        return;
    }

    let first_words: Vec<String> = para
        .sentences
        .iter()
        .filter_map(|sentence| sentence.words.first().map(|word| word.text.to_lowercase()))
        .collect();

    if first_words.len() < 3 {
        return;
    }

    for (sentence_index, window) in first_words.windows(3).enumerate() {
        let Some(a) = window.first() else {
            continue;
        };
        let Some(b) = window.get(1) else {
            continue;
        };
        let Some(c) = window.get(2) else {
            continue;
        };
        if a == b && b == c {
            let Some(sentence_a) = para.sentences.get(sentence_index) else {
                continue;
            };
            let Some(sentence_b) = para.sentences.get(sentence_index.saturating_add(1)) else {
                continue;
            };
            let Some(sentence_c) = para.sentences.get(sentence_index.saturating_add(2)) else {
                continue;
            };

            evidence.push(json!({
                "section_index": section_index,
                "paragraph_index": paragraph_index,
                "sentence_index": sentence_index,
                "sentence_index_next": sentence_index.saturating_add(1),
                "sentence_index_third": sentence_index.saturating_add(2),
                "matched_text": a,
                "sentence": sentence_a.text,
                "next_sentence": sentence_b.text,
                "third_sentence": sentence_c.text,
            }));
        }
    }
}

#[cfg(test)]
#[path = "triple_repeat_tests.rs"]
mod tests;
