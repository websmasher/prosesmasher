//! `FileReader` implementation using `std::fs`.

use std::path::Path;

use prosesmasher_domain_types::ReadError;
use prosesmasher_ports_outbound_traits::FileReader;

/// Filesystem-backed file reader.
#[derive(Debug)]
pub struct FsFileReader;

impl FileReader for FsFileReader {
    #[allow(clippy::disallowed_methods)] // reason: this IS the centralized fs module
    fn read_to_string(&self, path: &Path) -> Result<String, ReadError> {
        std::fs::read_to_string(path).map_err(|e| {
            let path_str = path.display().to_string();
            #[allow(clippy::wildcard_enum_match_arm)] // ErrorKind is non-exhaustive
            match e.kind() {
                std::io::ErrorKind::NotFound => ReadError::NotFound(path_str),
                std::io::ErrorKind::PermissionDenied => ReadError::PermissionDenied(path_str),
                _ => ReadError::Io(format!("{path_str}: {e}")),
            }
        })
    }
}

#[cfg(test)]
#[path = "file_reader_tests.rs"]
mod tests;
