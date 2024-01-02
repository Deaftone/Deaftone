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
