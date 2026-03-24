pub mod bold_density;
pub mod code_fences;
pub mod heading_counts;
pub mod heading_hierarchy;
pub mod word_count;

pub use bold_density::BoldDensityCheck;
pub use code_fences::CodeFencesCheck;
pub use heading_counts::HeadingCountsCheck;
pub use heading_hierarchy::HeadingHierarchyCheck;
pub use word_count::WordCountCheck;

use crate::check::BoxedCheck;

/// All document-policy checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![
        Box::new(BoldDensityCheck),
        Box::new(CodeFencesCheck),
        Box::new(HeadingCountsCheck),
        Box::new(HeadingHierarchyCheck),
        Box::new(WordCountCheck),
    ]
}
