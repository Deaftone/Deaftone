use config::{Config, ConfigError};
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
        /*         println!(
            "{:?}",
            s.to_owned()
                .try_deserialize::<HashMap<String, String>>()
                .unwrap()
        ); */
        s.try_deserialize()
    }
}
