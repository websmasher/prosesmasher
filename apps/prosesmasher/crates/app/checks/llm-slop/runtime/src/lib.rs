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

mod support;

#[path = "slop_01_llm_disclaimer.rs"]
pub mod llm_disclaimer;
#[path = "slop_02_response_wrapper.rs"]
pub mod response_wrapper;
#[path = "slop_03_generic_signposting.rs"]
pub mod generic_signposting;

pub use llm_disclaimer::LlmDisclaimerCheck;
pub use generic_signposting::GenericSignpostingCheck;
pub use response_wrapper::ResponseWrapperCheck;

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(LlmDisclaimerCheck),
        Box::new(ResponseWrapperCheck),
        Box::new(GenericSignpostingCheck),
    ]
}
