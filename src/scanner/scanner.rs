use chrono::Utc;
use entity;
use migration::OnConflict;
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::scanner::tag_helper;
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("An error occured: {}; skipped.", e);
                continue;
            }
        }
    };
}
pub async fn walk(db: &DatabaseConnection) -> anyhow::Result<()> {
    tracing::info!("Starting scan");

    let current_dir = "H:\\Music";
    for entry in WalkDir::new(current_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".flac") {
            let metadata = skip_fail!(tag_helper::get_metadata(
                entry.path().to_string_lossy().to_string()
            ));

            let id = Uuid::new_v4();
            let song = entity::songs::ActiveModel {
                id: Set(id.to_string()),
                path: Set(metadata.path),
                title: Set(metadata.name),
                disk: NotSet,
                artist: Set(metadata.album_artist),
                album_name: Set(metadata.album),
                codec: NotSet,
                sample_rate: NotSet,
                bits_per_sample: NotSet,
                track: NotSet,
                year: Set(Some(metadata.year)),
                label: NotSet,
                music_brainz_recording_id: NotSet,
                music_brainz_artist_id: NotSet,
                music_brainz_track_id: NotSet,
                created_at: Set(Utc::now().naive_local().to_string()),
                updated_at: Set(Utc::now().naive_local().to_string()),
                album_id: NotSet,
            };

            entity::songs::Entity::insert(song)
                .on_conflict(
                    // on conflict do nothing
                    OnConflict::column(entity::songs::Column::Path)
                        .do_nothing()
                        .to_owned(),
                )
                .exec(db)
                .await
                .expect("Failed to insert song");
        }
    }

    Ok(())
}
