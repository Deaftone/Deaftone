use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use sea_orm::EntityTrait;
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct PlayListResponse {
    id: String,
    name: String,
    songs: Vec<entity::song::Model>,
}
pub async fn get_playlist(
    Path(playlist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<PlayListResponse>, (StatusCode, String)> {
    let playlist = entity::playlist::Entity::find_by_id(playlist_id)
        .find_with_related(entity::song::Entity)
        .all(&state.database)
        .await
        .expect("Failed to get album");

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
        None => Err((StatusCode::ACCEPTED, "Failed to find album".to_owned())),
    }
}
