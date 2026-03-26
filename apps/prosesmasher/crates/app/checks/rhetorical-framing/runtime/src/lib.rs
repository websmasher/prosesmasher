//! Rhetorical-framing checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_rhetorical_framing_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

mod support;

#[path = "heur_11_affirmation_closers.rs"]
pub mod affirmation_closers;
#[path = "heur_13_false_question.rs"]
pub mod false_question;
#[path = "heur_10_llm_openers.rs"]
pub mod llm_openers;
#[path = "heur_12_summative_closer.rs"]
pub mod summative_closer;

pub use affirmation_closers::AffirmationClosersCheck;
pub use false_question::FalseQuestionCheck;
pub use llm_openers::LlmOpenersCheck;
pub use summative_closer::SummativeCloserCheck;

pub(crate) use support::{
    collect_section_sentence_evidence, resolve_affirmation_closers,
    resolve_false_question_patterns, resolve_llm_openers, resolve_summative_patterns,
    section_first_sentence, section_last_sentence, sentence_ends_with, sentence_starts_with,
};

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(LlmOpenersCheck),
        Box::new(AffirmationClosersCheck),
        Box::new(SummativeCloserCheck),
        Box::new(FalseQuestionCheck),
    ]
}
