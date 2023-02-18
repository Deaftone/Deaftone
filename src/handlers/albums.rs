use super::{AlbumResponse, GetAllAlbums};
use crate::{
    services::{self},
    ApiError, AppState, ASSETS,
};
use axum::{
    body::{boxed, Body, BoxBody, Full},
    extract::{Path, State},
    http::{header, Request, Response},
    Json,
};
use hyper::StatusCode;
use tower::ServiceExt;
use tower_http::services::ServeFile;

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
    let (album_model, songs) = services::album::get_album_by_id(&state.database, &album_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get album \"{:?}\" for {album_id}", e);
            e
        })?;
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
        true => services::album::get_albums_paginate(
            &state.database,
            params.page,
            params.size,
            params.sort,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to get albums: {:?}", e);
            e
        })?,
        _ => services::album::get_albums(&state.database, params.size, params.sort)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get albums: {:?}", e);
                e
            })?,
    };
    Ok(Json(albums))
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
    let album = services::album::get_album_by_id_single(&state.database, &album_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get cover: \"{:?}\" for {:}", e, album_id);
            e
        })?;

    if album.cover.is_some() {
        // Serve image from FS
        let cover = album.cover.unwrap();
        match ServeFile::new(&cover).oneshot(res).await {
            Ok(res) => {
                if res.status() == StatusCode::NOT_FOUND {
                    Err(ApiError::FileNotFound(cover))
                } else {
                    Ok(res.map(boxed))
                }
            }
            Err(err) => Err(ApiError::UnknownError(err.to_string())),
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
