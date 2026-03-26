use super::*;
use prosesmasher_adapters_outbound_fs_assertions::{
    assert_io_error, assert_not_found_error, assert_permission_denied_error,
};
use prosesmasher_ports_outbound_traits::FileReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

fn unique_temp_path(name: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("prosesmasher-file-reader-{name}-{id}"))
}

#[test]
#[allow(clippy::panic)] // test assertion
fn read_existing_file() {
    let reader = FsFileReader;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    match reader.read_to_string(&path) {
        Ok(content) => {
            assert!(
                content.contains("prosesmasher-adapters-outbound-fs-runtime"),
                "crate name in content"
            );
        }
        Err(e) => panic!("should read existing file — got {e:?}"),
    }
}

#[test]
fn read_nonexistent_file_is_not_found() {
    let reader = FsFileReader;
    let result = reader.read_to_string(Path::new("/nonexistent/path/to/file.txt"));
    assert_not_found_error(
        &result,
        "/nonexistent/path/to/file.txt",
        "missing file should be NotFound",
    );
}

#[test]
fn not_found_error_contains_path() {
    let reader = FsFileReader;
    let result = reader.read_to_string(Path::new("/does/not/exist.json"));
    assert_not_found_error(
        &result,
        "exist.json",
        "not found message should preserve filename",
    );
}

#[test]
#[allow(clippy::panic)] // test assertion
fn read_empty_file_returns_empty_string() {
    let path = unique_temp_path("empty.txt");
    #[allow(clippy::disallowed_methods)]
    std::fs::write(&path, "").unwrap_or_else(|e| panic!("failed to write: {e}"));
    let reader = FsFileReader;
    match reader.read_to_string(&path) {
        Ok(content) => assert_eq!(content, "", "empty content"),
        Err(e) => panic!("empty file should return Ok — got {e:?}"),
    }
    #[allow(clippy::disallowed_methods)]
    let _ = std::fs::remove_file(&path);
}

#[test]
fn read_directory_returns_io_error_with_path() {
    let reader = FsFileReader;
    let path = unique_temp_path("directory");
    let path_str = path.display().to_string();

    #[allow(clippy::disallowed_methods)]
    std::fs::create_dir(&path).unwrap_or_else(|e| panic!("failed to create temp dir: {e}"));
    let result = reader.read_to_string(&path);
    #[allow(clippy::disallowed_methods)]
    std::fs::remove_dir(&path).unwrap_or_else(|e| panic!("failed to remove temp dir: {e}"));

    assert_io_error(
        &result,
        &path_str,
        "directory Io error should preserve path",
    );
}

#[cfg(unix)]
#[test]
fn unreadable_file_returns_permission_denied_with_path() {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let reader = FsFileReader;
    let path = unique_temp_path("permission-denied.txt");
    let path_str = path.display().to_string();

    #[allow(clippy::disallowed_methods)]
    std::fs::write(&path, "secret").unwrap_or_else(|e| panic!("failed to write temp file: {e}"));

    let metadata =
        std::fs::metadata(&path).unwrap_or_else(|e| panic!("failed to stat temp file: {e}"));
    if metadata.uid() == 0 {
        #[allow(clippy::disallowed_methods)]
        std::fs::remove_file(&path).unwrap_or_else(|e| panic!("failed to clean temp file: {e}"));
        return;
    }

    let original_mode = metadata.permissions().mode();
    #[allow(clippy::disallowed_methods)]
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000))
        .unwrap_or_else(|e| panic!("failed to chmod temp file: {e}"));

    let result = reader.read_to_string(&path);

    #[allow(clippy::disallowed_methods)]
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(original_mode))
        .unwrap_or_else(|e| panic!("failed to restore temp file permissions: {e}"));
    #[allow(clippy::disallowed_methods)]
    std::fs::remove_file(&path).unwrap_or_else(|e| panic!("failed to clean temp file: {e}"));

    assert_permission_denied_error(&result, &path_str, "permission denied should preserve path");
}

#[test]
#[allow(clippy::panic)] // test assertion
fn trait_object_works() {
    let reader: &dyn FileReader = &FsFileReader;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    match reader.read_to_string(&path) {
        Ok(content) => assert!(!content.is_empty(), "content through trait object"),
        Err(e) => panic!("trait object should work — got {e:?}"),
    }
}
