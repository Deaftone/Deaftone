use std::time::Duration;
use tokio::sync::mpsc::Receiver;

use crate::scanner::Scanner;
#[derive(Debug)]
pub enum TaskType {
    ScanLibrary,
    Shutdown,
}

pub struct TaskService {
    pub task_queue: Vec<TaskType>,
    receiver: Receiver<TaskType>,
    scanner: Scanner,
}
impl TaskService {
    pub fn new(receiver: Receiver<TaskType>, scanner: Scanner) -> TaskService {
        TaskService {
            task_queue: Vec::new(),
            receiver,
            scanner,
        }
    }
    pub async fn run(&mut self) {
        tracing::info!("Started task service");
        loop {
            if let Ok(task) = self.receiver.try_recv() {
                tracing::info!("Running task: {:?}", TaskType::ScanLibrary);

                match task {
                    TaskType::ScanLibrary => self.scanner.start_scan().await,
                    TaskType::Shutdown => break,
                }
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
