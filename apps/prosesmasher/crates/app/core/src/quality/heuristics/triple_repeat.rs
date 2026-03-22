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

    for section in &doc.sections {
        for block in &section.blocks {
            collect_triple_repeat_evidence_from_block(block, &mut evidence);
        }
    }

    evidence
}

fn collect_triple_repeat_evidence_from_block(block: &Block, evidence: &mut Vec<Value>) {
    match block {
        Block::Paragraph(paragraph) => collect_triple_repeat_evidence_from_paragraph(paragraph, evidence),
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_triple_repeat_evidence_from_block(inner_block, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_triple_repeat_evidence_from_paragraph(para: &Paragraph, evidence: &mut Vec<Value>) {
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
                "matched_text": a,
                "sentence_1": sentence_a.text,
                "sentence_2": sentence_b.text,
                "sentence_3": sentence_c.text,
            }));
        }
    }
}

#[cfg(test)]
#[path = "triple_repeat_tests.rs"]
mod tests;
