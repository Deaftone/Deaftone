use axum::{
    body::{self, boxed, Body, BoxBody, Full},
    extract::{Extension, Path},
    http::{header, Request, Response, StatusCode},
    Json,
};

use include_dir::{include_dir, Dir};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use tower::ServiceExt;
use tower_http::services::ServeFile;
static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/resources");
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AlbumResponse {
    id: String,
    name: String,
    artist: String,
    artistId: String,
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
                artist: album_model.artist_name,
                artistId: album_model.artist_id.unwrap_or_default(),
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
pub async fn get_cover(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(album_id): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = Request::builder().uri("/").body(Body::empty()).unwrap();

    let album = entity::albums::Entity::find_by_id(album_id)
        .one(db)
        .await
        .unwrap();

    match album {
        Some(f) => {
            if f.cover.is_some() {
                match ServeFile::new(f.cover.unwrap()).oneshot(res).await {
                    Ok(res) => Ok(res.map(boxed)),
                    Err(err) => Err((
                        StatusCode::NOT_FOUND,
                        format!("Something went wrong: {}", err),
                    )),
                }
            } else {
                let unknown_album = ASSETS.get_file("unknown_album.jpg").unwrap();
                let body = boxed(Full::from(unknown_album.contents()));
                Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "image/jpg")
                    .body(body)
                    .unwrap())
            }
        }
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find album"))),
    }
    /*     match album {
        Some(f) => match ServeFile::new(f.cover.unwrap_or_default())
            .oneshot(res)
            .await
        {
            Ok(res) => Ok(res.map(boxed)),
            Err(_err) => {
                let body = boxed(Full::from(unknown_album.contents()));
                Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "image/jpg")
                    .body(body)
                    .unwrap())
            }
        },
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find album"))),
    } */
}
pub async fn get_all_albums(
    Extension(ref db): Extension<DatabaseConnection>,
) -> Json<Vec<entity::albums::Model>> {
    let albums: Vec<entity::albums::Model> = entity::albums::Entity::find()
        .all(db)
        .await
        .expect("Failed to get albums");
    return Json(albums);
}
