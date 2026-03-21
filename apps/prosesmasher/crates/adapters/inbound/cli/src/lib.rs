//! CLI adapter — composition root and argument handling.

use std::path::PathBuf;

pub mod args;
pub mod checks;
pub mod output;

// Used by the binary target (main.rs), not the library.
use low_expectations as _;
use prosesmasher_adapters_outbound_fs as _;
use prosesmasher_adapters_outbound_parser as _;
use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;

/// Collect markdown files from a path.
///
/// If the path is a directory, recursively finds all `.md` files and returns
/// them sorted. If the path is a single file, returns it as-is (regardless
/// of extension — the user explicitly chose it).
pub fn collect_files(path: &std::path::Path) -> Vec<PathBuf> {
    if path.is_dir() {
        let mut files: Vec<PathBuf> = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().is_some_and(|ext| ext == "md")
            })
            .map(walkdir::DirEntry::into_path)
            .collect();
        files.sort();
        files
    } else {
        vec![path.to_path_buf()]
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
