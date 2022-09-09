use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{fs, time::Duration};

use crate::SETTINGS;

#[derive(Clone)]
pub struct DB {
    pool: DatabaseConnection,
}
impl DB {
    pub async fn new() -> Result<DB> {
        let db_path = SETTINGS.db_path.as_str();
        if fs::metadata(db_path).is_err() {
            fs::File::create(db_path).expect("Created file");
        }
        let mut opt: ConnectOptions =
            ConnectOptions::new(format!("sqlite://{}?mode=rwc", db_path));
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let pool: DatabaseConnection = Database::connect(opt).await?;
        let db = DB { pool };
        db.migrate_up().await?;
        Ok(db)
    }

    pub fn connect(self) -> DatabaseConnection {
        self.pool
    }
    async fn migrate_up(&self) -> Result<()> {
        Migrator::up(&self.pool, None).await?;
        Ok(())
    }
}
