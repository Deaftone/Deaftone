use anyhow::Result;
use core::panic;
use deaftone::{
    database,
    services::{
        album::AlbumService,
        artist::ArtistService,
        casting::{device::DeviceService, CHROMECAST_SERVICE_NAME},
        playlist::PlaylistService,
        scanner::ScanService,
        song::SongService,
        task::TaskType,
        DeaftoneService,
    },
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

    let sqlite_pool = match database::connect_db_sqlx().await {
        Ok(pool) => pool,
        Err(_) => database::connect_db_sqlx().await.unwrap(),
    };

    let album_service = AlbumService::new(database.clone());
    let artist_service = ArtistService::new(database.clone());
    let song_service = SongService::new(database.clone());
    let playlist_service = PlaylistService::new(database.clone(), song_service.clone());
    let device_service = DeviceService::new(database.clone());

    let scanner_service = ScanService::new(
        sqlite_pool,
        album_service.clone(),
        artist_service.clone(),
        song_service.clone(),
    );

    let services = DeaftoneService {
        song: song_service,
        playlist: playlist_service,
        artist: artist_service,
        album: album_service,
        scanner: scanner_service.clone(),
        device: device_service,
        task: tasks_send.clone(),
    };

    // Build app state
    let state = AppState {
        database,
        services: services,
    };

    // Spawn task service
    std::mem::drop(tokio::spawn(async move {
        deaftone::services::task::TaskService::new(tasks_receiver, scanner_service)
            .run()
            .await
    }));

    // Spawn casting service
    std::mem::drop(tokio::spawn(async move {
        deaftone::services::casting::Mdns::new(CHROMECAST_SERVICE_NAME)
            .await
            .expect("Failed to start casting service")
            .discover()
            .await
    }));

    // Spawn http service
    std::mem::drop(
        tokio::spawn(async move { deaftone::services::http::Server::run(state).await }).await,
    );

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
