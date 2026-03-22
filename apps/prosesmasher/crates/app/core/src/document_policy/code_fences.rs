//! Code fences check — flags documents that contain code blocks.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

/// Checks that the document contains no code blocks.
#[derive(Debug)]
pub struct CodeFencesCheck;

impl Check for CodeFencesCheck {
    fn id(&self) -> &'static str {
        "code-fences"
    }

    fn label(&self) -> &'static str {
        "Code Fences"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.document_policy.allow_code_fences {
            return;
        }

        let mut evidence = Vec::new();

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                collect_code_block_evidence(block, section_index, &mut evidence);
            }
        }

        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "code-fences",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0, "rule": "no code blocks in prose content" }),
                json!({ "count": observed }),
                &evidence,
            )
            .label("Code Fences")
            .checking("code block count");
    }
}

fn collect_code_block_evidence(block: &Block, section_index: usize, evidence: &mut Vec<Value>) {
    match block {
        Block::CodeBlock(_) => {
            evidence.push(json!({
                "section_index": section_index,
                "code_block_text": block_code_text(block),
                "issue": "code block in prose",
            }));
        }
        Block::BlockQuote(blocks) => {
            for inner in blocks {
                collect_code_block_evidence(inner, section_index, evidence);
            }
        }
        Block::Paragraph(_) | Block::List(_) => {}
    }
}

fn block_code_text(block: &Block) -> String {
    match block {
        Block::CodeBlock(code) => code.clone(),
        Block::BlockQuote(_) | Block::Paragraph(_) | Block::List(_) => String::new(),
    }
}

#[cfg(test)]
#[path = "code_fences_tests.rs"]
mod tests;
