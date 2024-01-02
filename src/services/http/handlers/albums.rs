use super::{AlbumResponse, GetAllAlbums};
use crate::{
    services::{
        self,
        http::{
            error::{ApiError, Status},
            SuccessResponse,
        },
    },
    AppState, ASSETS,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, Request, Response},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use tower::ServiceExt;
use tower_http::services::ServeFile;

#[utoipa::path(
    get,
    path = "/albums/{album_id}",
    params(
        ("album_id" = String, Path, description = "Album Id")
    ),
    responses(
        (status = 200, description = "Returns a album", body = AlbumResponseOpenApi),
        (status = 500, description = "Error occured", body = ErrorResponse<String>),
        (status = 404, description = "Album not found", body = ErrorResponse<String>)

    )
)]
pub async fn get_album(
    Path(album_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<AlbumResponse>>, ApiError> {
    let (album_model, songs) = services::album::get_album_by_id(&state.database, &album_id).await?;
    Ok(Json(SuccessResponse {
        status: Status::Success,
        message: AlbumResponse {
            id: album_model.id,
            name: album_model.name,
            artist: album_model.artist_name,
            album_description: /* album_model.album_description.unwrap_or_default() */ String::from("TODO"),
            artist_id: album_model.artist_id.unwrap_or_default(),
            year: album_model.year,
            song_count: songs.len() as i32,
            songs,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/albums",
    params(
        GetAllAlbums
    ),
    responses(
        (status = 200, description = "List containing albums", body = AlbumsResponseOpenApi),
        (status = 500, description = "Failed to get albums", body = ErrorResponse<String>)
    )
)]
pub async fn get_albums(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllAlbums>,
) -> Result<Json<SuccessResponse<Vec<entity::album::Model>>>, ApiError> {
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
    Ok(Json(SuccessResponse {
        status: Status::Success,
        message: albums,
    }))
}

#[utoipa::path(
    get,
    path = "/albums/{album_id}/cover",
    params(
        ("album_id" = String, Path, description = "Album Id")
    ),
    responses(
        (status = 200, description = "Returns a album cover"),
        (status = 404, description = "Album not found", body = ErrorResponse<String>)

    )
)]
// Oneday fix the error handling or convert to a ApiError. Putting this logic inside the album service is questionable.
// Since the service is meant for DB logic only. We should have a 3 layer access WEB APP DATABASE. Currenlty we really only have 2
pub async fn get_cover(
    State(state): State<AppState>,
    Path(album_id): Path<String>,
) -> Result<Response<Body>, ApiError> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();
    let album = services::album::get_album_by_id_slim(&state.database, &album_id).await?;

    if album.cover.is_some() {
        // Serve image from FS
        let cover = album.cover.unwrap();
        match ServeFile::new(&cover).oneshot(res).await {
            Ok(res) => {
                if res.status() == StatusCode::NOT_FOUND {
                    Err(ApiError(
                        StatusCode::NOT_FOUND,
                        anyhow!("Unable to find file for album_id: {}", album_id),
                    ))
                } else {
                    Ok(Body::new(res).into_response())
                }
            }

            Err(err) => Err(ApiError(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow!(
                    "Unable to serve file for album_id: {}. Err: {}",
                    album_id,
                    err
                ),
            )),
        }
    } else {
        // Serve unknown album image
        let unknown_album = ASSETS.get_file("unknown_album.jpg").unwrap();
        let body = Body::new(Body::from(unknown_album.contents()));
        Ok(Response::builder()
            .header(header::CONTENT_TYPE, "image/jpg")
            .body(body)
            .unwrap())
    }
}
