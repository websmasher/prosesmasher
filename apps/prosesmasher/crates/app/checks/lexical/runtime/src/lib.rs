//! Lexical checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_lexical_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

mod support;

#[path = "lex_02_hedge_stacking.rs"]
pub mod hedge_words;
#[path = "lex_01_prohibited_terms.rs"]
pub mod prohibited_terms;
#[path = "lex_05_recommended_terms.rs"]
pub mod recommended_terms;
#[path = "lex_04_required_terms.rs"]
pub mod required_terms;
#[path = "lex_03_simplicity.rs"]
pub mod simplicity;

pub use hedge_words::HedgeStackingCheck;
pub use prohibited_terms::ProhibitedTermsCheck;
pub use recommended_terms::RecommendedTermsCheck;
pub use required_terms::RequiredTermsCheck;
pub use simplicity::SimplicityCheck;

pub(crate) use support::{
    collect_paragraph_words, collect_prohibited_term_evidence, resolve_hedge_words,
    resolve_prohibited_terms, resolve_simplicity_pairs, unique_matched_texts,
};

/// All term checks.
#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![
        Box::new(ProhibitedTermsCheck),
        Box::new(HedgeStackingCheck),
        Box::new(SimplicityCheck),
        Box::new(RequiredTermsCheck),
        Box::new(RecommendedTermsCheck),
    ]
}
