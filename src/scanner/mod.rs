use std::time::Instant;

use anyhow::Result;
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Statement};

use crate::{database::Database, SCAN_STATUS};
pub mod scanner;
pub mod tag_helper;
#[derive(Clone)]
pub struct Scanner {}
impl Scanner {
    pub fn new() -> Result<Scanner> {
        let scanner: Scanner = Scanner {};
        Ok(scanner)
    }
    /*     fn update_scanning(&mut self, status: Arc<AtomicBool>) {
        self.scanning = status
    }
    pub fn get_status(&self) -> bool {
        self.scanning.load(Ordering::Relaxed)
    } */

    pub fn start_scan(&mut self) {
        let start = Instant::now();

        tokio::spawn(async move {
            SCAN_STATUS
                .lock()
                .unwrap()
                .store(true, std::sync::atomic::Ordering::Relaxed);
            let db: DatabaseConnection = Database::new().await.unwrap().pool;
            let before: Instant = Instant::now();
            // Run full scan if no songs
            let count: usize = entity::song::Entity::find().count(&db).await.unwrap();
            if count > 0 {
                tracing::info!("Starting partial scan");
                scanner::walk_partial(&db).await.unwrap();
            } else {
                tracing::info!("Starting full scan");
                scanner::walk_full(&db).await.unwrap();
            }
            tracing::info!("Scan completed in: {:.2?}", before.elapsed());

            // Cleanup orphans
            db.execute(Statement::from_string(
                db.get_database_backend(),
                "delete from albums where id not in (
                select albumId from songs
              )"
                .to_owned(),
            ))
            .await
            .unwrap();
            db.execute(Statement::from_string(
                db.get_database_backend(),
                "delete from artists where id not in (
                select artistId from albums
              )"
                .to_owned(),
            ))
            .await
            .unwrap();
            let duration = start.elapsed();
            println!("Time elapsed in scan() is: {:?}", duration);
            SCAN_STATUS
                .lock()
                .unwrap()
                .store(false, std::sync::atomic::Ordering::Relaxed);
        });
    }
}
