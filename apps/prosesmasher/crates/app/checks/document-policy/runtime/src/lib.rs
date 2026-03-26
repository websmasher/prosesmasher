//! Document-policy checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_document_policy_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

#[path = "doc_04_bold_density.rs"]
pub mod bold_density;
#[path = "doc_05_code_fences.rs"]
pub mod code_fences;
#[path = "doc_03_heading_counts.rs"]
pub mod heading_counts;
#[path = "doc_02_heading_hierarchy.rs"]
pub mod heading_hierarchy;
#[path = "doc_01_word_count.rs"]
pub mod word_count;

pub use bold_density::BoldDensityCheck;
pub use code_fences::CodeFencesCheck;
pub use heading_counts::HeadingCountsCheck;
pub use heading_hierarchy::HeadingHierarchyCheck;
pub use word_count::WordCountCheck;

/// All document-policy checks.
#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(WordCountCheck),
        Box::new(HeadingHierarchyCheck),
        Box::new(HeadingCountsCheck),
        Box::new(BoldDensityCheck),
        Box::new(CodeFencesCheck),
    ]
}
