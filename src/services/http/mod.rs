use std::time::Duration;

use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::ToSchema;

use crate::AppState;

use self::{
    error::Status,
    handlers::{AlbumResponse, ArtistResponse, PlayListResponse},
};
pub mod error;
pub mod handlers;
pub struct Server {}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[aliases( AlbumResponseOpenApi = SuccessResponse<AlbumResponse>,
    AlbumsResponseOpenApi = SuccessResponse<Vec<entity::album::Model>>,
    ArtistResponseOpenApi = SuccessResponse<ArtistResponse>,
    ArtistsResponseOpenApi = SuccessResponse<Vec<entity::artist::Model>>,
    PlayListResponseOpenApi = SuccessResponse<PlayListResponse>,

)]
pub struct SuccessResponse<T> {
    pub status: Status,
    pub message: T,
}

impl Server {
    pub async fn run(state: AppState) -> Result<(), anyhow::Error> {
        // Build app router
        let app = Router::new()
            .route("/", get(_handler))
            .route("/stream/:id", get(handlers::streams::stream_handler))
            .route("/stream/cast/:id", get(handlers::streams::cast_handler))
            .route(
                "/stream/transcode/:id",
                get(handlers::streams::transcode_stream_handler),
            )
            .route("/songs/:id", get(handlers::songs::get_song))
            .route("/songs/:id/like", post(handlers::songs::like_song))
            .route("/albums", get(handlers::albums::get_albums))
            .route("/albums/:id", get(handlers::albums::get_album))
            .route("/albums/:id/cover", get(handlers::albums::get_cover))
            .route("/artists", get(handlers::artists::get_artists))
            .route("/artists/:id", get(handlers::artists::get_artist))
            .route("/playlists/:id", get(handlers::playlist::get_playlist))
            .route("/tasks", get(handlers::tasks::handle_task))
            .layer((
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                // requests don't hang forever.
                TimeoutLayer::new(Duration::from_secs(10)),
            ))
            .with_state(state)
            .into_make_service();

        // Starting listening
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
        tracing::debug!("Binding to socket");
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await;
        Ok(())
    }
}

async fn _handler(State(_state): State<AppState>) -> Html<&'static str> {
    /*     match state.task_service.send(TaskType::ScanLibrary).await {
        Ok(_) => {
            tracing::info!("Command sent to TaskService");
        }
        Err(err) => {
            tracing::error!("Failed to send command to TaskService {:}", err);
        }
    } */
    /*     match state.task_service.send(TaskType::PopulateMetadata).await {
        Ok(_) => {
            tracing::info!("Command sent to TaskService");
        }
        Err(err) => {
            tracing::error!("Failed to send command to TaskService {:}", err);
        }
    } */
    //println!("{:?}", SCAN_STATUS.lock().unwrap());
    Html("<h1>{Hello, World}!</h1>")
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("Shutting down HttpService");
}
