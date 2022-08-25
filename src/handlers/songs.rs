use axum::{
    body::{boxed, Body, BoxBody, Full},
    extract::{Path, State},
    http::{header, Request, Response, StatusCode},
};

use axum_macros::debug_handler;
use include_dir::{include_dir, Dir};
use sea_orm::EntityTrait;

use crate::{services, AppState};

pub async fn get_song(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();

        .await
        .unwrap();

    match album {
                Ok(res) => Ok(res.map(boxed)),
                Err(err) => Err((
                    StatusCode::NOT_FOUND,
                    format!("Something went wrong: {}", err),
                )),
            }
        }
    }
}
