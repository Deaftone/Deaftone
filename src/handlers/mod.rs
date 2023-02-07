use std::{fmt, str::FromStr};

use ::serde::{Deserialize, Serialize};
use axum::response::{IntoResponse, Response};
use entity::album::Model as AlbumModel;
use entity::song::Model as SongModel;
use hyper::StatusCode;
use serde::{de, Deserializer};
use utoipa::{IntoParams, ToSchema};

pub mod albums;
pub mod artists;
pub mod playlist;
pub mod songs;
pub mod stream;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AlbumResponse {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub artist_id: String,
    pub album_description: String,
    pub year: i32,
    pub song_count: i32,
    pub songs: Vec<SongModel>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ArtistResponse {
    pub id: String,
    pub name: String,
    pub image: String,
    pub bio: String,
    pub albums: Vec<AlbumModel>,
}

#[derive(Serialize)]
pub struct PlayListResponse {
    id: String,
    name: String,
    songs: Vec<entity::song::Model>,
}

#[derive(Serialize)]
pub struct LikeResponse {
    liked: bool,
}

#[derive(Serialize, ToSchema)]
pub struct SongResponse {
    id: String,
    path: String,
    title: String,
    disk: i32,
    artist: String,
    album_name: String,
    length: u32,
    year: i32,
    album_id: String,
    liked: bool,
}

#[derive(Deserialize, Clone, IntoParams, ToSchema)]
pub struct GetAllArtists {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    #[schema(example = "sort = name | latest")]
    sort: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    size: Option<u64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    page: Option<u64>,
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
#[derive(Debug)]
pub enum ApiError {
    RecordNotFound(String),
    DatabaseError(sea_orm::DbErr),
    CoverNotFound(std::io::Error),
}
// Convert sea_orm::DbErr into our custom ApiError allows ? to be called on sea_orm querys such as find_by_id().await? etc. Pushing up the error to the caller.
// Which most of the time is a web handler. Which with impl IntoResponse for ApiError can convert these errors into errors with response codes and good messages
impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(error)
    }
}

// Converts Service into a response with a HTTP StatusCode and a string to be returned to the user
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An unexpected exception has occured: {err}"),
            )
                .into_response(),
            ApiError::RecordNotFound(err) => {
                (StatusCode::NOT_FOUND, format!("Record not found: {err}")).into_response()
            }
            ApiError::CoverNotFound(err) => {
                (StatusCode::NOT_FOUND, format!("Cover not found: {err}")).into_response()
            }
        }
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
