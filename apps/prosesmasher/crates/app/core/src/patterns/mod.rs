pub mod affirmation_closers;
pub mod colon_dramatic;
pub mod em_dashes;
pub mod exclamation_density;
pub mod fake_timestamps;
pub mod false_question;
pub mod humble_bragger;
pub mod jargon_faker;
pub mod llm_openers;
pub mod negation_reframe;
pub mod smart_quotes;
pub mod summative_closer;
pub mod triple_repeat;

pub use affirmation_closers::AffirmationClosersCheck;
pub use colon_dramatic::ColonDramaticCheck;
pub use em_dashes::EmDashCheck;
pub use exclamation_density::ExclamationDensityCheck;
pub use fake_timestamps::FakeTimestampCheck;
pub use false_question::FalseQuestionCheck;
pub use humble_bragger::HumbleBraggerCheck;
pub use jargon_faker::JargonFakerCheck;
pub use llm_openers::LlmOpenersCheck;
pub use negation_reframe::NegationReframeCheck;
pub use smart_quotes::SmartQuotesCheck;
pub use summative_closer::SummativeCloserCheck;
pub use triple_repeat::TripleRepeatCheck;

use crate::check::BoxedCheck;

/// All pattern checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![
        Box::new(EmDashCheck),
        Box::new(SmartQuotesCheck),
        Box::new(ExclamationDensityCheck),
        Box::new(NegationReframeCheck),
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
