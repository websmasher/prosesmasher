//! Filesystem adapter — `FileReader` and `ConfigLoader` implementations.

mod config_dto;
pub mod config_loader;
pub mod file_reader;

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
pub fn preset_contents(name: &str) -> Option<&'static str> {
    match name {
        "general-en" => Some(include_str!("../presets/general-en.json")),
        "article-en" => Some(include_str!("../presets/article-en.json")),
        "substack-en" => Some(include_str!("../presets/substack-en.json")),
        "email-en" => Some(include_str!("../presets/email-en.json")),
        "tweet-en" => Some(include_str!("../presets/tweet-en.json")),
        _ => None,
    }
}

#[must_use]
pub const fn full_config_contents() -> &'static str {
    include_str!("../presets/full-config-en.json")
}
