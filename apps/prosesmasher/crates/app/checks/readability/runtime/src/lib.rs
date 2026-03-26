//! Readability checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_readability_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

#[path = "read_04_avg_sentence_length.rs"]
pub mod avg_sentence_length;
#[path = "read_03_coleman_liau.rs"]
pub mod coleman_liau;
#[path = "read_01_flesch_kincaid.rs"]
pub mod flesch_kincaid;
#[path = "read_02_gunning_fog.rs"]
pub mod gunning_fog;

pub use avg_sentence_length::AvgSentenceLengthCheck;
pub use coleman_liau::ColemanLiauCheck;
pub use flesch_kincaid::FleschKincaidCheck;
pub use gunning_fog::GunningFogCheck;

/// All readability checks.
#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(FleschKincaidCheck),
        Box::new(GunningFogCheck),
        Box::new(ColemanLiauCheck),
        Box::new(AvgSentenceLengthCheck),
    ]
}
