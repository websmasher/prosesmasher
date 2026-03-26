//! Reusable document-policy-check assertions.

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

#[path = "doc_04_bold_density.rs"]
pub mod bold_density;
#[path = "doc_05_code_fences.rs"]
pub mod code_fences;
#[path = "doc_03_heading_counts.rs"]
pub mod heading_counts;
#[path = "doc_02_heading_hierarchy.rs"]
pub mod heading_hierarchy;
#[path = "doc_01_word_count.rs"]
pub mod word_count;
