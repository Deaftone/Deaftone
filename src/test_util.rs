use std::{fs, time::Duration};

use crate::{services::device::DeviceService, *};
use axum::{routing::get, routing::post, Router};
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, DatabaseBackend, ExecResult, Statement};

use tower_http::trace::TraceLayer;

pub async fn app() -> Router {
    let database = new_seaorm_db().await.unwrap();
    seed_test_db(&database).await.unwrap();
    let (tasks_send, _tasks_receiver) = tokio::sync::mpsc::channel::<services::task::TaskType>(10);
    let services = DeaftoneService {
        device: DeviceService::new(database.clone()),
        task: tasks_send.clone(),
    };
    //scan.start_scan();
    let state = AppState {
        database,
        services: services,
    };
    Router::new()
        .route("/stream/:id", get(handlers::streams::stream_handler))
        .route(
            "/stream/transcode/:id",
            get(handlers::streams::transcode_stream_handler),
        )
        .route("/albums/:id", get(handlers::albums::get_album))
        .route("/songs/:id", get(handlers::songs::get_song))
        .route("/songs/:id/like", post(handlers::songs::like_song))
        .route("/albums/:id/cover", get(handlers::albums::get_cover))
        .route("/albums", get(handlers::albums::get_albums))
        .route("/artists/:id", get(handlers::artists::get_artist))
        .route("/artists", get(handlers::artists::get_artists))
        .route("/playlists/:id", get(handlers::playlist::get_playlist))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
pub async fn new_seaorm_db() -> Result<DatabaseConnection, anyhow::Error> {
    let mut opt: ConnectOptions = ConnectOptions::new(String::from("sqlite::memory:"));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let pool: DatabaseConnection = sea_orm::Database::connect(opt).await?;
    Migrator::up(&pool, None).await?;
    Ok(pool)
}
pub async fn seed_test_db(db: &DatabaseConnection) -> Result<ExecResult, DbErr> {
    let seed: String = fs::read_to_string("tests/test_seed.sql")
        .unwrap()
        .parse()
        .unwrap();

    let exec_res: ExecResult = db
        .execute(Statement::from_string(DatabaseBackend::Sqlite, seed))
        .await?;
    Ok(exec_res)
}
