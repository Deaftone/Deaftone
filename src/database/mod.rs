use anyhow::{anyhow, Result};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::{fs, time::Duration};

use crate::SETTINGS;
#[derive(Clone)]
pub struct Database {
    pub pool: DatabaseConnection,
}
impl Database {
    pub async fn new() -> Result<Database, anyhow::Error> {
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
        let db = Database { pool };
        db.migrate_up().await?;
        Ok(db)
    }

    async fn migrate_up(&self) -> Result<(), anyhow::Error> {
        Migrator::up(&self.pool, None).await?;
        Ok(())
    }
}
