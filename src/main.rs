use anyhow::{Ok, Result};
use axum::{response::Html, routing::get, Router};

use db::DB;
use lazy_static::lazy_static;
use scanner::Scanner;
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::{env, sync::Mutex};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

lazy_static! {
    static ref scan_status: Mutex<AtomicBool> = Mutex::new(AtomicBool::new(false));
    static ref SETTINGS: settings::Settings =
        settings::Settings::new().expect("Failed to load config: ");
}
mod db;
mod handlers;
mod scanner;
mod services;
pub mod settings;
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

    // Setup config

    /*    let mut s = Settings::new

    match settings {
        core::result::Result::Ok(config) => {}
        Err(e) => {
            tracing::error!("Failed to load config.json. Error: {}", e);
            std::process::exit(0);
        }
    } */
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
        .route("/stream/:id", get(handlers::stream::stream_handler))
        .route(
            "/stream/transcode/:id",
            get(handlers::stream::transcode_stream_handler),
        )
        .route("/albums/:id", get(handlers::albums::get_album))
        .route("/songs/:id", get(handlers::songs::get_song))
        .route("/songs/:id/cover", get(handlers::songs::get_cover))
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

async fn handler() -> Html<&'static str> {
    println!("{:?}", scan_status.lock().unwrap());
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
