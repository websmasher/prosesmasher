//! False-question check — flags rhetorical questions matching known patterns.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};

use crate::check::Check;

/// Detects false/rhetorical questions at section ends that match
/// known pattern phrases (e.g., "And isn't that what we all want?").
#[derive(Debug)]
pub struct FalseQuestionCheck;

impl Check for FalseQuestionCheck {
    fn id(&self) -> &'static str {
        "false-question"
    }

    fn label(&self) -> &'static str {
        "False Question"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.false_question_patterns.is_empty() {
            return;
        }
        let mut count: usize = 0;
        for section in &doc.sections {
            let Some(para) = last_paragraph(&section.blocks) else {
                continue;
            };
            let Some(sentence) = para.sentences.last() else {
                continue;
            };
            if !sentence.text.ends_with('?') {
                continue;
            }
            let lower = sentence.text.to_lowercase();
            let matched = config.terms.false_question_patterns.iter().any(|phrase| {
                lower.contains(&phrase.to_lowercase())
            });
            if matched {
                count = count.saturating_add(1);
            }
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("false-question", count_i64, 0, 0)
            .label("False Question")
            .checking("rhetorical questions matching known patterns");
    }
}

fn last_paragraph(blocks: &[Block]) -> Option<&Paragraph> {
    for block in blocks.iter().rev() {
        match block {
            Block::Paragraph(p) => return Some(p),
            Block::BlockQuote(inner) => {
                if let Some(p) = last_paragraph(inner) {
                    return Some(p);
                }
            }
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
    None
}

#[cfg(test)]
#[path = "false_question_tests.rs"]
mod tests;
