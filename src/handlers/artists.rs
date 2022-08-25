use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use sea_orm::{EntityTrait, QueryOrder};
use serde::Serialize;

use crate::AppState;

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

pub async fn get_all_artists(State(state): State<AppState>) -> Json<Vec<entity::artist::Model>> {
    let artists: Vec<entity::artist::Model> = entity::artist::Entity::find()
        .all(&state.database)
        .await
        .expect("Failed to get artists");
    Json(artists)
}
