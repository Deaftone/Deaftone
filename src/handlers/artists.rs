use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};

use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ArtistResponse {
    id: String,
    name: String,
    image: String,
    bio: String,
    albums: Vec<entity::albums::Model>,
}
pub async fn get_artist(
    Path(artist_id): Path<String>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<Json<ArtistResponse>, (StatusCode, String)> {
    let artist = entity::artists::Entity::find_by_id(artist_id)
        .order_by_desc(entity::albums::Column::Year)
        .find_with_related(entity::albums::Entity)
        .all(db)
        .await
        .expect("Failed to get album");

    match artist.first() {
        Some(f) => {
            let artist_model = f.0.to_owned();
            let albums = f.1.to_owned();
            return Ok(Json(ArtistResponse {
                id: artist_model.id,
                name: artist_model.name,
                image: artist_model.image.unwrap_or_default(),
                bio: artist_model.bio.unwrap_or_default(),
                albums,
            }));
        }
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find album".to_owned(),
            ))
        }
    }
}
/* #[axum_macros::debug_handler]
 */

pub async fn get_all_artists(
    Extension(ref db): Extension<DatabaseConnection>,
) -> Json<Vec<entity::artists::Model>> {
    let artists: Vec<entity::artists::Model> = entity::artists::Entity::find()
        .all(db)
        .await
        .expect("Failed to get artists");
    return Json(artists);
}
