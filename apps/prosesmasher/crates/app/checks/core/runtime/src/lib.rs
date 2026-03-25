//! Shared check contracts and runner.

#[cfg(test)]
use prosesmasher_app_checks_core_assertions as _;

pub mod check;
pub mod runner;

#[cfg(test)]
pub(crate) mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

pub use check::{BoxedCheck, Check};
pub use runner::run_checks;
