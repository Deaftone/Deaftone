use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Path, State},
    http::{Request, Response, StatusCode},
    Json,
};

use crate::{services, AppState};
use sea_orm::EntityTrait;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use super::{LikeResponse, SongResponse};

pub async fn get_song(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SongResponse>, (StatusCode, String)> {
    let song = services::song::get_song(&state.database, song_id)
        .await
        .unwrap();
    match song {
        Some(f) => Ok(Json(SongResponse {
            id: f.id,
            path: f.path,
            title: f.title,
            disk: f.disk.unwrap_or_default(),
            artist: f.artist,
            album_name: f.album_name,
            duration: f.duration,
            year: f.year.unwrap_or_default(),
            album_id: f.album_id.unwrap_or_default(),
            liked: f.liked,
        })),
        None => Err((StatusCode::ACCEPTED, "Failed to find song".to_owned())),
    }
}

pub async fn like_song(
    State(state): State<AppState>,
    Path(song_id): Path<String>,
) -> Result<Json<LikeResponse>, (StatusCode, String)> {
    let status = services::song::like_song(&state.database, song_id)
        .await
        .unwrap();
    Ok(Json(LikeResponse { liked: status }))
}
pub async fn get_cover(
    State(state): State<AppState>,
    Path(song_id): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();

    let album = entity::song::Entity::find_by_id(song_id)
        .find_also_related(entity::album::Entity)
        .one(&state.database)
        .await
        .unwrap();

    match album {
        Some(f) => {
            // Serve image from FS
            match ServeFile::new(f.1.unwrap().cover.unwrap())
                .oneshot(res)
                .await
            {
                Ok(res) => Ok(res.map(boxed)),
                Err(err) => Err((
                    StatusCode::NOT_FOUND,
                    format!("Something went wrong: {}", err),
                )),
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Unable to find album".to_string())),
    }
}
