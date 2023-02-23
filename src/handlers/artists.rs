use super::{ArtistResponse, GetAllArtists, GetResposne};
use crate::{
    services::{self, DbArtist},
    ApiError, AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
#[utoipa::path(
    get,
    path = "/artists/{id}",
    params(
        ("artist_id" = String, Path, description = "Artist Id")
    ),
    responses(
        (status = 200, description = "Returns a Artist", body = ArtistResponse),
        (status = 500, description = "Database error occured", body = String),
        (status = 404, description = "Failed to get artist ", body = String)

    )
)]
pub async fn get_artist(
    Path(artist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<GetResposne<ArtistResponse>>, ApiError> {
    let (artist_model, albums) = services::artist::get_artist_by_id(&state.database, &artist_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get artist: \"{:?}\" for {:}", e, artist_id);
            e
        })?;
    Ok(Json(GetResposne {
        data: DbArtist {
            id: artist_model.id,
            name: artist_model.name,
            image: artist_model.image.unwrap_or_default(),
            bio: artist_model.bio.unwrap_or_default(),
            albums,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/artists",
    params(
        GetAllArtists
    ),
    responses(
        (status = 200, description = "List containing artists", body = [entity::artist::Model]),
        (status = 500, description = "Database error occured", body = String)
    )
)]
pub async fn get_artists(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllArtists>,
) -> Result<Json<GetResposne<Vec<entity::artist::Model>>>, ApiError> {
    let artists = match params.page.is_some() {
        true => {
            services::artist::get_artists_paginate(
                &state.database,
                params.page,
                params.size,
                params.sort,
            )
            .await?
        }
        _ => services::artist::get_artists(&state.database, params.size, params.sort).await?,
    };
    Ok(Json(GetResposne { data: artists }))
}
