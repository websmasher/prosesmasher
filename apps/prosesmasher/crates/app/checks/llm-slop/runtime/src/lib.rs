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

#[path = "slop_12_authority_padding.rs"]
pub mod authority_padding;
#[path = "slop_11_blame_reframe.rs"]
pub mod blame_reframe;
#[path = "slop_08_boilerplate_conclusion.rs"]
pub mod boilerplate_conclusion;
#[path = "slop_04_boilerplate_framing.rs"]
pub mod boilerplate_framing;
#[path = "slop_10_contrastive_aphorism.rs"]
pub mod contrastive_aphorism;
#[path = "slop_09_empty_emphasis.rs"]
pub mod empty_emphasis;
#[path = "slop_03_generic_signposting.rs"]
pub mod generic_signposting;
#[path = "slop_13_lesson_framing.rs"]
pub mod lesson_framing;
#[path = "slop_01_llm_disclaimer.rs"]
pub mod llm_disclaimer;
#[path = "slop_05_llm_vocabulary.rs"]
pub mod llm_vocabulary;
#[path = "slop_14_observer_guidance.rs"]
pub mod observer_guidance;
#[path = "slop_02_response_wrapper.rs"]
pub mod response_wrapper;
#[path = "slop_06_softening_language.rs"]
pub mod softening_language;
#[path = "slop_07_universalizing_claims.rs"]
pub mod universalizing_claims;

pub use authority_padding::AuthorityPaddingCheck;
pub use blame_reframe::BlameReframeCheck;
pub use boilerplate_conclusion::BoilerplateConclusionCheck;
pub use boilerplate_framing::BoilerplateFramingCheck;
pub use contrastive_aphorism::ContrastiveAphorismCheck;
pub use empty_emphasis::EmptyEmphasisCheck;
pub use generic_signposting::GenericSignpostingCheck;
pub use lesson_framing::LessonFramingCheck;
pub use llm_disclaimer::LlmDisclaimerCheck;
pub use llm_vocabulary::LlmVocabularyCheck;
pub use observer_guidance::ObserverGuidanceCheck;
pub use response_wrapper::ResponseWrapperCheck;
pub use softening_language::SofteningLanguageCheck;
pub use universalizing_claims::UniversalizingClaimsCheck;

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(LlmDisclaimerCheck),
        Box::new(ResponseWrapperCheck),
        Box::new(LessonFramingCheck),
        Box::new(ObserverGuidanceCheck),
        Box::new(GenericSignpostingCheck),
        Box::new(BoilerplateFramingCheck),
        Box::new(BoilerplateConclusionCheck),
        Box::new(BlameReframeCheck),
        Box::new(AuthorityPaddingCheck),
        Box::new(EmptyEmphasisCheck),
        Box::new(ContrastiveAphorismCheck),
        Box::new(LlmVocabularyCheck),
        Box::new(SofteningLanguageCheck),
        Box::new(UniversalizingClaimsCheck),
    ]
}
