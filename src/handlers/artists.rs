use axum::{
    extract::Query,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};

use crate::{services, AppState};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};
#[derive(Serialize)]
pub struct ArtistResponse {
    id: String,
    name: String,
    image: String,
    bio: String,
    albums: Vec<entity::album::Model>,
}
pub async fn get_artist(
    Path(artist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ArtistResponse>, (StatusCode, String)> {
    let artist = entity::artist::Entity::find_by_id(artist_id)
        .order_by_desc(entity::album::Column::Year)
        .find_with_related(entity::album::Entity)
        .all(&state.database)
        .await
        .expect("Failed to get album");

    return match artist.first() {
        Some(f) => {
            let artist_model = f.0.to_owned();
            let albums = f.1.to_owned();
            Ok(Json(ArtistResponse {
                id: artist_model.id,
                name: artist_model.name,
                image: artist_model.image.unwrap_or_default(),
                bio: artist_model.bio.unwrap_or_default(),
                albums,
            }))
        }
        None => Err((StatusCode::ACCEPTED, "Failed to find album".to_owned())),
    };
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

    /*     match params.sort.as_ref().unwrap().as_str() {
        "latest" => get_latest_artists(&state, params.limit.unwrap_or(50)).await,
        _ => {

        }
    } */
}

pub async fn get_latest_artists(state: &AppState, size: u64) -> Json<Vec<entity::artist::Model>> {
    Json(
        services::artist::get_latest_albums(&state.database, Some(size))
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
