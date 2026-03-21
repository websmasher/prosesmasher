//! Fake-timestamp check — flags suspicious time references like "5:47 PM".

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};

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
        let mut count: usize = 0;
        for section in &doc.sections {
            check_blocks(&section.blocks, &mut count);
        }
        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
        let _result = suite
            .expect_value_to_be_between("fake-timestamps", count_i64, 0, 0)
            .label("Fake Timestamps")
            .checking("suspicious timestamp patterns (digit:digit AM/PM)");
    }
}

fn check_blocks(blocks: &[Block], count: &mut usize) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                for sentence in &p.sentences {
                    if contains_timestamp(&sentence.text) {
                        *count = count.saturating_add(1);
                    }
                }
            }
            Block::BlockQuote(inner) => check_blocks(inner, count),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

/// Scan text for patterns like "5:47 PM" or "2:30 AM" without regex.
/// Looks for: digit(s) ':' digit(s) space 'AM'/'PM' (case-insensitive).
fn contains_timestamp(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i: usize = 0;
    while i < len {
        let Some(&c) = chars.get(i) else {
            break;
        };
        // Look for a digit that could start a timestamp
        if c.is_ascii_digit() {
            // Scan digits before colon
            let start = i;
            while i < len {
                match chars.get(i) {
                    Some(ch) if ch.is_ascii_digit() => {
                        i = i.saturating_add(1);
                    }
                    _ => break,
                }
            }
            // Check for colon
            if i < len && chars.get(i) == Some(&':') {
                i = i.saturating_add(1);
                // Check for digits after colon
                let digit_start = i;
                while i < len {
                    match chars.get(i) {
                        Some(ch) if ch.is_ascii_digit() => {
                            i = i.saturating_add(1);
                        }
                        _ => break,
                    }
                }
                if i > digit_start {
                    // Skip optional space
                    if i < len && chars.get(i) == Some(&' ') {
                        i = i.saturating_add(1);
                    }
                    // Check for AM/PM
                    if let (Some(ch_a), Some(ch_b)) =
                        (chars.get(i), chars.get(i.saturating_add(1)))
                    {
                        let a = ch_a.to_ascii_uppercase();
                        let b = ch_b.to_ascii_uppercase();
                        if (a == 'A' || a == 'P') && b == 'M' {
                            return true;
                        }
                    }
                }
            }
            // If we didn't find a timestamp, continue from after start digit
            i = start.saturating_add(1);
        } else {
            i = i.saturating_add(1);
        }
    }
    false
}

#[cfg(test)]
#[path = "fake_timestamps_tests.rs"]
mod tests;
