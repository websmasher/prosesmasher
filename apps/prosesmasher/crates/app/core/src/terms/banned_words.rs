//! Banned words check — flags configured banned words found in prose.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

/// Checks that none of the configured banned words appear in the document.
///
/// Words are matched case-insensitively against the `banned_words` list
/// from the config. Matches in code blocks and list items are ignored.
#[derive(Debug)]
pub struct BannedWordsCheck;

impl Check for BannedWordsCheck {
    fn id(&self) -> &'static str {
        "banned-words"
    }

    fn label(&self) -> &'static str {
        "Banned Words"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.banned_words.is_empty() {
            return;
        }

        let banned = low_expectations::text::build_term_set(&config.terms.banned_words);
        let mut evidence = Vec::new();
        let mut paragraph_index: usize = 0;

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                collect_banned_word_evidence(
                    block,
                    section_index,
                    &mut paragraph_index,
                    &banned,
                    &mut evidence,
                );
            }
        }

        let observed = unique_matched_texts(&evidence);
        let _result = suite
            .record_custom_values(
                "banned-words",
                evidence.is_empty(),
                json!({ "absent": config.terms.banned_words }),
                json!(observed),
                &evidence,
            )
            .label("Banned Words")
            .checking("AI writing tells");
    }
}

fn collect_banned_word_evidence(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    banned: &std::collections::BTreeSet<String>,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                for word in &sentence.words {
                    let lowered = word.text.to_lowercase();
                    if banned.contains(&lowered) {
                        evidence.push(json!({
                            "section_index": section_index,
                            "paragraph_index": *paragraph_index,
                            "sentence_index": sentence_index,
                            "matched_text": word.text,
                            "sentence": sentence.text,
                        }));
                    }
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_banned_word_evidence(
                    inner_block,
                    section_index,
                    paragraph_index,
                    banned,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn unique_matched_texts(evidence: &[Value]) -> Vec<String> {
    let mut matched = std::collections::BTreeSet::new();

    for item in evidence {
        if let Some(text) = item.get("matched_text").and_then(Value::as_str) {
            let _inserted = matched.insert(text.to_owned());
        }
    }

    matched.into_iter().collect()
}

#[cfg(test)]
#[path = "banned_words_tests.rs"]
mod tests;
