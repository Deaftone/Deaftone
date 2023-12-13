use anyhow::Result;
use core::panic;
use deaftone::{
    services::{device::DeviceService, task::TaskType, DeaftoneService},
    AppState, SETTINGS,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing logger
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            SETTINGS.log_level.clone(),
        ))
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
    tracing::info!(
        "
██████╗ ███████╗ █████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗
██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝
██║  ██║█████╗  ███████║█████╗     ██║   ██║   ██║██╔██╗ ██║█████╗  
██║  ██║██╔══╝  ██╔══██║██╔══╝     ██║   ██║   ██║██║╚██╗██║██╔══╝  
██████╔╝███████╗██║  ██║██║        ██║   ╚██████╔╝██║ ╚████║███████╗
╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝        ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝
Version: {:} | Media Directory: {:} | Database: {:}",
        env!("CARGO_PKG_VERSION"),
        SETTINGS.media_path.as_str(),
        SETTINGS.db_path.as_str()
    );

    let database = deaftone::database::connect_to_db().await?;

    // Create task service with channel length of 10
    let (tasks_send, tasks_receiver) =
        tokio::sync::mpsc::channel::<deaftone::services::task::TaskType>(10);

    let services = DeaftoneService {
        device: DeviceService::new(database.clone()),
        task: tasks_send.clone(),
    };
    // Build app state
    let state = AppState { database, services };

    // Spawn task service
    let _ = tokio::spawn(async move {
        deaftone::services::task::TaskService::new(tasks_receiver)
            .run()
            .await
    });

    // Spawn casting service
    let _ = tokio::spawn(async move { deaftone::services::casting::run_discover().await });

    // Spawn http service
    let _ = tokio::spawn(async move { deaftone::services::http::Server::run(state).await }).await;

    // Send shutdown signal to tasks service
    match tasks_send.send(TaskType::Shutdown).await {
        Ok(_e) => {
            tracing::info!("Shutting down TaskService. Please wait for task queue completion")
        }
        Err(err) => {
            tracing::error!("Failed to send shutdown command to TaskService {:}", err);
            panic!("Failed to shutdown TaskService")
        }
    };

    tracing::info!("Goodbye!");
    Ok(())
}
