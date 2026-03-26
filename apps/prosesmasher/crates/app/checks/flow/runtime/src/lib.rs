//! Flow checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_flow_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

#[path = "flow_01_paragraph_length.rs"]
pub mod paragraph_length;
#[path = "flow_02_word_repetition.rs"]
pub mod word_repetition;

pub use paragraph_length::ParagraphLengthCheck;
pub use word_repetition::WordRepetitionCheck;

/// All prose-flow checks.
#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(ParagraphLengthCheck),
        Box::new(WordRepetitionCheck),
    ]
}
