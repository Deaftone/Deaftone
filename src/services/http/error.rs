use axum::response::{IntoResponse, Response};
use axum::Json;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Serialize, Debug, Deserialize, ToSchema)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

pub struct ApiError(pub StatusCode, pub anyhow::Error);
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!(
            "{:?}",
            ErrorResponse {
                status: Status::Error,
                message: self.1.to_string(),
            }
        );
        (
            self.0,
            Json(ErrorResponse {
                status: Status::Error,
                message: self.1.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}
