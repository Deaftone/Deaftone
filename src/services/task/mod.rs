use std::time::Duration;
use tokio::sync::mpsc::Receiver;

use crate::scanner;

#[derive(Debug)]
pub enum TaskType {
    ScanLibrary,
    Shutdown,
}

pub struct TaskService {
    pub task_queue: Vec<TaskType>,
    receiver: Receiver<TaskType>,
}
// TaskServices listens for TaskType:: on the Receiver only runnig 1 task at a time
impl TaskService {
    pub fn new(receiver: Receiver<TaskType>) -> TaskService {
        TaskService {
            task_queue: Vec::new(),
            receiver,
        }
    }
    pub async fn run(&mut self) {
        tracing::info!("Started task service");
        loop {
            if let Ok(task) = self.receiver.try_recv() {
                tracing::info!("Running task: {:?}", TaskType::ScanLibrary);

                match task {
                    TaskType::ScanLibrary => scanner::start_scan().await,
                    TaskType::Shutdown => break,
                }
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
