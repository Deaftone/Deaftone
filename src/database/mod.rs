use anyhow::{anyhow, Result};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    ConnectOptions as SqlxConnectOptions, Pool, Sqlite,
};
use std::{fs, str::FromStr, time::Duration};

use crate::SETTINGS;

// Connect to sea-orm DatabaseConnection using SETTINGS.db_path as the database to connect too
pub async fn connect_to_db() -> Result<DatabaseConnection, anyhow::Error> {
    let db_path = SETTINGS.db_path.as_str();
    if fs::metadata(db_path).is_err() {
        fs::File::create(db_path).map_err(|e| anyhow!("Error creating file: {}", e))?;
    }
    let mut opt: ConnectOptions = ConnectOptions::new(format!("sqlite://{db_path}?mode=rwc"));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let pool: DatabaseConnection = sea_orm::Database::connect(opt).await?;
    migrate_up(&pool).await?;
    Ok(pool)
}
// Returns a sqlx database connection
pub async fn connect_db_sqlx() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_file = "deaftone.sqlite";
    let database_url = format!("sqlite://{database_file}");
    let pool_timeout = Duration::from_secs(30);
    let connection_options = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(false)
        .busy_timeout(pool_timeout)
        .disable_statement_logging()
        .clone();

    SqlitePoolOptions::new()
        .min_connections(5)
        .max_connections(10)
        .connect_with(connection_options)
        .await
}

async fn migrate_up(pool: &DatabaseConnection) -> Result<(), anyhow::Error> {
    Migrator::up(pool, None).await?;
    Ok(())
}
