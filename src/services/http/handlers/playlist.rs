use crate::{
    services::http::{
        error::{ApiError, Status},
        SuccessResponse,
    },
    AppState,
};

use super::PlayListResponse;
use anyhow::anyhow;

use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;
use sea_orm::EntityTrait;
#[utoipa::path(
    get,
    path = "/playlists/{playlist_id}",
    params(
        ("playlist_id" = String, Path, description = "Playlist Id")
    ),
    responses(
        (status = 200, description = "Returns a playlist", body = PlayListResponseOpenApi),
        (status = 500, description = "Error occured", body = ErrorResponse<String>),
        (status = 404, description = "Playlist not found", body = ErrorResponse<String>)

    )
)]
pub async fn get_playlist(
    Path(playlist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<PlayListResponse>>, ApiError> {
    let playlist = entity::playlist::Entity::find_by_id(&playlist_id)
        .find_with_related(entity::song::Entity)
        .all(&state.database)
        .await?;
    match playlist.first() {
        Some(f) => {
            let _playlist_model: entity::playlist::Model = f.0.to_owned();
            let songs = f.1.to_owned();
            Ok(Json(SuccessResponse {
                status: Status::Success,
                message: PlayListResponse {
                    id: "test".to_string(),
                    name: "test".to_string(),
                    songs,
                },
            }))
        }
        None => Err(ApiError(
            StatusCode::NOT_FOUND,
            anyhow!("Unable to find Playlist for playlist_id: {}", playlist_id),
        )),
    }
}
