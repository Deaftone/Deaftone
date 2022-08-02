use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};

use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
pub struct AlbumResponse {
    id: String,
    name: String,
    artist_name: String,
    year: i32,
    songs: Vec<entity::songs::Model>,
}
pub async fn get_album(
    Path(album_id): Path<String>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<Json<AlbumResponse>, (StatusCode, String)> {
    let album = entity::albums::Entity::find_by_id(album_id)
        .find_with_related(entity::songs::Entity)
        .all(db)
        .await
        .expect("Failed to get album");

    match album.first() {
        Some(f) => {
            let album_model = f.0.to_owned();
            let songs = f.1.to_owned();
            return Ok(Json(AlbumResponse {
                id: album_model.id,
                name: album_model.name,
                artist_name: album_model.artist_name,
                year: album_model.year,
                songs,
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

pub async fn get_all_albums(
    Extension(ref db): Extension<DatabaseConnection>,
) -> Json<Vec<entity::albums::Model>> {
    let albums: Vec<entity::albums::Model> = entity::albums::Entity::find()
        .all(db)
        .await
        .expect("Failed to get albums");
    return Json(albums);
}
