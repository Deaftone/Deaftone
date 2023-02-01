use config::{Config, ConfigError, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log_level: String,
    pub db_path: String,
    pub media_path: String,
}

impl Settings {
    // Returns settings block
    pub fn new() -> Self {
        let s = Config::builder()
            .add_source(config::File::with_name("settings.toml"))
            .build();
        match s {
            Ok(s) => match s.try_deserialize::<Self>() {
                Ok(file) => file,
                Err(err) => {
                    println!("Failed to load config loading default {err}");
                    Self::new_default().unwrap()
                }
            },
            Err(err) => {
                println!("Failed to build config loading default {err}");
                Self::new_default().unwrap()
            }
        }
    }
    // Returns default settings useful for testing
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
impl Default for Settings {
    fn default() -> Self {
        Settings::new()
    }
}
