//! Hedge stacking check — flags sentences with too many hedge words.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that no single sentence contains too many hedge words.
///
/// For each sentence, counts how many words appear in the configured
/// `hedge_words` list. If the count meets or exceeds the threshold
/// (`max_hedges_per_sentence`, default 2), the sentence is flagged.
#[derive(Debug)]
pub struct HedgeStackingCheck;

impl Check for HedgeStackingCheck {
    fn id(&self) -> &'static str {
        "hedge-stacking"
    }

    fn label(&self) -> &'static str {
        "Hedge Stacking"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.terms.hedge_words.is_empty() {
            return;
        }

        let max_hedges = config.thresholds.max_hedges_per_sentence.unwrap_or(2);
        let hedge_set = low_expectations::text::build_term_set(&config.terms.hedge_words);
        let mut sentence_idx: usize = 0;

        for section in &doc.sections {
            for block in &section.blocks {
                check_block(block, &hedge_set, max_hedges, suite, &mut sentence_idx);
            }
        }
    }
}

fn check_block(
    block: &Block,
    hedge_set: &std::collections::BTreeSet<String>,
    max_hedges: usize,
    suite: &mut ExpectationSuite,
    sentence_idx: &mut usize,
) {
    match block {
        Block::Paragraph(p) => {
            for sentence in &p.sentences {
                let hedge_count: usize = sentence
                    .words
                    .iter()
                    .filter(|w| hedge_set.contains(&w.text.to_lowercase()))
                    .count();

                // max_hedges is the threshold: sentences with fewer hedges pass.
                // We want 0..max_hedges-1 to pass, max_hedges+ to fail.
                // expect_value_to_be_between checks observed in [min, max] inclusive.
                let observed = i64::try_from(hedge_count).unwrap_or(i64::MAX);
                let max_allowed =
                    i64::try_from(max_hedges.saturating_sub(1)).unwrap_or(i64::MAX);
                let col = format!("hedge-stacking-s{sentence_idx}");
                let _result = suite
                    .expect_value_to_be_between(&col, observed, 0, max_allowed)
                    .label("Hedge Stacking")
                    .checking("hedge word density per sentence");
                *sentence_idx = sentence_idx.saturating_add(1);
            }
        }
        Block::BlockQuote(inner) => {
            for b in inner {
                check_block(b, hedge_set, max_hedges, suite, sentence_idx);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[cfg(test)]
#[path = "hedge_words_tests.rs"]
mod tests;
