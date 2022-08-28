use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log_level: String,
    pub db_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::from_str(
                include_str!("settings.default.toml"),
                FileFormat::Toml,
            ))
            .build()?;

        s.try_deserialize()
    }
}
