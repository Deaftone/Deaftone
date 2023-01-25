use super::{ArtistResponse, GetAllArtists};
use crate::{services, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
#[utoipa::path(
    get,
    path = "/artists/{id}",
    params(
        ("id" = String, Path, description = "Artist Id")
    ),
    responses(
        (status = 200, description = "Returns a Artist", body = ArtistResponse),
        (status = 404, description = "Failed to get artist ", body = String)

    )
)]
pub async fn get_artist(
    Path(artist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ArtistResponse>, (StatusCode, String)> {
    let artist = services::artist::get_artist_by_id(&state.database, artist_id).await;
    match artist {
        Ok(_artist) => Ok(Json(_artist)),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Failed to get artist {}", err),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/artists",
    params(
        GetAllArtists
    ),
    responses(
        (status = 200, description = "List containing artists", body = [ArtistModel]),
        (status = 500, description = "Failed to get albums ", body = String)

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
            StatusCode::NOT_FOUND,
            format!("Failed to get albums {}", err),
        )),
    }
}
