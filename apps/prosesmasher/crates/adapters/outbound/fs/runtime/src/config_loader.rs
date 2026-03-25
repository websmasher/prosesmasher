//! `ConfigLoader` implementation — reads JSON config, validates, converts to domain.

use std::path::Path;

use garde::Validate;
use prosesmasher_domain_types_runtime::{CheckConfig, ConfigError, ReadError};
use prosesmasher_ports_outbound_traits::ConfigLoader;

use crate::config_dto::ConfigDto;
use crate::file_reader::FsFileReader;
use prosesmasher_ports_outbound_traits::FileReader;

/// Parse canonical JSON config content into the domain config model.
///
/// # Errors
///
/// Returns [`ConfigError`] when the JSON is malformed, fails DTO validation,
/// or cannot be converted into the canonical domain configuration.
pub fn parse_config_json(content: &str) -> Result<CheckConfig, ConfigError> {
    #[allow(clippy::disallowed_methods)] // reason: centralized deserialization point
    let dto: ConfigDto =
        serde_json::from_str(content).map_err(|e| ConfigError::InvalidJson(e.to_string()))?;

    dto.validate()
        .map_err(|e| ConfigError::ValidationFailed(e.to_string()))?;

    dto.into_domain()
}

/// Filesystem-backed config loader.
#[derive(Debug)]
pub struct FsConfigLoader;

impl ConfigLoader for FsConfigLoader {
    fn load_config(&self, path: &Path) -> Result<CheckConfig, ConfigError> {
        // 1. Read file — map ReadError variants to appropriate ConfigError
        let content = FsFileReader.read_to_string(path).map_err(|e| match e {
            ReadError::NotFound(msg) => ConfigError::NotFound(msg),
            ReadError::PermissionDenied(msg) | ReadError::Io(msg) => {
                ConfigError::NotFound(format!("cannot read config: {msg}"))
            }
        })?;
        parse_config_json(&content)
    }
}

#[cfg(test)]
#[path = "config_loader_tests/mod.rs"]
mod tests;
