use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde::Serialize;

#[derive(Serialize)]
pub struct PlayListResponse {
    id: String,
    name: String,
    songs: Vec<entity::playlists_song::Model>,
}
pub async fn get_playlist(
    Path(playlist_id): Path<String>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<Json<PlayListResponse>, (StatusCode, String)> {
    let playlist = entity::playlists::Entity::find_by_id(playlist_id)
        .find_with_related(entity::playlists_song::Entity)
        .all(db)
        .await
        .expect("Failed to get album");

    match playlist.first() {
        Some(f) => {
            let playlist_model: entity::playlists::Model = f.0.to_owned();
            let songs = f.1.to_owned();
            return Ok(Json(PlayListResponse {
                id: "test".to_string(),
                name: "test".to_string(),
                songs,
            }));
        }
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find album".to_owned(),
            ))
        }
    }
}
