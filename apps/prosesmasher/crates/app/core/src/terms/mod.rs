pub mod banned_words;
pub use banned_words::BannedWordsCheck;

use crate::check::BoxedCheck;

/// All term checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![Box::new(BannedWordsCheck)]
}
