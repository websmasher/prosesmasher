#![allow(clippy::disallowed_methods, clippy::panic)]

use std::path::{Path, PathBuf};

#[must_use]
pub fn cargo_bin() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

#[must_use]
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap_or_else(|e| panic!("workspace root: {e}"))
}

#[must_use]
pub fn fixture_path() -> PathBuf {
    workspace_root().join("crates/adapters/inbound/cli/tests/fixtures/test-essay.md")
}
