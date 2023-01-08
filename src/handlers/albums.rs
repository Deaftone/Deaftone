use std::{fmt, str::FromStr};

use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Path, State},
    http::{Request, Response, StatusCode},
    Json,
};

use sea_orm::EntityTrait;
use serde::{de, Deserialize, Deserializer, Serialize};

use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{services, AppState};

#[allow(non_snake_case)]
#[derive(Serialize)]
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
/* #[axum_macros::debug_handler]
 */
pub async fn get_cover(
    state: State<AppState>,
    Path(album_id): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let album = entity::album::Entity::find_by_id(album_id)
        .one(&state.database)
        .await
        .map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                format!("Failed to get Album from Db: {}", e),
            )
        })?;

    let cover_path = album
        .ok_or((StatusCode::NOT_FOUND, "Album not found".to_string()))?
        .cover
        .ok_or((StatusCode::NOT_FOUND, "Album has no cover".to_string()))?;

    let res = Request::builder()
        .uri("/")
        .body(Body::empty())
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error creating request: {}", e),
            )
        })?;
    let response = ServeFile::new(cover_path).oneshot(res).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error serving file: {}", e),
        )
    })?;
    Ok(response.map(boxed))
}
#[derive(Deserialize, Clone)]
pub struct GetAllAlbums {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    sort: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    size: Option<u64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    page: Option<u64>,
}
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
            StatusCode::ACCEPTED,
            format!("Failed to get albums {}", err),
        )),
    }

    //}
}

/* pub async fn get_albums_paginate(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllAlbums>,
    Path(page): Path<u64>,
) -> Result<Json<Vec<entity::album::Model>>, (StatusCode, String)> {
    let albums =
        services::album::get_albums_paginate(&state.database, page, params.size, params.sort).await;
    match albums {
        Ok(_albums) => Ok(Json(_albums)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get albums {}", err),
        )),
    }
    //}
} */
/* else if params.size.is_some() {
    let size: u64 = params.size.unwrap_or(10);
    albums =
        services::album::get_albums_paginate(&state.database, params.page.unwrap_or(0), size)
            .await;
    match albums {
        Ok(_albums) => Ok(Json(_albums)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get albums {}", err),
        )),
    }
} */
/* pub async fn get_album_page(
    Extension(ref db): Extension<DatabaseConnection>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<Vec<entity::album::Model>>, (StatusCode, String)> {
    println!("{:?}", params);
    let albums = services::album::get_albums_paginate(
        db,
        0,
        params.get("size").unwrap().parse::<usize>().unwrap(),
    )
    .await;
    match albums {
        Ok(_albums) => return Ok(Json(_albums)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get albums {}", err),
        )),
    }
} */
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
