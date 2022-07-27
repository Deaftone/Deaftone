use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};

use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]

/*

#[sea_orm(primary_key, auto_increment = false)]
pub id: String,
pub name: String,
#[sea_orm(column_name = "artistName")]
pub artist_name: String,
pub year: i32,
#[sea_orm(column_name = "createdAt")]
pub created_at: String,
#[sea_orm(column_name = "updatedAt")]
pub updated_at: String,
#[sea_orm(column_name = "artistId")]
pub artist_id: Option<String>,*/

pub struct AlbumResponse {
    name: String,
    artistName: String,
    year: i32,
    songs: Vec<entity::songs::Model>,
}
pub async fn get_album(
    Path(album_id): Path<String>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<axum::Json<AlbumResponse>, (StatusCode, String)> {
    let album = entity::albums::Entity::find_by_id(album_id)
        .find_with_related(entity::songs::Entity)
        .all(db)
        .await
        .expect("Failed to get song");

    match album.first() {
        Some(f) => {
            let album_model = f.0.to_owned();
            let songs = f.1.to_owned();
            return Ok(Json(AlbumResponse {
                name: album_model.name,
                artistName: album_model.artist_name,
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

    //println!("{:?}", album.first().unwrap().to_owned().0);
    //println!("{:?}", album.first().unwrap())

    /*  match album.first() {
        Some(_) => {
            let album_model = album.first().unwrap().to_owned().0;
            let songs = album.first().unwrap().to_owned().1;
            Ok(Json(AlbumResponse {
                name: album_model.name,
                artistName: album_model.artist_name,
                year: album_model.year,
                songs,
            }));
        }
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find song"))),
    } */

    /*     let album_model = album.first().unwrap().to_owned().0;
       let songs = album.first().unwrap().to_owned().1;
    */
    /*     match album {
        Some(album) => {}
        None => {
            todo!()
        }
    } */
    /*     Ok(Json(AlbumResponse {
        name: album_model.name,
        artistName: album_model.artist_name,
        year: album_model.year,
        songs,
    })); */
    /*   let res = Request::builder().uri("/").body(Body::empty()).unwrap();
    let song = db::song_repo::get_song(db, song_id).await.unwrap();
    match song {
        Some(f) => match ServeFile::new(f.path).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::NOT_FOUND,
                format!("Something went wrong: {}", err),
            )),
        },
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find song"))),
    } */
}

/*

use axum::{
    body::BoxBody,
    extract::{Extension, Path},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]

/*

#[sea_orm(primary_key, auto_increment = false)]
pub id: String,
pub name: String,
#[sea_orm(column_name = "artistName")]
pub artist_name: String,
pub year: i32,
#[sea_orm(column_name = "createdAt")]
pub created_at: String,
#[sea_orm(column_name = "updatedAt")]
pub updated_at: String,
#[sea_orm(column_name = "artistId")]
pub artist_id: Option<String>,*/

pub struct AlbumResponse {
    name: String,
    artistName: String,
    year: i32,
    songs: Vec<entity::songs::Model>,
}
pub async fn get_album(
    Path(album_id): Path<String>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<Json<AlbumResponse, (StatusCode, String)>> {
    let album = entity::albums::Entity::find_by_id(album_id)
        .find_with_related(entity::songs::Entity)
        .all(db)
        .await
        .expect("Failed to get song");

    match album.first() {
        Some(f) => {
            let album_model = f.0.to_owned();
            let songs = f.1.to_owned();
            Ok(Json(AlbumResponse {
                name: album_model.name,
                artistName: album_model.artist_name,
                year: album_model.year,
                songs,
            })
            .into_response());
        }
        None => Err(Json((
            StatusCode::NOT_FOUND,
            format!("Unable to find album"),
        ))),
    }

    //println!("{:?}", album.first().unwrap().to_owned().0);
    //println!("{:?}", album.first().unwrap())

    /*  match album.first() {
        Some(_) => {
            let album_model = album.first().unwrap().to_owned().0;
            let songs = album.first().unwrap().to_owned().1;
            Ok(Json(AlbumResponse {
                name: album_model.name,
                artistName: album_model.artist_name,
                year: album_model.year,
                songs,
            }));
        }
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find song"))),
    } */

    /*     let album_model = album.first().unwrap().to_owned().0;
       let songs = album.first().unwrap().to_owned().1;
    */
    /*     match album {
        Some(album) => {}
        None => {
            todo!()
        }
    } */
    /*     Ok(Json(AlbumResponse {
        name: album_model.name,
        artistName: album_model.artist_name,
        year: album_model.year,
        songs,
    })); */
    /*   let res = Request::builder().uri("/").body(Body::empty()).unwrap();
    let song = db::song_repo::get_song(db, song_id).await.unwrap();
    match song {
        Some(f) => match ServeFile::new(f.path).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::NOT_FOUND,
                format!("Something went wrong: {}", err),
            )),
        },
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find song"))),
    } */
}

*/
