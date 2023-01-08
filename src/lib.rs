pub mod database;
pub mod handlers;
pub mod scanner;
pub mod services;
pub mod settings;
pub mod test_util;
use scanner::Scanner;
use sea_orm::DatabaseConnection;
use std::sync::atomic::AtomicBool;
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub scanner: Scanner,
}

static SCAN_STATUS: AtomicBool = AtomicBool::new(false);
