use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{fs, time::Duration};

pub mod song_repo;

pub async fn get_connection() -> Result<DatabaseConnection> {
    if fs::metadata("./deaftone.sqlite").is_err() {
        fs::File::create("./deaftone.sqlite").expect("Created file");
    }
    let mut opt = ConnectOptions::new("sqlite://./deaftone.sqlite?mode=rwc".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;
    Ok(db)
}

pub async fn migrate_up(db: &DatabaseConnection) -> Result<()> {
    Migrator::up(db, None).await?;
    Ok(())
}
