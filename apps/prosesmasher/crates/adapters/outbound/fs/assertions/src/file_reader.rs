use std::path::{Path, PathBuf};

use prosesmasher_adapters_outbound_fs_runtime::FsFileReader;
use prosesmasher_domain_types_runtime::ReadError;
use prosesmasher_ports_outbound_traits::FileReader;

use crate::support::{cleanup, write_temp};

#[must_use]
pub fn write_temp_file(name: &str, content: &str) -> PathBuf {
    write_temp(name, content)
}

#[allow(clippy::panic)]
pub fn assert_read_ok_contains(path: &Path, expected: &str, context: &str) {
    match FsFileReader.read_to_string(path) {
        Ok(content) => {
            assert!(
                content.contains(expected),
                "{context}: expected file content to contain `{expected}`"
            );
        }
        Err(err) => panic!("{context}: expected Ok, got Err: {err:?}"),
    }
}

pub fn assert_read_error_kind(path: &Path, predicate: impl FnOnce(&ReadError) -> bool, context: &str) {
    let result = FsFileReader.read_to_string(path);
    assert!(
        matches!(result, Err(ref err) if predicate(err)),
        "{context}: unexpected result {result:?}"
    );
}

pub fn cleanup_temp_file(path: &Path) {
    cleanup(path);
}
