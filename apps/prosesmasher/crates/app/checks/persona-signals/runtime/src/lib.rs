//! Persona-signals checks runtime.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

#[cfg(test)]
use prosesmasher_app_checks_persona_signals_assertions as _;

#[cfg(test)]
pub mod test_helpers {
    pub use prosesmasher_app_checks_test_support::*;
}

mod support;

#[path = "heur_14_humble_bragger.rs"]
pub mod humble_bragger;
#[path = "heur_15_jargon_faker.rs"]
pub mod jargon_faker;

pub use humble_bragger::HumbleBraggerCheck;
pub use jargon_faker::JargonFakerCheck;

pub(crate) use support::{
    collect_sentence_phrase_evidence, resolve_humble_bragger_phrases, resolve_jargon_faker_phrases,
    sentence_contains,
};

#[must_use]
pub fn all_checks() -> Vec<check::BoxedCheck> {
    vec![Box::new(HumbleBraggerCheck), Box::new(JargonFakerCheck)]
}
