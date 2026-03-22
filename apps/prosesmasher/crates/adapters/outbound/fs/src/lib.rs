//! Filesystem adapter — `FileReader` and `ConfigLoader` implementations.

mod config_dto;
pub mod config_loader;
pub mod file_reader;

use std::path::{Path, PathBuf};

pub use config_loader::FsConfigLoader;
pub use file_reader::FsFileReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PresetInfo {
    pub name: &'static str,
    pub description: &'static str,
}

const PRESETS: &[PresetInfo] = &[
    PresetInfo {
        name: "general-en",
        description: "Baseline quality defaults with no document-shape policy.",
    },
    PresetInfo {
        name: "article-en",
        description: "Standard article structure with heading policy.",
    },
    PresetInfo {
        name: "substack-en",
        description: "Longer, looser newsletter/article structure.",
    },
    PresetInfo {
        name: "email-en",
        description: "Short prose body with no heading policy.",
    },
    PresetInfo {
        name: "tweet-en",
        description: "Very short prose body with no heading policy.",
    },
];

#[must_use]
pub const fn shipped_presets() -> &'static [PresetInfo] {
    PRESETS
}

#[must_use]
pub fn preset_path(name: &str) -> Option<PathBuf> {
    let preset = PRESETS.iter().find(|preset| preset.name == name)?;
    Some(presets_dir().join(format!("{}.json", preset.name)))
}

#[must_use]
pub fn full_config_path() -> PathBuf {
    presets_dir().join("full-config-en.json")
}

fn presets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../presets")
}
