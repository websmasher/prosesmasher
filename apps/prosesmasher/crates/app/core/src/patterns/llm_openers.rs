//! LLM-openers check — flags section openers that match known LLM patterns.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};

use crate::check::Check;

/// Detects sections that open with common LLM phrases
/// (e.g., "The interesting part is that...").
#[derive(Debug)]
pub struct LlmOpenersCheck;

impl Check for LlmOpenersCheck {
    fn id(&self) -> &'static str {
        "llm-openers"
    }

    fn label(&self) -> &'static str {
        "LLM Openers"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.llm_openers.is_empty() {
            return;
        }
        let mut count: usize = 0;
        for section in &doc.sections {
            let Some(para) = first_paragraph(&section.blocks) else {
                continue;
            };
            let Some(sentence) = para.sentences.first() else {
                continue;
            };
            let lower = sentence.text.to_lowercase();
            let matched = config.terms.llm_openers.iter().any(|phrase| {
                lower.starts_with(&phrase.to_lowercase())
            });
            if matched {
                count = count.saturating_add(1);
            }
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("llm-openers", count_i64, 0, 0)
            .label("LLM Openers")
            .checking("section openers matching LLM patterns");
    }
}

fn first_paragraph(blocks: &[Block]) -> Option<&Paragraph> {
    for block in blocks {
        match block {
            Block::Paragraph(p) => return Some(p),
            Block::BlockQuote(inner) => {
                if let Some(p) = first_paragraph(inner) {
                    return Some(p);
                }
            }
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
    None
}

#[cfg(test)]
#[path = "llm_openers_tests.rs"]
mod tests;
