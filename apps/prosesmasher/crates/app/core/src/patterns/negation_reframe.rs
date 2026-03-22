//! Negation-reframe check — flags "not X. It's Y." sentence pairs.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};
use serde_json::{Value, json};

use crate::check::Check;

/// Detects consecutive sentence pairs where A contains a negation signal
/// and B contains a reframe signal (e.g., "This isn't defiance. It's developmental.").
#[derive(Debug)]
pub struct NegationReframeCheck;

impl Check for NegationReframeCheck {
    fn id(&self) -> &'static str {
        "negation-reframe"
    }

    fn label(&self) -> &'static str {
        "Negation-Reframe Pattern"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.negation_signals.is_empty() || config.terms.reframe_signals.is_empty() {
            return;
        }
        let evidence = collect_negation_reframe_evidence(doc, config);
        let _result = suite
            .record_custom_values(
                "negation-reframe",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Negation-Reframe Pattern")
            .checking("consecutive negation + reframe sentence pairs");
    }
}

fn collect_negation_reframe_evidence(doc: &Document, config: &CheckConfig) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_negation_reframe_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                config,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_negation_reframe_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    config: &CheckConfig,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            collect_negation_reframe_evidence_from_paragraph(
                paragraph,
                section_index,
                *paragraph_index,
                config,
                evidence,
            );
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_negation_reframe_evidence_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    config,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_negation_reframe_evidence_from_paragraph(
    para: &Paragraph,
    section_index: usize,
    paragraph_index: usize,
    config: &CheckConfig,
    evidence: &mut Vec<Value>,
) {
    if para.sentences.len() < 2 {
        return;
    }

    for (sentence_index, pair) in para.sentences.windows(2).enumerate() {
        let Some(a) = pair.first() else {
            continue;
        };
        let Some(b) = pair.get(1) else {
            continue;
        };
        let a_lower = a.text.to_lowercase();
        let b_lower = b.text.to_lowercase();
        let Some(negation_signal) = find_matching_term(&a_lower, &config.terms.negation_signals)
        else {
            continue;
        };
        let Some(reframe_signal) = find_matching_term(&b_lower, &config.terms.reframe_signals)
        else {
            continue;
        };

        evidence.push(json!({
            "section_index": section_index,
            "paragraph_index": paragraph_index,
            "sentence_index": sentence_index,
            "sentence_index_next": sentence_index.saturating_add(1),
            "matched_text": format!("{negation_signal} -> {reframe_signal}"),
            "sentence": a.text,
            "next_sentence": b.text,
            "negation_signal": negation_signal,
            "reframe_signal": reframe_signal,
        }));
    }
}

fn find_matching_term<'a>(text: &str, terms: &'a [String]) -> Option<&'a str> {
    terms.iter().map(String::as_str).find(|term| text.contains(term))
}

#[cfg(test)]
#[path = "negation_reframe_tests.rs"]
mod tests;
