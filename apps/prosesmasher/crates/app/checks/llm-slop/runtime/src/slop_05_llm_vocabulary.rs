//! LLM-vocabulary check — flags repeated stock LLM-era diction.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{normalize, sentence_evidence, strip_quoted_segments};

#[derive(Debug)]
pub struct LlmVocabularyCheck;

impl Check for LlmVocabularyCheck {
    fn id(&self) -> &'static str {
        "llm-vocabulary"
    }

    fn label(&self) -> &'static str {
        "LLM Vocabulary"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.llm_vocabulary.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config.quality.heuristics.llm_vocabulary.max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_llm_vocabulary_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "llm-vocabulary",
                observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("LLM Vocabulary")
            .checking("repeated stock llm-era diction per document");
    }
}

const LLM_VOCABULARY: &[&str] = &[
    "delve",
    "vibrant",
    "landscape",
    "realm",
    "embark",
    "excels",
    "vital",
    "comprehensive",
    "intricate",
    "pivotal",
    "moreover",
    "tapestry",
];

fn collect_llm_vocabulary_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();
    let mut paragraph_index: usize = 0;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_from_block(block, section_index, &mut paragraph_index, &mut evidence);
        }
    }

    evidence
}

fn collect_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                for matched_text in match_llm_vocabulary(&sentence.text) {
                    evidence.push(sentence_evidence(
                        section_index,
                        *paragraph_index,
                        sentence_index,
                        &[("matched_text", matched_text), ("sentence", &sentence.text)],
                    ));
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_from_block(inner_block, section_index, paragraph_index, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn match_llm_vocabulary(sentence: &str) -> Vec<&'static str> {
    let normalized = strip_quoted_segments(&normalize(sentence));
    token_words(&normalized)
        .into_iter()
        .filter_map(|token| LLM_VOCABULARY.iter().find(|candidate| token == **candidate))
        .copied()
        .collect()
}

fn token_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "slop_05_llm_vocabulary_tests/mod.rs"]
mod tests;
