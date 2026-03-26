//! Reusable assertions for domain types tests.

use prosesmasher_domain_types_runtime as _;

pub mod config;
pub mod errors;
pub mod lib_root;

pub use config::assert_english_default_quality;
pub use errors::{
    assert_boxed_error_contains, assert_config_error_display, assert_error_source_is_none,
    assert_parse_error_display, assert_read_error_display,
};
pub use lib_root::{
    assert_clone_preserves_display, assert_clone_preserves_display_only, assert_display_contains,
    assert_send_sync,
};
