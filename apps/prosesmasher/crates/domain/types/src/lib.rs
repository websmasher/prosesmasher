//! Domain types for prosesmasher.

pub mod config;
pub mod document;
pub mod error;
pub mod locale;
pub mod metadata;

pub use config::{
    CheckConfig, DocumentPolicyConfig, EnabledCheck, ExclamationDensityConfig,
    HeadingCountsPolicy, HedgeStackingConfig, HeuristicsConfig, LexicalConfig, OverrideList,
    ParagraphLengthConfig, QualityConfig, Range, ReadabilityConfig, SimplePair, TermPool,
    WordRepetitionConfig, default_quality_for_locale,
};
pub use document::{Block, Document, Heading, Link, ListBlock, Paragraph, Section, Sentence, Word};
pub use error::{ConfigError, ParseError, ReadError};
pub use locale::Locale;
pub use metadata::{DocumentMetadata, HeadingCounts};

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
