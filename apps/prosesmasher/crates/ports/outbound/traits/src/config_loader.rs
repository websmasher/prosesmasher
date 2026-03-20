use std::path::Path;

use prosesmasher_domain_types::{CheckConfig, ConfigError};

/// Port for loading check configuration.
pub trait ConfigLoader {
    /// Load configuration from the given path.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::NotFound` if the config file is missing,
    /// `ConfigError::InvalidJson` if the file is not valid JSON,
    /// or `ConfigError::ValidationFailed` if the config fails validation.
    fn load_config(&self, path: &Path) -> Result<CheckConfig, ConfigError>;
}
