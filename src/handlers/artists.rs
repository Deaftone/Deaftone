use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::{IntoParams, ToSchema};

use crate::{services, AppState};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

#[utoipa::path(
    get,
    path = "/artists/{id}",
    params(
        ("id" = String, Path, description = "Artist Id")
    ),
    responses(
        (status = 200, description = "List containing albums", body = [entity::album::Model])
    )
)]
pub async fn get_artist(
    Path(artist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<services::DbArtist>, (StatusCode, String)> {
    let artist = services::artist::get_artist_by_id(&state.database, artist_id).await;

    match artist {
        Ok(_artist) => Ok(Json(_artist)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get artist {}", err),
        )),
    }
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

#[utoipa::path(
    get,
    path = "/artists",
    params(
        GetAllArtists
    ),
    responses(
        (status = 200, description = "List containing artists", body = [entity::artist::Model])
    )
)]
pub async fn get_artists(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllArtists>,
) -> Result<Json<Vec<entity::artist::Model>>, (StatusCode, String)> {
    let artists = match params.page.is_some() {
        true => {
            services::artist::get_artists_paginate(
                &state.database,
                params.page,
                params.size,
                params.sort,
            )
            .await
        }
        _ => services::artist::get_artists(&state.database, params.size, params.sort).await,
    };
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
