use config::{Config, ConfigError, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log_level: String,
    pub db_path: String,
    pub media_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("settings.toml"))
            .build()?;

        s.try_deserialize()
    }

    pub fn new_default() -> Result<Self, ConfigError> {
        tracing::info!("Failed to load settings.toml. Loaded default config");
        let s = Config::builder()
            .add_source(config::File::from_str(
                "log_level= \"info\"
            db_path=\"./deaftone.sqlite\"
            media_path=\"./music\"",
                FileFormat::Toml,
            ))
            .build()?;
        s.try_deserialize()
    }
}
