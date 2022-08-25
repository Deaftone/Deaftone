use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Path, State},
    http::{Request, StatusCode},
    response::Response,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{services, AppState};

pub async fn stream_handler(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();
    let song: Option<entity::song::Model> = services::song::get_song(&state.database, song_id)
        .await
        .unwrap();
    match song {
        Some(f) => match ServeFile::new(f.path).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::NOT_FOUND,
                format!("Something went wrong: {}", err),
            )),
        },
        None => Err((StatusCode::NOT_FOUND, "Unable to find song".to_string())),
    }
}
