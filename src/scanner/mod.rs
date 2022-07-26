use crate::db::get_connection;
use anyhow::Result;
use std::sync::{atomic::AtomicBool, Arc};

pub mod scanner;
pub mod tag_helper;
#[derive(Clone)]
pub struct Scanner {
    scanning: bool,
}
impl Scanner {
    pub fn new() -> Result<Scanner> {
        let scanner = Scanner { scanning: false };
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
            let db_connection = get_connection().await.expect("Failed to connect to db");
            use std::time::Instant;
            let before = Instant::now();
            scanner::walk(&db_connection).await.unwrap();
            tracing::info!("Scan completed in: {:.2?}", before.elapsed());
            //self.update_scanning(AtomicBool::new(false))
            //val = Arc::new(false);
        });
    }
}
