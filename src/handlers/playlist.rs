use super::PlayListResponse;
use crate::{ApiError, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::EntityTrait;

pub async fn get_playlist(
    Path(playlist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<PlayListResponse>, ApiError> {
    let playlist = entity::playlist::Entity::find_by_id(&playlist_id)
        .find_with_related(entity::song::Entity)
        .all(&state.database)
        .await?;
    match playlist.first() {
        Some(f) => {
            let _playlist_model: entity::playlist::Model = f.0.to_owned();
            let songs = f.1.to_owned();
            Ok(Json(PlayListResponse {
                id: "test".to_string(),
                name: "test".to_string(),
                songs,
            }))
        }
        None => Err(ApiError::RecordNotFound(format!(
            "Playlist \"{playlist_id}\" not found"
        ))),
    }
}
