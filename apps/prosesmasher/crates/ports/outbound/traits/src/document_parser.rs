use prosesmasher_domain_types::{Document, Locale, ParseError};

/// Port for parsing markdown into a structured document.
pub trait DocumentParser {
    /// Parse markdown text with the given locale into a Document.
    ///
    /// # Errors
    ///
    /// Returns `ParseError::InvalidMarkdown` if the input cannot be parsed,
    /// or `ParseError::SegmentationFailed` if sentence/word segmentation fails.
    fn parse(&self, markdown: &str, locale: &Locale) -> Result<Document, ParseError>;
}
