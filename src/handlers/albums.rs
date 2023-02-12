use super::{AlbumResponse, ApiError, GetAllAlbums};
use crate::{
    services::{self},
    AppState,
};
use axum::{
    body::{boxed, Body, BoxBody, Full},
    extract::{Path, State},
    http::{header, Request, Response},
    Json,
};
use include_dir::{include_dir, Dir};
use tower::ServiceExt;
use tower_http::services::ServeFile;
static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");

#[utoipa::path(
    get,
    path = "/albums/{id}",
    params(
        ("id" = String, Path, description = "Album Id")
    ),
    responses(
        (status = 200, description = "Returns a album", body = AlbumResponse),
        (status = 500, description = "Database error occured", body = String),
        (status = 404, description = "Album not found", body = String)

    )
)]
pub async fn get_album(
    Path(album_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AlbumResponse>, ApiError> {
    let (album_model, songs) = services::album::get_album_by_id(&state.database, album_id).await?;
    Ok(Json(AlbumResponse {
        id: album_model.id,
        name: album_model.name,
        artist: album_model.artist_name,
        album_description: /* album_model.album_description.unwrap_or_default() */ String::from("TODO"),
        artist_id: album_model.artist_id.unwrap_or_default(),
        year: album_model.year,
        song_count: songs.len() as i32,
        songs,
    }))
}

#[utoipa::path(
    get,
    path = "/albums/{id}/cover",
    params(
        ("id" = String, Path, description = "Album Id")
    ),
    responses(
        (status = 200, description = "Returns a album cover"),
        (status = 404, description = "Album not found", body = String)

    )
)]
// Oneday fix the error handling or convert to a ApiError. Putting this logic inside the album service is questionable.
// Since the service is meant for DB logic only. We should have a 3 layer access WEB APP DATABASE. Currenlty we really only have 2
pub async fn get_cover(
    State(state): State<AppState>,
    Path(album_id): Path<String>,
) -> Result<Response<BoxBody>, ApiError> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();
    let album = services::album::get_album_by_id_single(&state.database, album_id).await?;

    if album.cover.is_some() {
        // Serve image from FS
        match ServeFile::new(album.cover.unwrap()).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err(ApiError::FileNotFound(err)),
        }
    } else {
        // Serve unknown album image
        let unknown_album = ASSETS.get_file("unknown_album.jpg").unwrap();
        let body = boxed(Full::from(unknown_album.contents()));
        Ok(Response::builder()
            .header(header::CONTENT_TYPE, "image/jpg")
            .body(body)
            .unwrap())
    }
}

#[utoipa::path(
    get,
    path = "/albums",
    params(
        GetAllAlbums
    ),
    responses(
        (status = 200, description = "List containing albums", body = [entity::album::Model]),
        (status = 500, description = "Failed to get albums", body = String)
    )
)]
pub async fn get_albums(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllAlbums>,
) -> Result<Json<Vec<entity::album::Model>>, ApiError> {
    let albums = match params.page.is_some() {
        true => {
            services::album::get_albums_paginate(
                &state.database,
                params.page,
                params.size,
                params.sort,
            )
            .await?
        }
        _ => services::album::get_albums(&state.database, params.size, params.sort).await?,
    };
    Ok(Json(albums))
}
