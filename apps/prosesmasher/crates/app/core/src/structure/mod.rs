pub mod word_count;
pub use word_count::WordCountCheck;

use crate::check::BoxedCheck;

/// All structure checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![Box::new(WordCountCheck)]
}
