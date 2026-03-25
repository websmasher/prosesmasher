//! Reusable assertions and temp-fixture helpers for the filesystem adapter tests.

mod support;

pub mod config_loader;
pub mod file_reader;

pub use config_loader::{load_json_err, load_json_ok, load_preset_ok};
pub use file_reader::{assert_read_error_kind, assert_read_ok_contains, write_temp_file};
