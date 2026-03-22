//! Hedge stacking check — flags sentences with too many hedge words.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

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
        if !config.quality.heuristics.hedge_stacking.enabled {
            return;
        }
        let hedge_words = super::resolve_hedge_words(config);
        if hedge_words.is_empty() {
            return;
        }

        let max_hedges = config.quality.heuristics.hedge_stacking.max_per_sentence;
        let hedge_set = low_expectations::text::build_term_set(&hedge_words);
        let mut sentence_idx: usize = 0;
        let mut evidence: Vec<Value> = Vec::new();

        for (section_index, section) in doc.sections.iter().enumerate() {
            for block in &section.blocks {
                check_block(
                    block,
                    section_index,
                    &hedge_set,
                    max_hedges,
                    &mut sentence_idx,
                    &mut evidence,
                );
            }
        }

        let max_allowed =
            i64::try_from(max_hedges.saturating_sub(1)).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "hedge-stacking",
                evidence.is_empty(),
                json!({ "max": max_allowed }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Hedge Stacking")
            .checking("hedge word density per sentence");
    }
}

fn check_block(
    block: &Block,
    section_index: usize,
    hedge_set: &std::collections::BTreeSet<String>,
    max_hedges: usize,
    sentence_idx: &mut usize,
    evidence: &mut Vec<Value>,
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
                if observed > max_allowed {
                    evidence.push(json!({
                        "section_index": section_index,
                        "sentence_index": *sentence_idx,
                        "sentence": sentence.text,
                        "hedge_count": observed,
                        "max_allowed": max_allowed,
                    }));
                }
                *sentence_idx = sentence_idx.saturating_add(1);
            }
        }
        Block::BlockQuote(inner) => {
            for b in inner {
                check_block(
                    b,
                    section_index,
                    hedge_set,
                    max_hedges,
                    sentence_idx,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

#[cfg(test)]
#[path = "hedge_words_tests.rs"]
mod tests;
