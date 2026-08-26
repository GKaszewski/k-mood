use std::path::Path;

use figment::Figment;
use figment::providers::{Env, Format, Toml};

use crate::AppConfig;
use crate::errors::ConfigError;

const DEFAULT_CONFIG_FILE: &str = "config.toml";
const CONFIG_FILE_VARIABLE: &str = "KMOOD_CONFIG_FILE";
const VARIABLE_PREFIX: &str = "KMOOD_";
const SECTION_SEPARATOR: &str = "__";

pub fn load() -> Result<AppConfig, ConfigError> {
    let named_file = std::env::var(CONFIG_FILE_VARIABLE).ok();
    let path = named_file.as_deref().unwrap_or(DEFAULT_CONFIG_FILE);

    if named_file.is_some() && !Path::new(path).exists() {
        return Err(ConfigError::NamedFileMissing(path.to_owned()));
    }

    let config = Figment::new()
        .merge(Toml::file(path))
        .merge(Env::prefixed(VARIABLE_PREFIX).split(SECTION_SEPARATOR))
        .extract()
        .map_err(Box::new)?;

    announce(path);

    Ok(config)
}

fn announce(path: &str) {
    if Path::new(path).exists() {
        tracing::info!(path, "loaded configuration from file");
    } else {
        tracing::info!("no configuration file found, using defaults");
    }

    let overridden = overriding_variables();
    if !overridden.is_empty() {
        tracing::info!(
            variables = ?overridden,
            "configuration overridden from the environment"
        );
    }
}

fn overriding_variables() -> Vec<String> {
    let mut names: Vec<String> = std::env::vars()
        .map(|(name, _)| name)
        .filter(|name| name.starts_with(VARIABLE_PREFIX) && name != CONFIG_FILE_VARIABLE)
        .collect();
    names.sort();
    names
}
