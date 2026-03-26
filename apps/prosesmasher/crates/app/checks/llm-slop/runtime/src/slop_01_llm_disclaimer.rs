//! LLM-disclaimer check — flags explicit model/disclaimer leakage.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

#[derive(Debug)]
pub struct LlmDisclaimerCheck;

impl Check for LlmDisclaimerCheck {
    fn id(&self) -> &'static str {
        "llm-disclaimer"
    }

    fn label(&self) -> &'static str {
        "LLM Disclaimer"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.llm_disclaimer.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_llm_disclaimer_evidence(doc);
        let _result = suite
            .record_custom_values(
                "llm-disclaimer",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("LLM Disclaimer")
            .checking("explicit LLM identity, knowledge-cutoff, or real-time-access disclaimers");
    }
}

const START_PATTERNS: &[&str] = &[
    "as a language model",
    "as an ai language model",
    "as a language model ai",
    "as a large language model",
    "as of my knowledge cutoff",
    "as of my last knowledge update",
    "my knowledge cutoff date is",
    "i do not have access to real-time",
    "i don't have access to real-time",
    "i do not have real-time",
    "i don't have real-time",
];

const CONTAINS_PATTERNS: &[&str] = &[
    "as i am an ai language model",
    "as i am a language model",
    "i am an ai language model",
    "i am a language model",
    "knowledge cutoff date is",
    "my knowledge is based on data up to",
    "my training data up until",
    "do not have access to real-time information",
    "don't have access to real-time information",
    "do not have access to real-time data",
    "don't have access to real-time data",
];

fn collect_llm_disclaimer_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();
    let mut paragraph_index: usize = 0;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_llm_disclaimer_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_llm_disclaimer_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                if let Some(pattern) = match_disclaimer_pattern(&sentence.text) {
                    evidence.push(json!({
                        "section_index": section_index,
                        "paragraph_index": *paragraph_index,
                        "sentence_index": sentence_index,
                        "matched_text": pattern,
                        "sentence": sentence.text,
                    }));
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_llm_disclaimer_evidence_from_block(
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

fn match_disclaimer_pattern(sentence: &str) -> Option<&'static str> {
    let normalized = normalize(sentence);

    START_PATTERNS
        .iter()
        .find(|pattern| starts_with_disclaimer(&normalized, pattern))
        .copied()
        .or_else(|| {
            CONTAINS_PATTERNS
                .iter()
                .find(|pattern| normalized.contains(**pattern))
                .copied()
        })
}

fn starts_with_disclaimer(normalized: &str, pattern: &str) -> bool {
    normalized == pattern
        || normalized
            .strip_prefix(pattern)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with([' ', ',', '.', ':', ';']))
        || normalized
            .strip_prefix("however, ")
            .is_some_and(|rest| starts_with_disclaimer(rest, pattern))
        || normalized
            .strip_prefix("but ")
            .is_some_and(|rest| starts_with_disclaimer(rest, pattern))
}

fn normalize(sentence: &str) -> String {
    sentence
        .chars()
        .map(|ch| match ch {
            '\u{2018}' | '\u{2019}' => '\'',
            '\u{201C}' | '\u{201D}' => '"',
            _ => ch.to_ascii_lowercase(),
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "slop_01_llm_disclaimer_tests/mod.rs"]
mod tests;
