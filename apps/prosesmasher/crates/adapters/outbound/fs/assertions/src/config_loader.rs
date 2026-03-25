use prosesmasher_adapters_outbound_fs_runtime::{
    config_loader::parse_config_json, preset_contents, FsConfigLoader,
};
use prosesmasher_domain_types_runtime::{CheckConfig, ConfigError};
use prosesmasher_ports_outbound_traits::ConfigLoader;

use crate::support::{cleanup, write_temp};

#[allow(clippy::panic)]
#[must_use]
pub fn load_json_ok(json: &str) -> CheckConfig {
    let path = write_temp("json", json);
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    match result {
        Ok(config) => config,
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

#[allow(clippy::panic)]
#[must_use]
pub fn load_json_err(json: &str) -> ConfigError {
    let path = write_temp("json", json);
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    match result {
        Ok(_) => panic!("expected Err, got Ok"),
        Err(err) => err,
    }
}

#[allow(clippy::panic)]
#[must_use]
pub fn load_preset_ok(name: &str) -> CheckConfig {
    let json = preset_contents(name).unwrap_or_else(|| panic!("missing preset {name}"));
    parse_config_json(json).unwrap_or_else(|err| panic!("failed to load preset {name}: {err}"))
}
