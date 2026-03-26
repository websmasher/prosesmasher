//! LLM-slop checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_llm_slop_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    Vec::new()
}
