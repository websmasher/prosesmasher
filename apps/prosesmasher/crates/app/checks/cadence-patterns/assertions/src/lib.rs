//! Reusable cadence-patterns-check assertions.

use prosesmasher_app_checks_core_runtime as _;
macro_rules! define_rule_assertions {
    ($check:path, $id:literal, $label:literal, $locales:expr) => {
        use low_expectations::types::SuiteValidationResult;
        use prosesmasher_app_checks_test_support::result_helpers::{
            assert_fail, assert_metadata, assert_pass, assert_skipped, run_single_check,
        };
        use prosesmasher_domain_types::{CheckConfig, Document};

        pub fn run(doc: &Document, config: &CheckConfig) -> SuiteValidationResult {
            run_single_check(&$check, doc, config)
        }

        pub fn assert_passes(doc: &Document, config: &CheckConfig, message: &str) {
            let result = run(doc, config);
            assert_pass(&result, message);
        }

        pub fn assert_fails(doc: &Document, config: &CheckConfig, message: &str) {
            let result = run(doc, config);
            assert_fail(&result, message);
        }

        pub fn assert_skips(doc: &Document, config: &CheckConfig, message: &str) {
            let result = run(doc, config);
            assert_skipped(&result, message);
        }

        pub fn assert_check_metadata() {
            assert_metadata(&$check, $id, $label, $locales);
        }
    };
}

pub(crate) use define_rule_assertions;

#[path = "heur_06_fragment_stacking.rs"]
pub mod fragment_stacking;
#[path = "heur_05_negation_reframe.rs"]
pub mod negation_reframe;
#[path = "heur_07_triple_repeat.rs"]
pub mod triple_repeat;
