pub mod hedge_words;
pub mod prohibited_terms;
pub mod recommended_terms;
pub mod required_terms;
pub mod simplicity;

pub use hedge_words::HedgeStackingCheck;
pub use prohibited_terms::ProhibitedTermsCheck;
pub use recommended_terms::RecommendedTermsCheck;
pub use required_terms::RequiredTermsCheck;
pub use simplicity::SimplicityCheck;

use prosesmasher_domain_types::Block;
use serde_json::{Value, json};

use crate::check::BoxedCheck;

type PhraseTokens = [(Vec<String>, String)];

/// All term checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![
        Box::new(ProhibitedTermsCheck),
        Box::new(HedgeStackingCheck),
        Box::new(SimplicityCheck),
        Box::new(RequiredTermsCheck),
        Box::new(RecommendedTermsCheck),
    ]
}

/// Collect word texts from paragraphs and block quotes.
///
/// Skips code blocks and list items — only prose paragraphs contribute words.
#[must_use]
pub fn collect_paragraph_words(block: &Block) -> Vec<&str> {
    match block {
        Block::Paragraph(p) => p
            .sentences
            .iter()
            .flat_map(|s| s.words.iter().map(|w| w.text.as_str()))
            .collect(),
        Block::BlockQuote(inner) => inner
            .iter()
            .flat_map(|b| collect_paragraph_words(b))
            .collect(),
        Block::List(_) | Block::CodeBlock(_) => Vec::new(),
    }
}

#[must_use]
pub fn resolve_prohibited_terms(config: &prosesmasher_domain_types::CheckConfig) -> Vec<String> {
    let values = resolve_string_override_list(&config.quality.lexical.prohibited_terms);
    dedupe_strings(values)
}

#[must_use]
pub fn resolve_string_override_list(
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
    dedupe_strings(values)
}

#[must_use]
pub fn resolve_simplicity_pairs(
    config: &prosesmasher_domain_types::CheckConfig,
) -> Vec<prosesmasher_domain_types::SimplePair> {
    let mut values = config.quality.lexical.simplicity_pairs.add.clone();
    if !config.quality.lexical.simplicity_pairs.remove.is_empty() {
        let removed: std::collections::BTreeSet<String> = config
            .quality
            .lexical
            .simplicity_pairs
            .remove
            .iter()
            .map(|pair| pair.complex.to_lowercase())
            .collect();
        values.retain(|pair| !removed.contains(&pair.complex.to_lowercase()));
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for pair in values {
        let lowered = pair.complex.to_lowercase();
        if seen.insert(lowered) {
            deduped.push(pair);
        }
    }
    deduped
}

#[must_use]
pub fn resolve_hedge_words(config: &prosesmasher_domain_types::CheckConfig) -> Vec<String> {
    if !config.terms.hedge_words.is_empty() {
        return dedupe_strings(config.terms.hedge_words.clone());
    }

    if config.locale == prosesmasher_domain_types::Locale::En {
        dedupe_strings(vec![
            "might".to_owned(),
            "maybe".to_owned(),
            "perhaps".to_owned(),
            "possibly".to_owned(),
            "likely".to_owned(),
            "probably".to_owned(),
            "seems".to_owned(),
            "apparently".to_owned(),
        ])
    } else {
        Vec::new()
    }
}

#[must_use]
pub fn collect_prohibited_term_evidence(
    doc: &prosesmasher_domain_types::Document,
    prohibited_terms: &[String],
) -> Vec<Value> {
    let banned_words: std::collections::BTreeSet<String> = prohibited_terms
        .iter()
        .filter(|term| term.split_whitespace().count() == 1)
        .map(|term| term.to_lowercase())
        .collect();
    let banned_phrases = low_expectations::text::build_phrase_list(
        &prohibited_terms
            .iter()
            .filter(|term| term.split_whitespace().count() > 1)
            .cloned()
            .collect::<Vec<_>>(),
    );

    let mut evidence = Vec::new();
    let mut paragraph_index: usize = 0;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_prohibited_term_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &banned_words,
                &banned_phrases,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_prohibited_term_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    banned_words: &std::collections::BTreeSet<String>,
    banned_phrases: &PhraseTokens,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                for word in &sentence.words {
                    let lowered = word.text.to_lowercase();
                    if banned_words.contains(&lowered) {
                        evidence.push(json!({
                            "section_index": section_index,
                            "paragraph_index": *paragraph_index,
                            "sentence_index": sentence_index,
                            "matched_text": word.text,
                            "sentence": sentence.text,
                        }));
                    }
                }

                let sentence_words: Vec<String> = sentence
                    .words
                    .iter()
                    .map(|word| word.text.to_lowercase())
                    .collect();

                for (tokens, original) in banned_phrases {
                    if tokens.is_empty() || tokens.len() > sentence_words.len() {
                        continue;
                    }

                    for window_start in 0..=sentence_words.len().saturating_sub(tokens.len()) {
                        let window_end = window_start.saturating_add(tokens.len());
                        if sentence_words
                            .get(window_start..window_end)
                            .is_some_and(|window| window == tokens.as_slice())
                        {
                            evidence.push(json!({
                                "section_index": section_index,
                                "paragraph_index": *paragraph_index,
                                "sentence_index": sentence_index,
                                "matched_text": original,
                                "sentence": sentence.text,
                            }));
                        }
                    }
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_prohibited_term_evidence_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    banned_words,
                    banned_phrases,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[must_use]
pub fn unique_matched_texts(evidence: &[Value]) -> Vec<String> {
    let mut matched = std::collections::BTreeSet::new();

    for item in evidence {
        if let Some(text) = item.get("matched_text").and_then(Value::as_str) {
            let _inserted = matched.insert(text.to_owned());
        }
    }

    matched.into_iter().collect()
}

fn dedupe_strings(values: Vec<String>) -> Vec<String> {
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
