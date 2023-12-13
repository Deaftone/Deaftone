use std::{process::Stdio, str::FromStr};

use crate::{
    services::{self},
    ApiError, AppState,
};

use axum::{
    body::{boxed, Body, BoxBody, StreamBody},
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
    Json,
};
use serde::de;

use super::TestResponse;
use futures::StreamExt;
use hyper::StatusCode;
use rust_cast::{
    channels::{
        heartbeat::HeartbeatResponse,
        media::{Image, Media, Metadata, MusicTrackMediaMetadata, StreamType},
        receiver::CastDeviceApp,
    },
    CastDevice, ChannelMessage,
};
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use tower::ServiceExt;
use tower_http::services::ServeFile;
const SERVICE_TYPE: &str = "_googlecast._tcp.local.";
const DEFAULT_DESTINATION_ID: &str = "receiver-0";
#[utoipa::path(
    get,
    path = "/stream/{id}",
    params(
        ("song_id" = String, Path, description = "Song Id")
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
    let song = services::song::get_song_by_id(&state.database, &song_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to stream: \"{:?}\" for {:}", e, song_id);
            e
        })?;

    match ServeFile::new(&song.path).oneshot(res).await {
        Ok(res) => {
            if res.status() == StatusCode::NOT_FOUND {
                Err(ApiError::FileNotFound(song.path))
            } else {
                Ok(res.map(boxed))
            }
        }
        Err(err) => Err(ApiError::UnknownError(err.to_string())),
    }
}

pub async fn cast_handler(
    Path(device_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<TestResponse>, ApiError> {
    let song_id = "931e4d26-bb71-42e4-ac1a-2fd41c16ee79";
    let device = state
        .services
        .device
        .get_cast_device_by_id(&device_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get device: \"{:?}\" for {:}", e, device_id);
            e
        })?;
    let song = services::song::get_song_by_id(&state.database, &song_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to stream: \"{:?}\" for {:}", e, song_id);
            e
        })?;
    let cast_device = match CastDevice::connect_without_host_verification(device.address_v4, 8009) {
        Ok(cast_device) => cast_device,
        Err(err) => panic!("Could not establish connection with Cast Device: {:?}", err),
    };

    cast_device
        .connection
        .connect(DEFAULT_DESTINATION_ID.to_string())
        .unwrap();
    cast_device.heartbeat.ping().unwrap();

    let media_type = "audio/flac";

    let media_stream_type = StreamType::from_str("buffered").unwrap();

    play_media(
        &cast_device,
        &CastDeviceApp::from_str("default").unwrap(),
        "http://192.168.1.2:3030/stream/931e4d26-bb71-42e4-ac1a-2fd41c16ee79".to_owned(),
        media_type.to_owned(),
        media_stream_type,
        song,
    );

    Ok(Json(TestResponse {
        state: "test".to_owned(),
    }))
}

fn play_media(
    device: &CastDevice,
    app_to_run: &CastDeviceApp,
    media: String,
    media_type: String,
    media_stream_type: StreamType,
    song: entity::song::Model,
) {
    let app = device.receiver.launch_app(app_to_run).unwrap();

    device
        .connection
        .connect(app.transport_id.as_str())
        .unwrap();

    let status = device
        .media
        .load(
            app.transport_id.as_str(),
            app.session_id.as_str(),
            &Media {
                content_id: media,
                content_type: media_type,
                stream_type: media_stream_type,
                duration: None,
                metadata: Some(Metadata::MusicTrack(MusicTrackMediaMetadata {
                    title: Some(song.title.clone()),
                    artist: Some(song.artist),
                    album_name: Some(song.album_name),
                    album_artist: Some(song.album_artist.unwrap()),
                    track_number: Some(1),
                    disc_number: Some(1),
                    images: vec![Image {
                        url: "https://upload.wikimedia.org/wikipedia/en/8/85/AKonvicted.jpg"
                            .to_string(),
                        dimensions: None,
                    }],
                    release_date: None,
                    composer: None,
                })),
            },
        )
        .unwrap();
}

pub async fn transcode_stream_handler(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    //"G:\\aa\\B\\Billie Eilish\\Billie Eilish - Happier Than Ever [2021] - WEB FLAC\\07. Lost Cause.flac"
    let song = services::song::get_song_by_id(&state.database, &song_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to stream: \"{:?}\" for {:}", e, song_id);
            e
        })?;
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
