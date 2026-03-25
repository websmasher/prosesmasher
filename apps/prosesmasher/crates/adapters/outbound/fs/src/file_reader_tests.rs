use super::*;
use prosesmasher_ports_outbound_traits::FileReader;
use std::path::Path;

#[test]
#[allow(clippy::panic)] // test assertion
fn read_existing_file() {
    let reader = FsFileReader;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    match reader.read_to_string(&path) {
        Ok(content) => {
            assert!(
                content.contains("prosesmasher-adapters-outbound-fs"),
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
    assert!(
        matches!(result, Err(ReadError::NotFound(_))),
        "should be NotFound — got {result:?}"
    );
}

#[test]
fn not_found_error_contains_path() {
    let reader = FsFileReader;
    let result = reader.read_to_string(Path::new("/does/not/exist.json"));
    match result {
        Err(ReadError::NotFound(msg)) => {
            assert!(
                msg.contains("exist.json"),
                "should contain filename — got: {msg}"
            );
        }
        other => {
            assert!(
                matches!(other, Err(ReadError::NotFound(_))),
                "expected NotFound, got {other:?}"
            );
        }
    }
}

#[test]
#[allow(clippy::panic)] // test assertion
fn read_empty_file_returns_empty_string() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    let id = CTR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("prosesmasher-test-empty-file-{id}"));
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
fn read_directory_returns_error() {
    let reader = FsFileReader;
    let result = reader.read_to_string(Path::new("/tmp"));
    assert!(
        result.is_err(),
        "reading directory should fail — got {result:?}"
    );
    // Should NOT be NotFound — the path exists, it's just not a file
    // On macOS/Linux this is typically an Io error
    assert!(
        !matches!(result, Err(ReadError::NotFound(_))),
        "directory read should not be NotFound — got {result:?}"
    );
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
