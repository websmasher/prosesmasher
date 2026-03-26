//! Style-signals checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_style_signals_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

#[path = "heur_09_colon_dramatic.rs"]
pub mod colon_dramatic;
#[path = "heur_01_em_dashes.rs"]
pub mod em_dashes;
#[path = "heur_04_exclamation_density.rs"]
pub mod exclamation_density;
#[path = "heur_08_fake_timestamps.rs"]
pub mod fake_timestamps;
#[path = "heur_02_sentence_case.rs"]
pub mod sentence_case;
#[path = "heur_03_smart_quotes.rs"]
pub mod smart_quotes;

pub use colon_dramatic::ColonDramaticCheck;
pub use em_dashes::EmDashCheck;
pub use exclamation_density::ExclamationDensityCheck;
pub use fake_timestamps::FakeTimestampCheck;
pub use sentence_case::SentenceCaseCheck;
pub use smart_quotes::SmartQuotesCheck;

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(EmDashCheck),
        Box::new(SentenceCaseCheck),
        Box::new(SmartQuotesCheck),
        Box::new(ExclamationDensityCheck),
        Box::new(FakeTimestampCheck),
        Box::new(ColonDramaticCheck),
    ]
}
