use anyhow::Result;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

use crate::db::DB;

pub mod scanner;
pub mod tag_helper;
#[derive(Clone)]
pub struct Scanner {
    scanning: bool,
}
impl Scanner {
    pub fn new() -> Result<Scanner> {
        let scanner: Scanner = Scanner { scanning: false };
        Ok(scanner)
    }

    fn update_scanning(&mut self, status: bool) {
        self.scanning = status
    }
    pub fn get_status(&self) -> bool {
        return self.scanning;
    }
    pub fn start_scan(&mut self) {
        tokio::spawn(async move {
            let db: DatabaseConnection = DB::new().await.unwrap().connect();

            use std::time::Instant;
            let before: Instant = Instant::now();

            scanner::walk_partial(&db).await.unwrap();
            //scanner::walk_full(&db).await.unwrap();
            /*             scanner::walk(&db).await.unwrap();
            scanner::create_albums(&db).await;
            scanner::create_artists(&db).await; */
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
        });
    }
}
