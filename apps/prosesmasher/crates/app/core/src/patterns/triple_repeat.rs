//! Triple-repeat check — flags three consecutive sentences starting with the same word.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};

use crate::check::Check;

/// Detects paragraphs where three consecutive sentences begin with the same word.
#[derive(Debug)]
pub struct TripleRepeatCheck;

impl Check for TripleRepeatCheck {
    fn id(&self) -> &'static str {
        "triple-repeat"
    }

    fn label(&self) -> &'static str {
        "Triple Repeat Opener"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut match_count: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, &mut match_count);
        }
        let count_i64 = i64::try_from(match_count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("triple-repeat", count_i64, 0, 0)
            .label("Triple Repeat Opener")
            .checking("three consecutive sentences starting with the same word");
    }
}

fn check_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => check_paragraph(p, count),
            Block::BlockQuote(inner) => check_blocks(inner, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

fn check_paragraph(para: &Paragraph, count: &mut usize) {
    if para.sentences.len() < 3 {
        return;
    }
    let first_words: Vec<String> = para
        .sentences
        .iter()
        .filter_map(|s| s.words.first().map(|w| w.text.to_lowercase()))
        .collect();

    if first_words.len() < 3 {
        return;
    }

    let mut idx: usize = 0;
    loop {
        let mid = idx.saturating_add(1);
        let end = idx.saturating_add(2);
        if end >= first_words.len() {
            break;
        }
        let Some(a) = first_words.get(idx) else {
            break;
        };
        let Some(b) = first_words.get(mid) else {
            break;
        };
        let Some(c) = first_words.get(end) else {
            break;
        };
        if a == b && b == c {
            *count = count.saturating_add(1);
        }
        idx = mid;
    }
}

#[cfg(test)]
#[path = "triple_repeat_tests.rs"]
mod tests;
