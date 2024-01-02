use crate::{
    services::{http::error::ApiError, scanner::ScanType, task::TaskType},
    AppState,
};
use anyhow::anyhow;
use axum::{extract::State, Json};
use hyper::StatusCode;
use tokio::sync::mpsc::Sender;

use super::{TaskQuery, TaskResponse};
pub async fn handle_task(
    axum::extract::Query(params): axum::extract::Query<TaskQuery>,
    State(state): State<AppState>,
) -> Result<Json<TaskResponse>, ApiError> {
    match params.task {
        Some(task) => match task.as_str() {
            "scan_library_full" => {
                send_task(
                    TaskType::ScanLibrary(ScanType::FullScan),
                    state.services.task,
                )
                .await
            }
            "scan_library_partial" => {
                send_task(
                    TaskType::ScanLibrary(ScanType::PartialScan),
                    state.services.task,
                )
                .await
            }
            "scan_metadata" => send_task(TaskType::PopulateMetadata, state.services.task).await,

            _ => Err(ApiError(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow!("Invalid task type: {}", task),
            )),
        },
        None => Err(ApiError(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow!("Missing param type"),
        )),
    }
}

async fn send_task(
    task: TaskType,
    task_service: Sender<TaskType>,
) -> Result<Json<TaskResponse>, ApiError> {
    match task_service.send(task).await {
        Ok(_) => {
            tracing::info!("Command sent to TaskService");
            Ok(Json(TaskResponse {
                status: r#"sent"#.to_owned(),
            }))
        }
        Err(err) => {
            tracing::error!("Failed to send command to TaskService {:}", err);
            Err(ApiError(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow!("Task error: {}", err),
            ))
        }
    }
}
