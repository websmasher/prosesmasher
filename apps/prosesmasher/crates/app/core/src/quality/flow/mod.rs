pub mod paragraph_length;
pub mod word_repetition;

pub use paragraph_length::ParagraphLengthCheck;
pub use word_repetition::WordRepetitionCheck;

use crate::check::BoxedCheck;

/// All prose-flow checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![Box::new(ParagraphLengthCheck), Box::new(WordRepetitionCheck)]
}
