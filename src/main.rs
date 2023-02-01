use anyhow::Result;
use axum::{extract::State, response::Html, routing::get, routing::post, Router};
use core::panic;
use deaftone::{handlers, services::task::TaskType, AppState, SETTINGS};
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing logger
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            SETTINGS.log_level.clone(),
        ))
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
    tracing::info!(
        "
██████╗ ███████╗ █████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗
██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝
██║  ██║█████╗  ███████║█████╗     ██║   ██║   ██║██╔██╗ ██║█████╗  
██║  ██║██╔══╝  ██╔══██║██╔══╝     ██║   ██║   ██║██║╚██╗██║██╔══╝  
██████╔╝███████╗██║  ██║██║        ██║   ╚██████╔╝██║ ╚████║███████╗
╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝        ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝
Version: {:} | Media Directory: {:} | Database: {:}",
        env!("CARGO_PKG_VERSION"),
        SETTINGS.media_path.as_str(),
        SETTINGS.db_path.as_str()
    );

    let db = deaftone::database::connect_to_db().await?;
    // Create task service with channel length of 10
    let (tasks_send, tasks_receiver) =
        tokio::sync::mpsc::channel::<deaftone::services::task::TaskType>(10);
    let mut task_manager = deaftone::services::task::TaskService::new(tasks_receiver);
    // Spawn task service
    let _task_manager_thread = tokio::spawn(async move { task_manager.run().await });
    // Build app state
    let state = AppState {
        database: db,
        task_service: tasks_send.clone(),
    };
    // Build app router
    let app = Router::new()
        .route("/", get(handler))
        .route("/stream/:id", get(handlers::stream::stream_handler))
        .route(
            "/stream/transcode/:id",
            get(handlers::stream::transcode_stream_handler),
        )
        .route("/songs/:id", get(handlers::songs::get_song))
        .route("/songs/:id/cover", get(handlers::songs::get_cover))
        .route("/songs/:id/like", post(handlers::songs::like_song))
        .route("/albums", get(handlers::albums::get_albums))
        .route("/albums/:id", get(handlers::albums::get_album))
        .route("/albums/:id/cover", get(handlers::albums::get_cover))
        .route("/artists", get(handlers::artists::get_artists))
        .route("/artists/:id", get(handlers::artists::get_artist))
        .route("/playlists/:id", get(handlers::playlist::get_playlist))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Starting listening
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3030));
    tracing::debug!("Binding to socket");
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Send shutdown signal to tasks service
    match tasks_send.send(TaskType::Shutdown).await {
        Ok(_e) => {
            tracing::info!("Shutting down TaskService. Please wait for task queue completion")
        }
        Err(err) => {
            tracing::error!("Failed to send shutdown command to TaskService {:}", err);
            panic!("Failed to shutdown TaskService")
        }
    };
    tracing::info!("Goodbye!");
    Ok(())
}

async fn handler(State(state): State<AppState>) -> Html<&'static str> {
    match state.task_service.send(TaskType::ScanLibrary).await {
        Ok(_) => {
            tracing::info!("Command sent to TaskService");
        }
        Err(err) => {
            tracing::error!("Failed to send command to TaskService {:}", err);
        }
    }
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
    tracing::info!("Shutting down HTTP service");
}
