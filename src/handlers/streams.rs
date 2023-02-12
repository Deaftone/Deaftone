use std::process::Stdio;

use crate::{
    services::{self},
    AppState,
};

use axum::{
    body::{boxed, Body, BoxBody, StreamBody},
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
};

use futures::StreamExt;
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use super::ApiError;
#[utoipa::path(
    get,
    path = "/stream/{id}",
    params(
        ("id" = String, Path, description = "Song Id")
    ),
    responses(
        (status = 200, description = "Returns a song stream", body = BoxBody),
        (status = 404, description = "Song not found", body = String)

    )
)]
pub async fn stream_handler(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response<BoxBody>, ApiError> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();
    let song = services::song::get_song_by_id(&state.database, song_id).await?;
    match ServeFile::new(song.path).oneshot(res).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err(ApiError::FileNotFound(err)),
    }
}

pub async fn transcode_stream_handler(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    //"G:\\aa\\B\\Billie Eilish\\Billie Eilish - Happier Than Ever [2021] - WEB FLAC\\07. Lost Cause.flac"
    let song = services::song::get_song_by_id(&state.database, song_id).await?;
    let mut child = Command::new("ffmpeg")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .arg("-v")
        .arg("0")
        .arg("-i")
        .arg(song.path)
        .arg("-map")
        .arg("0:a:0")
        .arg("-codec:a")
        .arg("libmp3lame")
        .arg("-b:a")
        .arg("128k")
        .arg("-f")
        .arg("mp3")
        .arg("-")
        .spawn()
        .unwrap();

    //    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let stream = ReaderStream::new(stdout).boxed();
    let body = StreamBody::new(stream);
    Ok(body.into_response())
}
