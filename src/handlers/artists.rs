use axum::{
    extract::Query,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QuerySelect};

use crate::{services, AppState};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

pub async fn get_artist(
    Path(artist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<services::artist::DbArtist>, (StatusCode, String)> {
    let artist = services::artist::get_artist_by_id(&state.database, artist_id).await;

    match artist {
        Ok(_artist) => Ok(Json(_artist)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get artist {}", err),
        )),
    }
}
#[derive(Deserialize, Clone)]
pub struct GetArtists {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    sort: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    size: Option<u64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    page: Option<u64>,
}
pub async fn get_artists(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetArtists>,
) -> Result<Json<Vec<entity::artist::Model>>, (StatusCode, String)> {
    let artists = services::artist::get_artists(&state.database, params.size, params.sort).await;
    /* = match params.page.is_some() {
        true => services::artist::get_artists(&state.database, params.size, params.sort).await,
        _ => ,
    }; */
    match artists {
        Ok(artists) => Ok(Json(artists)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get albums {}", err),
        )),
    }
}

pub async fn get_latest_artists(state: &AppState, size: u64) -> Json<Vec<entity::artist::Model>> {
    Json(
        services::artist::get_latest_artist(&state.database, Some(size))
            .await
            .unwrap(),
    )
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
