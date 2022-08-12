use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{fs, time::Duration};

#[derive(Clone)]
pub struct DB {
    pool: DatabaseConnection,
}
impl DB {
    pub async fn new() -> Result<DB> {
        if fs::metadata("./deaftone.sqlite").is_err() {
            fs::File::create("./deaftone.sqlite").expect("Created file");
        }
        let mut opt: ConnectOptions =
            ConnectOptions::new("sqlite://./deaftone.sqlite?mode=rwc".to_owned());
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
        return self.pool;
    }
    async fn migrate_up(&self) -> Result<()> {
        Migrator::up(&self.pool, None).await?;
        Ok(())
    }
}
