use crate::{database::Database, *};
use axum::{routing::get, routing::post, Router};
use tower_http::trace::TraceLayer;
pub async fn app() -> Router {
    let settings = match settings::Settings::new() {
        std::result::Result::Ok(file) => file,
        Err(err) => {
            println!("Failed to load config {:}. Loading default config", err);
            settings::Settings::new_default().unwrap()
        }
    };
    let db = Database::new(&settings).await.unwrap();
    /*     create_playlist(&db).await?;
     */
    let mut scan: Scanner = Scanner::new(settings).unwrap();
    scan.start_scan();
    // build our application with a route and state
    let state = AppState {
        database: db.pool,
        scanner: scan,
    };
    Router::new()
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
        .with_state(state)
}
