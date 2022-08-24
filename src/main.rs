use anyhow::{Ok, Result};
use axum::{response::Html, routing::get, Extension, Router};

use db::DB;
use scanner::Scanner;
use sea_orm::DatabaseConnection;
use std::env;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::services::playlist::create_playlist;
mod db;
mod handlers;
mod scanner;
mod services;
#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub scanner: Scanner,
}
#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");

    // Setup tracing logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Connecting SQLite

    let db: DatabaseConnection = DB::new().await.unwrap().connect();
    /*     create_playlist(&db).await?;
     */

    let mut scan: Scanner = scanner::Scanner::new().unwrap();
    scan.start_scan();
    // build our application with a route and state

    let state = AppState {
        database: db,
        scanner: scan,
    };
    let app = Router::with_state(state)
        .route("/", get(handler))
        .route("/stream/:id", get(handlers::songs::stream_handler))
        .route("/albums/:id", get(handlers::albums::get_album))
        .route("/albums/:id/cover", get(handlers::albums::get_cover))
        .route("/albums", get(handlers::albums::get_all_albums))
        .route("/artists/:id", get(handlers::artists::get_artist))
        .route("/artists", get(handlers::artists::get_all_artists))
        .route("/playlists/:id", get(handlers::playlist::get_playlist))
        .layer(TraceLayer::new_for_http());

    // run it
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3030));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn handler(
    Extension(ref scanner): Extension<Scanner>,
    Extension(ref _db): Extension<DatabaseConnection>,
) -> Html<&'static str> {
    println!("{:?}", scanner.get_status());
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
    tracing::info!("Shutting down");
}
