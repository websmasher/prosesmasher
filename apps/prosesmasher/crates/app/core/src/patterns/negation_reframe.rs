//! Negation-reframe check — flags "not X. It's Y." sentence pairs.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};

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
        let mut match_count: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, config, &mut match_count);
        }
        let count_i64 = i64::try_from(match_count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("negation-reframe", count_i64, 0, 0)
            .label("Negation-Reframe Pattern")
            .checking("consecutive negation + reframe sentence pairs");
    }
}

fn check_blocks(blocks: &[Block], config: &CheckConfig, count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => check_paragraph(p, config, count),
            Block::BlockQuote(inner) => check_blocks(inner, config, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

fn check_paragraph(para: &Paragraph, config: &CheckConfig, count: &mut usize) {
    if para.sentences.len() < 2 {
        return;
    }
    let mut idx: usize = 0;
    loop {
        let next = idx.saturating_add(1);
        if next >= para.sentences.len() {
            break;
        }
        let Some(a) = para.sentences.get(idx) else {
            break;
        };
        let Some(b) = para.sentences.get(next) else {
            break;
        };
        let a_lower = a.text.to_lowercase();
        let b_lower = b.text.to_lowercase();
        let has_negation = config.terms.negation_signals.iter().any(|sig| {
            a_lower.contains(&sig.to_lowercase())
        });
        let has_reframe = config.terms.reframe_signals.iter().any(|sig| {
            b_lower.contains(&sig.to_lowercase())
        });
        if has_negation && has_reframe {
            *count = count.saturating_add(1);
        }
        idx = next;
    }
}

#[cfg(test)]
#[path = "negation_reframe_tests.rs"]
mod tests;
