//! Markdown parser adapter — `DocumentParser` implementation.

mod html_text;
mod markdown;
mod segmenter;
mod syllables;

use prosesmasher_domain_types::{Document, Locale, ParseError};
use prosesmasher_ports_outbound_traits::DocumentParser;

/// Markdown parser that implements the `DocumentParser` port.
///
/// Delegates to pulldown-cmark for markdown parsing, ICU4X for
/// sentence/word segmentation, and the hyphenation crate for
/// syllable counting.
#[derive(Debug)]
pub struct MarkdownParser;

impl DocumentParser for MarkdownParser {
    fn parse(&self, markdown: &str, locale: &Locale) -> Result<Document, ParseError> {
        Ok(markdown::parse_markdown(markdown, *locale))
    }
}

#[cfg(test)]
mod lib_tests;
