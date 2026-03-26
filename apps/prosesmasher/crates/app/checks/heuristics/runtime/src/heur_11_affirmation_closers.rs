//! Affirmation-closers check — flags sections ending with affirmation phrases.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

/// Detects sections that end with affirmation phrases
/// (e.g., "...and that's the key.").
#[derive(Debug)]
pub struct AffirmationClosersCheck;

impl Check for AffirmationClosersCheck {
    fn id(&self) -> &'static str {
        "affirmation-closers"
    }

    fn label(&self) -> &'static str {
        "Affirmation Closers"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.affirmation_closers.enabled {
            return;
        }
        let affirmation_closers = super::resolve_affirmation_closers(config);
        if affirmation_closers.is_empty() {
            return;
        }
        let evidence = super::collect_section_sentence_evidence(
            doc,
            &affirmation_closers,
            super::section_last_sentence,
            super::sentence_ends_with,
        );
        let broad_evidence = collect_thats_the_formula_evidence(doc);
        let merged_evidence = merge_evidence(evidence, broad_evidence);
        let _result = suite
            .record_custom_values(
                "affirmation-closers",
                merged_evidence.is_empty(),
                json!({ "min": 0, "max": 0, "absent": affirmation_closers }),
                json!(merged_evidence.len()),
                &merged_evidence,
            )
            .label("Affirmation Closers")
            .checking("affirmation-style formula sentences and closers");
    }
}

fn collect_thats_the_formula_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();

    for section in &doc.sections {
        for block in &section.blocks {
            collect_thats_the_formula_from_block(block, &mut evidence);
        }
    }

    evidence
}

fn collect_thats_the_formula_from_block(block: &Block, evidence: &mut Vec<Value>) {
    match block {
        Block::Paragraph(paragraph) => {
            for sentence in &paragraph.sentences {
                let lowered = sentence.text.to_lowercase();
                if looks_like_thats_the_formula(&lowered, sentence.words.len()) {
                    evidence.push(json!({
                        "matched_text": "that's the ...",
                        "sentence": sentence.text,
                    }));
                }
            }
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_thats_the_formula_from_block(inner_block, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn looks_like_thats_the_formula(sentence: &str, word_count: usize) -> bool {
    word_count <= 6 && (sentence.starts_with("that's the ") || sentence.starts_with("that is the "))
}

fn merge_evidence(mut a: Vec<Value>, b: Vec<Value>) -> Vec<Value> {
    for item in b {
        if !a.contains(&item) {
            a.push(item);
        }
    }
    a
}

#[cfg(test)]
#[path = "heur_11_affirmation_closers_tests/mod.rs"]
mod tests;
