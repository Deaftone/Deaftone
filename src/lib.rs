pub mod database;
pub mod handlers;
pub mod scanner;
pub mod services;
pub mod settings;
pub mod test_util;
use self::services::task::TaskType;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc::Sender;

use crate::settings::Settings;
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub task_service: Sender<TaskType>,
}
// Inject global settings static ref
lazy_static! {
    pub static ref SETTINGS: Settings = Settings::default();
}
static SCAN_STATUS: AtomicBool = AtomicBool::new(false);
