//! Banned phrases check — flags configured multi-word phrases found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

type PhraseTokens = [(Vec<String>, String)];

/// Checks that none of the configured banned phrases appear in the document.
///
/// Phrases are matched via sliding-window over contiguous words,
/// case-insensitively. Matches in code blocks and list items are ignored.
#[derive(Debug)]
pub struct BannedPhrasesCheck;

impl Check for BannedPhrasesCheck {
    fn id(&self) -> &'static str {
        "banned-phrases"
    }

    fn label(&self) -> &'static str {
        "Banned Phrases"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.banned_phrases.is_empty() {
            return;
        }

        let phrase_tokens = low_expectations::text::build_phrase_list(&config.terms.banned_phrases);
        let mut evidence = Vec::new();
        let mut paragraph_index: usize = 0;

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                collect_banned_phrase_evidence(
                    block,
                    section_index,
                    &mut paragraph_index,
                    &phrase_tokens,
                    &mut evidence,
                );
            }
        }

        let observed = unique_matched_phrases(&evidence);
        let _result = suite
            .record_custom_values(
                "banned-phrases",
                evidence.is_empty(),
                json!({ "absent": config.terms.banned_phrases }),
                json!(observed),
                &evidence,
            )
            .label("Banned Phrases")
            .checking("AI writing tells");
    }
}

fn collect_banned_phrase_evidence(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    phrases: &PhraseTokens,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                let sentence_words: Vec<String> = sentence
                    .words
                    .iter()
                    .map(|word| word.text.to_lowercase())
                    .collect();

                for (tokens, original) in phrases {
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
                collect_banned_phrase_evidence(
                    inner_block,
                    section_index,
                    paragraph_index,
                    phrases,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn unique_matched_phrases(evidence: &[Value]) -> Vec<String> {
    let mut matched = std::collections::BTreeSet::new();

    for item in evidence {
        if let Some(text) = item.get("matched_text").and_then(Value::as_str) {
            let _inserted = matched.insert(text.to_owned());
        }
    }

    matched.into_iter().collect()
}

#[cfg(test)]
#[path = "banned_phrases_tests.rs"]
mod tests;
