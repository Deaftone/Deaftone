use std::{fmt, str::FromStr};

use ::serde::{Deserialize, Serialize};
use entity::album::Model as AlbumModel;
use entity::song::Model as SongModel;
use serde::{de, Deserializer};
use utoipa::{IntoParams, ToSchema};
pub mod albums;
pub mod artists;
pub mod playlist;
pub mod songs;
pub mod stream;

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
    songs: Vec<SongModel>,
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
    duration: u32,
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
