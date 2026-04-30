//! Cadence-patterns checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_cadence_patterns_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

#[path = "heur_08_demonstrative_emphasis.rs"]
pub mod demonstrative_emphasis;
#[path = "heur_06_fragment_stacking.rs"]
pub mod fragment_stacking;
#[path = "heur_05_negation_reframe.rs"]
pub mod negation_reframe;
#[path = "heur_07_triple_repeat.rs"]
pub mod triple_repeat;

pub use demonstrative_emphasis::DemonstrativeEmphasisCheck;
pub use fragment_stacking::FragmentStackingCheck;
pub use negation_reframe::NegationReframeCheck;
pub use triple_repeat::TripleRepeatCheck;

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(NegationReframeCheck),
        Box::new(FragmentStackingCheck),
        Box::new(TripleRepeatCheck),
        Box::new(DemonstrativeEmphasisCheck),
    ]
}
