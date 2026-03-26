//! Reusable readability-check assertions.

use prosesmasher_app_checks_core_runtime as _;
macro_rules! define_rule_assertions {
    ($check:path, $id:literal, $label:literal) => {
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
            assert_metadata(&$check, $id, $label, None);
        }
    };
}

pub(crate) use define_rule_assertions;

#[path = "read_04_avg_sentence_length.rs"]
pub mod avg_sentence_length;
#[path = "read_03_coleman_liau.rs"]
pub mod coleman_liau;
#[path = "read_01_flesch_kincaid.rs"]
pub mod flesch_kincaid;
#[path = "read_02_gunning_fog.rs"]
pub mod gunning_fog;
