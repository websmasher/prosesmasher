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

#[path = "slop_01_llm_disclaimer.rs"]
pub mod llm_disclaimer;

pub use llm_disclaimer::LlmDisclaimerCheck;

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![Box::new(LlmDisclaimerCheck)]
}
