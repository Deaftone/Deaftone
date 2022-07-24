use axum::{response::Html, routing::get, Extension, Router};
use chrono::Utc;
use entity;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ConnectOptions, Database, DatabaseConnection, Set,
};
use std::{env, fs};
use std::{net::SocketAddr, time::Duration};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");
    if fs::metadata("./deaftone.sqlite").is_err() {
        fs::File::create("./deaftone.sqlite").expect("Created file");
    }
    // Connecting SQLite

    let mut opt = ConnectOptions::new("sqlite://./deaftone.sqlite?mode=rwc".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;

    // Setup tracing logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler(Extension(ref pool): Extension<DatabaseConnection>) -> Html<&'static str> {
    let id = Uuid::new_v4();
    entity::artists::ActiveModel {
        id: Set(id.to_string().to_owned()),
        name: Set("test".to_owned()),
        image: NotSet,
        bio: NotSet,
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
    }
    .insert(pool)
    .await
    .expect("Failed to insert");
    Html("<h1>Hello, World!</h1>")
}
