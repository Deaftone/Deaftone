use anyhow::Result;
use chrono::{DateTime, Utc};
use entity;
use migration::OnConflict;
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use std::time::SystemTime;
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

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

pub async fn process_directory(
    db: &DatabaseConnection,
    entry: DirEntry,
    path: String,
    dirs: &Vec<entity::directories::Model>,
) -> Result<bool> {
    let index = dirs.iter().position(|r| r.path == path);
    let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
    let mtime: DateTime<Utc> = fmtime.into();
    if index == None {
        tracing::info!("New dir found scanning... {:?}", path);
        insert_directory(&path, &mtime, db).await?;
        return Ok(true);
    } else if dirs[index.unwrap()].mtime > mtime.naive_utc() {
        tracing::info!("Dir changed {}", path);
        insert_directory(&path, &mtime, db).await?;
        return Ok(true);
    } else {
        tracing::debug!("Dir hasn't {}", path);
        Ok(false)
    }
}

pub async fn insert_directory(
    path: &String,
    mtime: &DateTime<Utc>,
    db: &DatabaseConnection,
) -> Result<()> {
    let init_time: String = Utc::now().naive_local().to_string();
    let dir = entity::directories::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        path: Set(path.to_owned()),
        mtime: Set(mtime.naive_utc()),
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
    };

    entity::directories::Entity::insert(dir)
        .on_conflict(
            OnConflict::column(entity::directories::Column::Path)
                .update_column(entity::directories::Column::UpdatedAt)
                .update_column(entity::directories::Column::Mtime)
                .to_owned(),
        )
        .exec(db)
        .await
        .expect("Failed to insert dir");
    Ok(())
}
pub async fn walk(db: &DatabaseConnection) -> Result<()> {
    tracing::info!("Starting scan");
    let dirs = entity::directories::Entity::find().all(db).await?;
    let current_dir = "G:\\aa";
    for entry in WalkDir::new(current_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: String = entry.path().to_string_lossy().to_string();
        let mut should_process: bool = false;
        if entry.file_type().is_dir() {
            should_process = process_directory(db, entry.clone(), path.clone(), &dirs).await?;
        }

        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".flac") {
            let metadata = skip_fail!(tag_helper::get_metadata(path));

            let id = Uuid::new_v4();

            let init_time: String = Utc::now().naive_local().to_string();

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
                created_at: Set(init_time.to_owned()),
                updated_at: Set(init_time),
                album_id: NotSet,
            };

            entity::songs::Entity::insert(song)
                .on_conflict(
                    // on conflict do nothing
                    OnConflict::column(entity::songs::Column::Path)
                        .update_column(entity::songs::Column::UpdatedAt)
                        .to_owned(),
                )
                .exec(db)
                .await
                .expect("Failed to insert song");
        }
    }

    Ok(())
}
