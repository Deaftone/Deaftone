use tokio::sync::mpsc::Sender;

use self::{
    album::AlbumService, casting::device::DeviceService, http::handlers::ArtistResponse,
    scanner::ScanService, task::TaskType,
};

pub mod album;
pub mod artist;
pub mod casting;
pub mod http;
pub mod metadata;
pub mod playlist;
pub mod scanner;
pub mod song;
pub mod task;
// Rewrite DbArtist to ArtistResponse
pub type DbArtist = ArtistResponse;
// Convert sea_orm::DbErr into our custom ServiceError allows ? to be called on sea_orm querys such as find_by_id().await? etc. Pushing up the error to the caller.
// Which most of the time is a web handler. Which with impl IntoResponse for ServiceError can convert these errors into errors with response codes and good messages
#[derive(Clone)]
pub struct DeaftoneService {
    pub scanner: ScanService,
    pub album: AlbumService,
    pub device: DeviceService,
    pub task: Sender<TaskType>,
}
