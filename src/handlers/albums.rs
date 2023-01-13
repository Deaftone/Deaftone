use std::{fmt, str::FromStr};

use axum::{
    body::{boxed, Body, BoxBody, Full},
    extract::{Path, State},
    http::{header, Request, Response, StatusCode},
    Json,
};

use include_dir::{include_dir, Dir};
use sea_orm::EntityTrait;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{services, AppState};
use tower::ServiceExt;
use tower_http::services::ServeFile;
use utoipa::{IntoParams, ToSchema};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");
#[allow(non_snake_case)]
#[derive(Serialize, ToSchema)]
pub struct AlbumResponse {
    id: String,
    name: String,
    artist: String,
    artistId: String,
    albumDescription: String,
    year: i32,
    songCount: i32,
    songs: Vec<entity::song::Model>,
}

#[utoipa::path(
    get,
    path = "/albums/{id}",
    params(
        ("id" = String, Path, description = "Album Id")
    ),
    responses(
        (status = 200, description = "Returns a album", body = AlbumResponse)
    )
)]
pub async fn get_album(
    Path(album_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AlbumResponse>, (StatusCode, String)> {
    let album = services::album::get_album_by_id(&state.database, album_id).await;
    match album.ok() {
        Some(_album) => match _album.first() {
            Some(f) => {
                let album_model = f.0.to_owned();
                let songs = f.1.to_owned();
                Ok(Json(AlbumResponse {
                    id: album_model.id,
                    name: album_model.name,
                    artist: album_model.artist_name,
                    albumDescription: album_model.album_description.unwrap_or_default(),
                    artistId: album_model.artist_id.unwrap_or_default(),
                    year: album_model.year,
                    songCount: songs.len() as i32,
                    songs,
                }))
            }
            None => Err((StatusCode::ACCEPTED, "Failed to find album".to_owned())),
        },
        None => Err((StatusCode::ACCEPTED, "Failed to find album".to_owned())),
    }
}
pub async fn get_cover(
    State(state): State<AppState>,
    Path(album_id): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();

    let album: Option<entity::album::Model> = entity::album::Entity::find_by_id(album_id)
        .one(&state.database)
        .await
        .unwrap();

    match album {
        Some(f) => {
            if f.cover.is_some() {
                // Serve image from FS
                match ServeFile::new(f.cover.unwrap()).oneshot(res).await {
                    Ok(res) => Ok(res.map(boxed)),
                    Err(err) => Err((
                        StatusCode::NOT_FOUND,
                        format!("Something went wrong: {}", err),
                    )),
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
        None => Err((StatusCode::NOT_FOUND, "Unable to find album".to_string())),
    }
}
#[derive(Deserialize, Clone, IntoParams, ToSchema)]
pub struct GetAllAlbums {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    #[schema(example = "sort = name | artist_name | year | latest")]
    sort: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    size: Option<u64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    page: Option<u64>,
}

#[utoipa::path(
    get,
    path = "/albums",
    params(
        GetAllAlbums
    ),
    responses(
        (status = 200, description = "List containing albums", body = [Vec<entity::album::Model>]),
        (status = 404, description = "Album not found")
    )
)]
pub async fn get_albums(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllAlbums>,
) -> Result<Json<Vec<entity::album::Model>>, (StatusCode, String)> {
    let albums = match params.page.is_some() {
        true => {
            services::album::get_albums_paginate(
                &state.database,
                params.page,
                params.size,
                params.sort,
            )
            .await
        }
        _ => services::album::get_albums(&state.database, params.size, params.sort).await,
    };
    match albums {
        Ok(_albums) => Ok(Json(_albums)),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Failed to get albums {}", err),
        )),
    }
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
