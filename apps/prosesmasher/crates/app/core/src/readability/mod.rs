pub mod avg_sentence_length;
pub mod coleman_liau;
pub mod flesch_kincaid;
pub mod gunning_fog;

pub use avg_sentence_length::AvgSentenceLengthCheck;
pub use coleman_liau::ColemanLiauCheck;
pub use flesch_kincaid::FleschKincaidCheck;
pub use gunning_fog::GunningFogCheck;

use crate::check::BoxedCheck;

/// All readability checks.
#[must_use]
pub fn all_checks() -> Vec<BoxedCheck> {
    vec![
        Box::new(AvgSentenceLengthCheck),
        Box::new(ColemanLiauCheck),
        Box::new(FleschKincaidCheck),
        Box::new(GunningFogCheck),
    ]
}
