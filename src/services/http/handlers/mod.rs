use crate::empty_string_as_none;
use ::serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub mod albums;
pub mod artists;
pub mod playlist;
pub mod songs;
pub mod streams;
pub mod tasks;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TestResponse {
    state: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AlbumResponse {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub artist_id: String,
    pub album_description: String,
    pub year: i32,
    pub song_count: i32,
    pub songs: Vec<entity::song::Model>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ArtistResponse {
    pub id: String,
    pub name: String,
    pub image: String,
    pub biography: String,
    pub links: ArtistLinks,
    pub albums: Vec<entity::album::Model>,
}
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ArtistLinks {
    all_music: Option<String>,
    amazon_music: Option<String>,
    apple_music: Option<String>,
    deezer: Option<String>,
    discogs: Option<String>,
    facebook: Option<String>,
    itunes: Option<String>,
    spotify: Option<String>,
    tidal: Option<String>,
    twitter: Option<String>,
    wiki: Option<String>,
    youtube: Option<String>,
}
// Now you can access the link data using the struct fields, like link_data.link_all_music

#[derive(Serialize, ToSchema)]
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
pub struct TaskQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    // #[schema(example = "sort = name | latest")]
    task: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct TaskResponse {
    status: String,
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
