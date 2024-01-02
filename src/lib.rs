pub mod database;
pub mod services;
pub mod settings;
pub mod test_util;

use core::fmt;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use serde::{de, Deserialize, Deserializer};
use services::DeaftoneService;
use std::{str::FromStr, sync::atomic::AtomicBool};

use crate::settings::Settings;
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub services: DeaftoneService,
}
// Inject global settings static ref
lazy_static! {
    pub static ref SETTINGS: Settings = Settings::default();
}
static SCAN_STATUS: AtomicBool = AtomicBool::new(false);
static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Error;

    #[derive(Debug, PartialEq, Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "empty_string_as_none")]
        value: Option<i32>,
    }

    #[test]
    fn test_empty_string_as_none() {
        // Test case 1: Non-empty string
        let json = r#"{"value": "42"}"#;
        let result: Result<TestStruct, Error> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), TestStruct { value: Some(42) });

        // Test case 2: Empty string should be converted to None
        let json = r#"{"value": ""}"#;
        let result: Result<TestStruct, Error> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), TestStruct { value: None });

        // Test case 3: Null should be converted to None
        let json = r#"{"value": null}"#;
        let result: Result<TestStruct, Error> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), TestStruct { value: None });

        // Test case 4: Non-empty string that cannot be parsed should result in an error
        let json = r#"{"value": "abc"}"#;
        let result: Result<TestStruct, Error> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
