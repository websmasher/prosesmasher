use super::*;
use std::path::Path;

#[test]
fn read_existing_file() {
    let reader = FsFileReader;
    // Read Cargo.toml from the crate root — guaranteed to exist
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let result = reader.read_to_string(&path);
    assert!(result.is_ok(), "should read existing file — got {result:?}");
    let content = result.unwrap_or_default();
    assert!(content.contains("prosesmasher-adapters-outbound-fs"),
        "content should contain crate name");
}

#[test]
fn read_nonexistent_file() {
    let reader = FsFileReader;
    let path = Path::new("/nonexistent/path/to/file.txt");
    let result = reader.read_to_string(path);
    assert!(result.is_err(), "should fail on nonexistent file");
    assert!(
        matches!(result, Err(ReadError::NotFound(_))),
        "should be NotFound error — got {result:?}",
    );
}

#[test]
fn not_found_error_contains_path() {
    let reader = FsFileReader;
    let path = Path::new("/does/not/exist.json");
    if let Err(ReadError::NotFound(msg)) = reader.read_to_string(path) {
        assert!(msg.contains("exist.json"), "error should contain filename — got: {msg}");
    }
}
