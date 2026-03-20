//! Domain types for prosesmasher.

pub mod config;
pub mod document;
pub mod error;
pub mod locale;
pub mod metadata;

pub use config::{CheckConfig, Range, SimplePair, TermLists, Thresholds};
pub use document::{Block, Document, Heading, Link, ListBlock, Paragraph, Section, Sentence, Word};
pub use error::{ConfigError, ParseError, ReadError};
pub use locale::Locale;
pub use metadata::{DocumentMetadata, HeadingCounts};

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
