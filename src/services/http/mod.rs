use std::net::SocketAddr;

use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{handlers, AppState};

pub struct Server {}

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
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .with_state(state)
            .into_make_service();

        // Starting listening
        let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3030));
        tracing::debug!("Binding to socket");
        tracing::info!("listening on {}", addr);
        let _ = axum::Server::bind(&addr)
            .serve(app)
            .with_graceful_shutdown(http_shutdown_signal())
            .await;
        Ok(())
    }
}

async fn http_shutdown_signal() {
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
