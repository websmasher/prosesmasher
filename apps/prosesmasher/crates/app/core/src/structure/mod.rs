pub mod bold_density;
pub mod code_fences;
pub mod heading_counts;
pub mod heading_hierarchy;
pub mod paragraph_length;
pub mod sentence_case;
pub mod word_count;
pub mod word_repetition;

pub use bold_density::BoldDensityCheck;
pub use code_fences::CodeFencesCheck;
pub use heading_counts::HeadingCountsCheck;
pub use heading_hierarchy::HeadingHierarchyCheck;
pub use paragraph_length::ParagraphLengthCheck;
pub use sentence_case::SentenceCaseCheck;
pub use word_count::WordCountCheck;
pub use word_repetition::WordRepetitionCheck;

use crate::check::BoxedCheck;

/// All structure checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![
        Box::new(BoldDensityCheck),
        Box::new(CodeFencesCheck),
        Box::new(HeadingCountsCheck),
        Box::new(HeadingHierarchyCheck),
        Box::new(ParagraphLengthCheck),
        Box::new(SentenceCaseCheck),
        Box::new(WordCountCheck),
        Box::new(WordRepetitionCheck),
    ]
}
