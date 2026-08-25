use config::AppConfig;

const CONFIG_FILE: &str = "config.toml";

pub fn load() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(CONFIG_FILE);

    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        tracing::info!(path = CONFIG_FILE, "loaded config from file");
        Ok(config)
    } else {
        tracing::info!("no config file found, using defaults");
        Ok(AppConfig::default())
    }
}
