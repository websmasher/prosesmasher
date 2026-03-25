//! Heuristic checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_heuristics_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

mod support;

pub mod affirmation_closers;
pub mod colon_dramatic;
pub mod em_dashes;
pub mod exclamation_density;
pub mod fake_timestamps;
pub mod false_question;
pub mod fragment_stacking;
pub mod humble_bragger;
pub mod jargon_faker;
pub mod llm_openers;
pub mod negation_reframe;
pub mod sentence_case;
pub mod smart_quotes;
pub mod summative_closer;
pub mod triple_repeat;

pub use affirmation_closers::AffirmationClosersCheck;
pub use colon_dramatic::ColonDramaticCheck;
pub use em_dashes::EmDashCheck;
pub use exclamation_density::ExclamationDensityCheck;
pub use fake_timestamps::FakeTimestampCheck;
pub use false_question::FalseQuestionCheck;
pub use fragment_stacking::FragmentStackingCheck;
pub use humble_bragger::HumbleBraggerCheck;
pub use jargon_faker::JargonFakerCheck;
pub use llm_openers::LlmOpenersCheck;
pub use negation_reframe::NegationReframeCheck;
pub use sentence_case::SentenceCaseCheck;
pub use smart_quotes::SmartQuotesCheck;
pub use summative_closer::SummativeCloserCheck;
pub use triple_repeat::TripleRepeatCheck;

pub(crate) use support::{
    collect_section_sentence_evidence, collect_sentence_phrase_evidence,
    resolve_affirmation_closers, resolve_false_question_patterns, resolve_humble_bragger_phrases,
    resolve_jargon_faker_phrases, resolve_llm_openers, resolve_summative_patterns,
    section_first_sentence, section_last_sentence, sentence_contains, sentence_ends_with,
    sentence_starts_with,
};

/// All pattern checks.
#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(EmDashCheck),
        Box::new(SentenceCaseCheck),
        Box::new(SmartQuotesCheck),
        Box::new(ExclamationDensityCheck),
        Box::new(NegationReframeCheck),
        Box::new(FragmentStackingCheck),
        Box::new(TripleRepeatCheck),
        Box::new(FakeTimestampCheck),
        Box::new(ColonDramaticCheck),
        Box::new(LlmOpenersCheck),
        Box::new(AffirmationClosersCheck),
        Box::new(SummativeCloserCheck),
        Box::new(FalseQuestionCheck),
        Box::new(HumbleBraggerCheck),
        Box::new(JargonFakerCheck),
    ]
}
