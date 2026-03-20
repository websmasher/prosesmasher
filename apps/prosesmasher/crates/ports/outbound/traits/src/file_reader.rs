use std::path::Path;

use prosesmasher_domain_types::ReadError;

/// Port for reading file contents.
pub trait FileReader {
    /// Read file at the given path to a string.
    ///
    /// # Errors
    ///
    /// Returns `ReadError::NotFound` if the file does not exist,
    /// `ReadError::PermissionDenied` if access is denied,
    /// or `ReadError::Io` for other I/O failures.
    fn read_to_string(&self, path: &Path) -> Result<String, ReadError>;
}
