use super::{ArtistLinks, ArtistResponse, GetAllArtists};
use crate::{
    services::{
        http::{
            error::{ApiError, Status},
            SuccessResponse,
        },
        DbArtist,
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use tracing::instrument;
#[utoipa::path(
    get,
    path = "/artists/{artist_id}",
    params(
        ("artist_id" = String, Path, description = "Artist Id")
    ),
    responses(
        (status = 200, description = "Returns a Artist", body = ArtistResponseOpenApi),
        (status = 500, description = "Database error occured", body = ErrorResponse<String>),
        (status = 404, description = "Failed to get artist ", body = ErrorResponse<String>)

    )
)]
#[instrument(level = "info", name = "get_artist", skip_all)]
pub async fn get_artist(
    Path(artist_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<ArtistResponse>>, ApiError> {
    let (artist_model, albums) = state.services.artist.get_artist_by_id(&artist_id).await?;
    Ok(Json(SuccessResponse {
        status: Status::Success,
        message: DbArtist {
            id: artist_model.id,
            name: artist_model.name,
            image: artist_model.image.unwrap_or_default(),
            biography: artist_model.biography.unwrap_or_default(),
            links: ArtistLinks {
                all_music: artist_model.link_all_music,
                amazon_music: artist_model.link_amazon_music,
                apple_music: artist_model.link_apple_music,
                deezer: artist_model.link_deezer,
                discogs: artist_model.link_discogs,
                facebook: artist_model.link_facebook,
                itunes: artist_model.link_itunes,
                spotify: artist_model.link_spotify,
                tidal: artist_model.link_tidal,
                twitter: artist_model.link_twitter,
                wiki: artist_model.link_wiki,
                youtube: artist_model.link_youtube,
            },
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
        (status = 200, description = "List containing artists", body = ArtistsResponseOpenApi),
        (status = 500, description = "Error occured", body = ErrorResponse<String>)
    )
)]
pub async fn get_artists(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetAllArtists>,
) -> Result<Json<SuccessResponse<Vec<entity::artist::Model>>>, ApiError> {
    let artists = match params.page.is_some() {
        true => {
            state
                .services
                .artist
                .get_artists_paginate(params.page, params.size, params.sort)
                .await?
        }
        _ => {
            state
                .services
                .artist
                .get_artists(params.size, params.sort)
                .await?
        }
    };
    Ok(Json(SuccessResponse {
        status: Status::Success,
        message: artists,
    }))
}
