//! Reusable style-signals-check assertions.

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
