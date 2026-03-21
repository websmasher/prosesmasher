pub mod em_dashes;
pub use em_dashes::EmDashCheck;

use crate::check::BoxedCheck;

/// All pattern checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![Box::new(EmDashCheck)]
}
