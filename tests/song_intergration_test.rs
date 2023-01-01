use axum::{response::Html, routing::get, routing::post, Router};
#[allow(dead_code)]
use deaftone::{database::Database, handlers, scanner::Scanner, AppState};
use tower_http::trace::TraceLayer;
async fn app() -> Router {
    let db = Database::new().await.unwrap();
    /*     create_playlist(&db).await?;
     */
    let mut scan: Scanner = Scanner::new().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use std::net::{SocketAddr, TcpListener};
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn the_real_deal() {
        let listener = TcpListener::bind("127.0.0.1:8000".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app().await.into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{}", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, World!");
    }
}
