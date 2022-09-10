use std::process::Stdio;

use crate::{services, AppState};
use anyhow::Error;
use axum::{
    body::{boxed, Body, BoxBody, StreamBody},
    extract::{Path, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};

use futures::StreamExt;
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use tower::ServiceExt;
use tower_http::services::ServeFile;
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

pub async fn transcode_stream_handler(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    //"G:\\aa\\B\\Billie Eilish\\Billie Eilish - Happier Than Ever [2021] - WEB FLAC\\07. Lost Cause.flac"
    let song = services::song::get_song(&state.database, song_id)
        .await
        .unwrap();

    match song {
        Some(f) => {
            let mut child = Command::new("ffmpeg")
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .arg("-v")
                .arg("0")
                .arg("-i")
                .arg(f.path)
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
        None => return Err((StatusCode::NOT_FOUND, "Unable to find song".to_string())),
    }
}
