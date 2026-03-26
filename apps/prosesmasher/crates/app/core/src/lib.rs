//! Compatibility facade over the split check crates.

pub mod check {
    pub use prosesmasher_app_checks_core_runtime::check::*;
}

pub mod runner {
    pub use prosesmasher_app_checks_core_runtime::runner::*;
}

pub mod document_policy {
    pub use prosesmasher_app_checks_document_policy_runtime::*;
}

pub mod quality {
    pub mod lexical {
        pub use prosesmasher_app_checks_lexical_runtime::*;
    }

    pub mod heuristics {
        pub mod cadence_patterns {
            pub use prosesmasher_app_checks_cadence_patterns_runtime::*;
        }

        pub mod llm_slop {
            pub use prosesmasher_app_checks_llm_slop_runtime::*;
        }

        pub mod persona_signals {
            pub use prosesmasher_app_checks_persona_signals_runtime::*;
        }

        pub mod rhetorical_framing {
            pub use prosesmasher_app_checks_rhetorical_framing_runtime::*;
        }

        pub mod style_signals {
            pub use prosesmasher_app_checks_style_signals_runtime::*;
        }
    }

    pub mod flow {
        pub use prosesmasher_app_checks_flow_runtime::*;
    }

    pub mod readability {
        pub use prosesmasher_app_checks_readability_runtime::*;
    }

    use prosesmasher_app_checks_core_runtime::check::BoxedCheck;

    #[must_use]
    pub fn all_checks() -> Vec<BoxedCheck> {
        let mut all = Vec::new();
        all.extend(lexical::all_checks());
        all.extend(prosesmasher_app_checks_style_signals_runtime::all_checks());
        all.extend(prosesmasher_app_checks_cadence_patterns_runtime::all_checks());
        all.extend(prosesmasher_app_checks_rhetorical_framing_runtime::all_checks());
        all.extend(prosesmasher_app_checks_persona_signals_runtime::all_checks());
        all.extend(prosesmasher_app_checks_llm_slop_runtime::all_checks());
        all.extend(flow::all_checks());
        all.extend(readability::all_checks());
        all
    }
}
