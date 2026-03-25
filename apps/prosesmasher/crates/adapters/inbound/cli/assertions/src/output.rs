use std::path::Path;

use low_expectations::types::SuiteValidationResult;
use prosesmasher_adapters_inbound_cli_runtime::output::{FileResult, build_file_result, format_line};

#[must_use]
pub fn format(success: bool, label: &str, observed: &str) -> String {
    format_line(success, label, observed)
}

#[must_use]
pub fn build(path: &Path, result: &SuiteValidationResult, include_checks: bool) -> FileResult {
    build_file_result(path, result, include_checks)
}
