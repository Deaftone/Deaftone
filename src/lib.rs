pub mod database;
pub mod handlers;
pub mod scanner;
pub mod services;
pub mod settings;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use scanner::Scanner;
use sea_orm::DatabaseConnection;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub scanner: Scanner,
}

/* static SCAN_STATUS: Lazy<Mutex<AtomicBool>> = Lazy::new(|| Mutex::new(AtomicBool::new(false)));
static SETTINGS: Lazy<settings::Settings> =
    Lazy::new(|| settings::Settings::new().expect("Failed to load config: ")); */
lazy_static! {
    static ref SCAN_STATUS: Mutex<AtomicBool> = Mutex::new(AtomicBool::new(false));
    static ref SETTINGS: settings::Settings =
        settings::Settings::new().expect("Failed to load config: ");
}
