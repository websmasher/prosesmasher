use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[must_use]
pub fn write_temp(name: &str, content: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("prosesmasher-test-{name}-{id}"));
    #[allow(clippy::disallowed_methods)]
    std::fs::write(&path, content)
        .unwrap_or_else(|e| panic!("failed to write temp {}: {e}", path.display()));
    path
}

pub fn cleanup(path: &Path) {
    #[allow(clippy::disallowed_methods)]
    let _ = std::fs::remove_file(path);
}
