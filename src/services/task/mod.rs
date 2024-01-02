use std::time::Duration;
use tokio::sync::mpsc::Receiver;

use crate::database;

use super::scanner::{ScanService, ScanType};

#[derive(Debug)]
pub enum TaskType {
    ScanLibrary(ScanType),
    Shutdown,
    PopulateMetadata,
}

pub struct TaskService {
    pub task_queue: Vec<TaskType>,
    scanner: ScanService,
    receiver: Receiver<TaskType>,
}
// TaskServices listens for TaskType:: on the Receiver only runnig 1 task at a time
impl TaskService {
    pub fn new(receiver: Receiver<TaskType>, scanner: ScanService) -> TaskService {
        TaskService {
            task_queue: Vec::new(),
            scanner,
            receiver,
        }
    }
    pub async fn run(&mut self) {
        tracing::debug!("Connecting to DB");
        let sqlite_pool = match database::connect_db_sqlx().await {
            Ok(pool) => pool,
            Err(_) => database::connect_db_sqlx().await.unwrap(),
        };
        tracing::debug!("Connected DB");
        tracing::info!("Started task service");
        loop {
            if let Ok(task) = self.receiver.try_recv() {
                tracing::info!("Running task: {:?}", task);
                match task {
                    TaskType::ScanLibrary(ScanType::FullScan) => {
                        self.scanner.start_scan(ScanType::FullScan).await
                    }
                    TaskType::ScanLibrary(ScanType::PartialScan) => {
                        self.scanner.start_scan(ScanType::PartialScan).await
                    }
                    TaskType::PopulateMetadata => {
                        crate::services::metadata::scrap_metadata(&sqlite_pool).await
                    }
                    TaskType::Shutdown => break,
                }
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
