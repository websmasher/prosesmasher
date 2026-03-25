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

/// All document-policy checks.
#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(BoldDensityCheck),
        Box::new(CodeFencesCheck),
        Box::new(HeadingCountsCheck),
        Box::new(HeadingHierarchyCheck),
        Box::new(WordCountCheck),
    ]
}
