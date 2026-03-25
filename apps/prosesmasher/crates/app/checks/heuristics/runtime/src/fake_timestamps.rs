//! Fake-timestamp check — flags suspicious time references like "5:47 PM".

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;

/// Detects fake timestamps (e.g., "5:47 PM") which are a common AI writing tell.
#[derive(Debug)]
pub struct FakeTimestampCheck;

impl Check for FakeTimestampCheck {
    fn id(&self) -> &'static str {
        "fake-timestamps"
    }

    fn label(&self) -> &'static str {
        "Fake Timestamps"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let evidence = collect_fake_timestamp_evidence(doc);
        let _result = suite
            .record_custom_values(
                "fake-timestamps",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Fake Timestamps")
            .checking("suspicious timestamp patterns (digit:digit AM/PM)");
    }
}

fn collect_fake_timestamp_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_fake_timestamp_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_fake_timestamp_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                let timestamps = collect_timestamp_matches(&sentence.text);
                if timestamps.is_empty() {
                    continue;
                }
                evidence.push(json!({
                    "section_index": section_index,
                    "paragraph_index": *paragraph_index,
                    "sentence_index": sentence_index,
                    "matched_text": timestamps.first().cloned().unwrap_or_default(),
                    "timestamps": timestamps,
                    "match_count": timestamps.len(),
                    "sentence": sentence.text,
                }));
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_fake_timestamp_evidence_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

/// Scan text for patterns like "5:47 PM" or "2:30 AM" without regex.
/// Looks for: digit(s) ':' digit(s) space 'AM'/'PM' (case-insensitive).
fn collect_timestamp_matches(text: &str) -> Vec<String> {
    let chars = text.char_indices().collect::<Vec<_>>();
    let mut matches = Vec::new();
    let mut i: usize = 0;

    while i < chars.len() {
        let Some((digit_start_byte, c)) = chars.get(i).copied() else {
            break;
        };
        if !c.is_ascii_digit() {
            i = i.saturating_add(1);
            continue;
        }

        let digit_start = i;
        while chars.get(i).is_some_and(|(_, ch)| ch.is_ascii_digit()) {
            i = i.saturating_add(1);
        }

        if chars.get(i).is_none_or(|(_, ch)| *ch != ':') {
            i = digit_start.saturating_add(1);
            continue;
        }

        i = i.saturating_add(1);
        let minute_start = i;
        while chars.get(i).is_some_and(|(_, ch)| ch.is_ascii_digit()) {
            i = i.saturating_add(1);
        }
        if i == minute_start {
            i = digit_start.saturating_add(1);
            continue;
        }

        if chars.get(i).is_some_and(|(_, ch)| *ch == ' ') {
            i = i.saturating_add(1);
        }

        let Some((_, ch_a)) = chars.get(i).copied() else {
            i = digit_start.saturating_add(1);
            continue;
        };
        let Some((pm_byte, ch_b)) = chars.get(i.saturating_add(1)).copied() else {
            i = digit_start.saturating_add(1);
            continue;
        };
        let a = ch_a.to_ascii_uppercase();
        let b = ch_b.to_ascii_uppercase();
        if (a == 'A' || a == 'P') && b == 'M' {
            let start_byte = digit_start_byte;
            let end_byte = pm_byte.saturating_add(ch_b.len_utf8());
            if let Some(timestamp) = text.get(start_byte..end_byte) {
                matches.push(timestamp.to_owned());
            }
            i = i.saturating_add(2);
        } else {
            i = digit_start.saturating_add(1);
        }
    }

    matches
}

#[cfg(test)]
#[path = "fake_timestamps_tests/mod.rs"]
mod tests;
