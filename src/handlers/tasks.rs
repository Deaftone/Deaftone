use crate::{
    services::{scanner::ScanType, task::TaskType},
    ApiError, AppState,
};
use axum::{extract::State, Json};
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
                    state.task_service,
                )
                .await
            }
            "scan_library_partial" => {
                send_task(
                    TaskType::ScanLibrary(ScanType::PartialScan),
                    state.task_service,
                )
                .await
            }
            "scan_metadata" => send_task(TaskType::PopulateMetadata, state.task_service).await,

            _ => Err(ApiError::ParamError(r#"Invalid task type"#.to_owned())),
        },
        None => Err(ApiError::ParamError(r#"Missing param task"#.to_owned())),
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
            Err(ApiError::TaskError(err))
        }
    }
}
