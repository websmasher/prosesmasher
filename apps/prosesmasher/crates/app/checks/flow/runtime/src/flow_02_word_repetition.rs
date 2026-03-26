//! Word repetition check — flags words that repeat too often within a paragraph.

use std::collections::BTreeMap;

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};
use serde_json::{Value, json};

use crate::check::Check;

/// Checks that no single word exceeds the configured repetition threshold
/// within a paragraph.
#[derive(Debug)]
pub struct WordRepetitionCheck;

impl Check for WordRepetitionCheck {
    fn id(&self) -> &'static str {
        "word-repetition"
    }

    fn label(&self) -> &'static str {
        "Word Repetition"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.flow.word_repetition.enabled {
            return;
        }

        let max_repetition = config.quality.flow.word_repetition.max;

        let excluded_terms =
            resolve_excluded_terms(&config.quality.flow.word_repetition.excluded_terms);

        let max_i64 = i64::try_from(max_repetition).unwrap_or(i64::MAX);
        let mut evidence: Vec<Value> = Vec::new();

        for section in &doc.sections {
            for block in &section.blocks {
                collect_block_evidence(block, max_i64, &excluded_terms, &mut evidence);
            }
        }

        let _result = suite
            .record_custom_values(
                "word-repetition",
                evidence.is_empty(),
                json!({ "max": max_i64 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Word Repetition")
            .checking("word repetition within a paragraph above threshold");
    }
}

fn collect_block_evidence(
    block: &Block,
    max_i64: i64,
    excluded_terms: &[String],
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(p) => collect_paragraph_evidence(p, max_i64, excluded_terms, evidence),
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                collect_block_evidence(inner, max_i64, excluded_terms, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_paragraph_evidence(
    para: &Paragraph,
    max_i64: i64,
    excluded_terms: &[String],
    evidence: &mut Vec<Value>,
) {
    if is_markup_like_paragraph(para) {
        return;
    }

    let mut freq: BTreeMap<String, usize> = BTreeMap::new();

    for sentence in &para.sentences {
        for word in &sentence.words {
            let lowered = word.text.to_lowercase();
            let entry = freq.entry(lowered).or_insert(0);
            *entry = entry.saturating_add(1);
        }
    }

    for (word, count) in &freq {
        if word.len() < 4 {
            continue;
        }

        if excluded_terms.iter().any(|term| term == word) {
            continue;
        }

        let observed = i64::try_from(*count).unwrap_or(i64::MAX);
        if observed > max_i64 {
            evidence.push(json!({
                "word": word,
                "count": observed,
                "max": max_i64,
                "paragraph_text": paragraph_text(para),
            }));
        }
    }
}

fn paragraph_text(para: &Paragraph) -> String {
    para.sentences
        .iter()
        .map(|sentence| sentence.text.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_markup_like_paragraph(para: &Paragraph) -> bool {
    paragraph_text(para).trim_start().starts_with('<')
}

fn resolve_excluded_terms(
    overrides: &prosesmasher_domain_types::OverrideList<String>,
) -> Vec<String> {
    let mut values = overrides.add.clone();
    if !overrides.remove.is_empty() {
        let removed: std::collections::BTreeSet<String> = overrides
            .remove
            .iter()
            .map(|item| item.to_lowercase())
            .collect();
        values.retain(|item| !removed.contains(&item.to_lowercase()));
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for value in values {
        let lowered = value.to_lowercase();
        if seen.insert(lowered) {
            deduped.push(value);
        }
    }
    deduped
}

#[cfg(test)]
#[path = "flow_02_word_repetition_tests/mod.rs"]
mod tests;
