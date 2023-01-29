pub mod database;
pub mod handlers;
pub mod scanner;
pub mod services;
pub mod settings;
pub mod test_util;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use std::sync::atomic::AtomicBool;
use crate::settings::Settings;
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub scanner: Scanner,
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}
static SCAN_STATUS: AtomicBool = AtomicBool::new(false);
