pub mod database;
pub mod handlers;
pub mod services;
pub mod settings;
pub mod test_util;
use self::services::task::TaskType;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use core::fmt;
use handlers::ErrorResposne;
use hyper::StatusCode;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use serde::{de, Deserialize, Deserializer};
use services::DeaftoneService;
use std::{str::FromStr, sync::atomic::AtomicBool};
use tokio::sync::mpsc::error::SendError;

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

#[derive(Debug)]
pub enum ApiError {
    RecordNotFound,
    ParamError(String),
    TaskError(SendError<TaskType>),
    DatabaseError(sea_orm::DbErr),
    FileNotFound(String),
    IoError(std::io::Error),
    UnknownError(String),
}
// Convert sea_orm::DbErr into our custom ApiError allows ? to be called on sea_orm querys such as find_by_id().await? etc. Pushing up the error to the caller.
// Which most of the time is a web handler. Which with impl IntoResponse for ApiError can convert these errors into errors with response codes and good messages
impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(error)
    }
}
impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        ApiError::IoError(error)
    }
}
// Converts Service into a response with a HTTP StatusCode and a string to be returned to the user
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An unexpected exception has occured: {}", err),
            ),
            ApiError::RecordNotFound => (StatusCode::NOT_FOUND, r#"Record not found"#.to_string()),
            ApiError::FileNotFound(err) => {
                (StatusCode::NOT_FOUND, format!("File not found: {}", err))
            }
            ApiError::IoError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO Error: {}", err),
            ),
            ApiError::UnknownError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown error: {}", err),
            ),
            ApiError::TaskError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Task has failed to be sent: {}", err),
            ),
            ApiError::ParamError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Missing: {}", err),
            ),
        };

        (
            status_code,
            Json(ErrorResposne {
                error: error_message,
            }),
        )
            .into_response()
    }
}

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
