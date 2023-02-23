use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    services::{self},
    ApiError, AppState,
};

use super::{GetResposne, LikeResponse, SongResponse};

#[utoipa::path(
    get,
    path = "/song/{id}",
    params(
        ("song_id" = String, Path, description = "Song Id")
    ),
    responses(
        (status = 200, description = "Returns a song", body = SongResponse),
        (status = 404, description = "Song not found", body = String)

    )
)]
pub async fn get_song(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<GetResposne<SongResponse>>, ApiError> {
    let song = services::song::get_song_by_id(&state.database, &song_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get song: \"{:?}\" for {:}", e, song_id);
            e
        })?;
    Ok(Json(GetResposne {
        data: SongResponse {
            id: song.id,
            path: song.path,
            title: song.title,
            disk: song.disk.unwrap_or_default(),
            artist: song.artist,
            album_name: song.album_name,
            length: song.length,
            year: song.year.unwrap_or_default(),
            album_id: song.album_id.unwrap_or_default(),
            liked: song.liked,
        },
    }))
}

pub async fn like_song(
    State(state): State<AppState>,
    Path(song_id): Path<String>,
) -> Result<Json<GetResposne<LikeResponse>>, ApiError> {
    let liked = services::song::like_song(&state.database, song_id).await?;
    Ok(Json(GetResposne {
        data: LikeResponse { liked },
    }))
}
