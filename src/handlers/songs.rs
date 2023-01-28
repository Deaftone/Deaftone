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

#[utoipa::path(
    get,
    path = "/song/{id}",
    params(
        ("id" = String, Path, description = "Song Id")
    ),
    responses(
        (status = 200, description = "Returns a song", body = SongResponse),
        (status = 404, description = "Song not found", body = String)

    )
)]
pub async fn get_song(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SongResponse>, (StatusCode, String)> {
    match services::song::get_song(&state.database, song_id).await {
        Ok(song) => match song {
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
            None => Err((StatusCode::NOT_FOUND, "Failed to find song".to_owned())),
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read from DB {:}", err),
        )),
    }
}

pub async fn like_song(
    State(state): State<AppState>,
    Path(song_id): Path<String>,
) -> Result<Json<LikeResponse>, (StatusCode, String)> {
    match services::song::like_song(&state.database, song_id).await {
        Ok(i) => Ok(Json(LikeResponse { liked: i })),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to like song: {err}"),
        )),
    }
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
                    format!("Failed to serve cover: {err}"),
                )),
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Unable to find album".to_string())),
    }
}
