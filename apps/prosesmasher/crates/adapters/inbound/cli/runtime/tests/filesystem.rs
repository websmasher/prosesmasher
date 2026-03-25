use clap as _;
use low_expectations as _;
use prosesmasher_adapters_inbound_cli_assertions as _;
use prosesmasher_adapters_outbound_fs as _;
use prosesmasher_adapters_outbound_parser as _;
use prosesmasher_app_checks_catalog_runtime as _;
use prosesmasher_app_checks_core_runtime as _;
use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
use serde as _;
use serde_json as _;
use std::path::{Path, PathBuf};
use walkdir as _;

use prosesmasher_adapters_inbound_cli_runtime::collect_files;

#[test]
fn single_file_returns_that_file() {
    let path = Path::new("some/file.md");
    let files = collect_files(path);
    assert_eq!(files.len(), 1, "single file should return one entry");
    assert_eq!(
        files.first().map(PathBuf::as_path),
        Some(path),
        "should be the same path"
    );
}

#[test]
fn single_file_non_md_still_returned() {
    // User explicitly chose a non-.md file — we still return it
    let path = Path::new("readme.txt");
    let files = collect_files(path);
    assert_eq!(files.len(), 1, "non-md single file still returned");
}

#[test]
#[allow(clippy::panic, clippy::disallowed_methods, clippy::disallowed_types)] // test: temp dir setup
fn directory_with_md_files() {
    let dir = std::env::temp_dir().join("prosesmasher-test-collect-md");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("mkdir: {e}"));
    std::fs::write(dir.join("a.md"), "# A").unwrap_or_else(|e| panic!("write a: {e}"));
    std::fs::write(dir.join("b.md"), "# B").unwrap_or_else(|e| panic!("write b: {e}"));

    let files = collect_files(&dir);
    assert_eq!(files.len(), 2, "should find 2 md files");
    // Should be sorted
    let names: Vec<&str> = files
        .iter()
        .filter_map(|p| p.file_name())
        .filter_map(|n| n.to_str())
        .collect();
    assert_eq!(names, vec!["a.md", "b.md"], "sorted order");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[allow(clippy::panic, clippy::disallowed_methods, clippy::disallowed_types)] // test: temp dir setup
fn empty_directory_returns_empty() {
    let dir = std::env::temp_dir().join("prosesmasher-test-collect-empty");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("mkdir: {e}"));

    let files = collect_files(&dir);
    assert!(files.is_empty(), "empty dir should return empty vec");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[allow(clippy::panic, clippy::disallowed_methods, clippy::disallowed_types)] // test: temp dir setup
fn directory_skips_non_md() {
    let dir = std::env::temp_dir().join("prosesmasher-test-collect-skip");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("mkdir: {e}"));
    std::fs::write(dir.join("notes.txt"), "text").unwrap_or_else(|e| panic!("write txt: {e}"));
    std::fs::write(dir.join("data.json"), "{}").unwrap_or_else(|e| panic!("write json: {e}"));
    std::fs::write(dir.join("doc.md"), "# Doc").unwrap_or_else(|e| panic!("write md: {e}"));

    let files = collect_files(&dir);
    assert_eq!(files.len(), 1, "only .md files");
    assert_eq!(
        files
            .first()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str()),
        Some("doc.md"),
        "should be the .md file"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
