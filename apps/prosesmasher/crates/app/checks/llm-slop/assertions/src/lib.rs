//! Reusable llm-slop-check assertions.

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

#[path = "slop_01_llm_disclaimer.rs"]
pub mod llm_disclaimer;
#[path = "slop_02_response_wrapper.rs"]
pub mod response_wrapper;
#[path = "slop_03_generic_signposting.rs"]
pub mod generic_signposting;
#[path = "slop_04_boilerplate_framing.rs"]
pub mod boilerplate_framing;
#[path = "slop_05_llm_vocabulary.rs"]
pub mod llm_vocabulary;
