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
    let artist = services::artist::get_artist(&state.database, artist_id).await;

    match artist {
        Ok(_artist) => Ok(Json(_artist)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get artist {}", err),
        )),
    }
}
#[derive(Deserialize, Clone)]
pub struct GetALlArtists {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    limit: Option<u64>,
    sort: Option<String>,
}
pub async fn get_artists(
    State(state): State<AppState>,
    params: Query<GetALlArtists>,
) -> Json<Vec<entity::artist::Model>> {
    // Check for latest param
    match params.sort.as_deref() {
        Some("latest") => return get_latest_artists(&state, params.limit.unwrap_or(50)).await,
        // Match for limit parma
        _ => match params.limit {
            Some(i) => Json(
                entity::artist::Entity::find()
                    .limit(params.limit.unwrap_or(i))
                    .all(&state.database)
                    .await
                    .expect("Failed to get artists"),
            ),
            _ => Json(
                entity::artist::Entity::find()
                    .all(&state.database)
                    .await
                    .expect("Failed to get artists"),
            ),
        },
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
