//! Core prose quality checks.

pub mod flow;
pub mod heuristics;
pub mod lexical;
pub mod readability;

use crate::check::BoxedCheck;

/// All core prose quality checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    let mut all = Vec::new();
    all.extend(lexical::all_checks());
    all.extend(heuristics::all_checks());
    all.extend(flow::all_checks());
    all.extend(readability::all_checks());
    all
}
