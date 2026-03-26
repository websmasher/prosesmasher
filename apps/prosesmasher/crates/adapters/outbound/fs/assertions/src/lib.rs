//! Reusable assertions and temp-fixture helpers for the filesystem adapter tests.

mod support;

pub mod config_loader;
pub mod file_reader;

pub use config_loader::{
    assert_config_not_found_contains, assert_config_validation_failed_contains,
    assert_heading_policy, assert_locale, assert_prohibited_terms, assert_readability_thresholds,
    assert_recommended_terms, assert_shared_quality_defaults, assert_simplicity_pair,
    assert_word_count_range, load_json_err, load_json_ok, load_preset_ok,
};
pub use file_reader::{
    assert_io_error, assert_not_found_error, assert_permission_denied_error,
    assert_read_error_kind, assert_read_ok_contains, write_temp_file,
};
