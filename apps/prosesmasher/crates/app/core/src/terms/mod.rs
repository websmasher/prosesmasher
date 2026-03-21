pub mod banned_phrases;
pub mod banned_words;
pub mod forbidden_terms;
pub mod gendered_terms;
pub mod hedge_words;
pub mod race_terms;
pub mod simplicity;

pub use banned_phrases::BannedPhrasesCheck;
pub use banned_words::BannedWordsCheck;
pub use forbidden_terms::ForbiddenTermsCheck;
pub use gendered_terms::GenderedTermsCheck;
pub use hedge_words::HedgeStackingCheck;
pub use race_terms::RaceTermsCheck;
pub use simplicity::SimplicityCheck;

use prosesmasher_domain_types::Block;

use crate::check::BoxedCheck;

/// All term checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![
        Box::new(BannedWordsCheck),
        Box::new(BannedPhrasesCheck),
        Box::new(GenderedTermsCheck),
        Box::new(ForbiddenTermsCheck),
        Box::new(RaceTermsCheck),
        Box::new(HedgeStackingCheck),
        Box::new(SimplicityCheck),
    ]
}

/// Collect word texts from paragraphs and block quotes.
///
/// Skips code blocks and list items — only prose paragraphs contribute words.
#[must_use]
pub fn collect_paragraph_words(block: &Block) -> Vec<&str> {
    match block {
        Block::Paragraph(p) => p
            .sentences
            .iter()
            .flat_map(|s| s.words.iter().map(|w| w.text.as_str()))
            .collect(),
        Block::BlockQuote(inner) => inner
            .iter()
            .flat_map(|b| collect_paragraph_words(b))
            .collect(),
        Block::List(_) | Block::CodeBlock(_) => Vec::new(),
    }
}
