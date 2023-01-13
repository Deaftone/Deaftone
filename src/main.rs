use anyhow::{Ok, Result};
use axum::{response::Html, routing::get, routing::post, Router};
use deaftone::{database::Database, handlers, scanner::Scanner, AppState};
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            deaftone::handlers::albums::get_albums,
            deaftone::handlers::albums::get_album,
            deaftone::handlers::artists::get_artists,
            deaftone::handlers::artists::get_artist
        ),
        components(
            schemas(
                deaftone::handlers::albums::GetAllAlbums,
                deaftone::handlers::albums::AlbumResponse,
                entity::album::Model,
                deaftone::handlers::artists::GetAllArtists,
                entity::artist::Model,
            )
        ),
        tags(
            (name = "deaftone::handlers::albums", description = "Deaftone Albums API"),
            (name = "deaftone::handlers::artists", description = "Deaftone Artists API")
            //(name = "deaftone", description = "Deaftone API")
        )
    )]
    struct ApiDoc;

    let settings = match deaftone::settings::Settings::new() {
        std::result::Result::Ok(file) => file,
        Err(err) => {
            println!("Failed to load config {:}. Loading default config", err);
            deaftone::settings::Settings::new_default().unwrap()
        }
    };
    // Setup tracing logger
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            settings.log_level.clone(),
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
        settings.media_path.as_str(),
        settings.db_path.as_str()
    );

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

    let db = Database::new(&settings).await?;
    /*     create_playlist(&db).await?;
     */
    let mut scan: Scanner = Scanner::new(settings).unwrap();
    scan.start_scan();
    // build our application with a route and state
    let state = AppState {
        database: db.pool,
        scanner: scan,
    };
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/", get(handler))
        .route("/stream/:id", get(handlers::stream::stream_handler))
        .route(
            "/stream/transcode/:id",
            get(handlers::stream::transcode_stream_handler),
        )
        .route("/albums/:id", get(handlers::albums::get_album))
        .route("/songs/:id", get(handlers::songs::get_song))
        .route("/songs/:id/cover", get(handlers::songs::get_cover))
        .route("/songs/:id/like", post(handlers::songs::like_song))
        .route("/albums/:id/cover", get(handlers::albums::get_cover))
        .route("/albums", get(handlers::albums::get_albums))
        .route("/artists/:id", get(handlers::artists::get_artist))
        .route("/artists", get(handlers::artists::get_artists))
        .route("/playlists/:id", get(handlers::playlist::get_playlist))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // run it
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3030));
    tracing::debug!("Binding to socket");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    tracing::info!("listening on {}", addr);
    Ok(())
}

async fn handler() -> Html<&'static str> {
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
    tracing::info!("Shutting down");
}
