use crate::scanner::tag_helper::{self};
use crate::{services, SETTINGS};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use entity;
use migration::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::path::PathBuf;
use std::result::Result::Ok;
use std::time::SystemTime;
use tokio::fs;
use tokio_stream::StreamExt;
use uuid::Uuid;
use walkdir::WalkDir;
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

pub async fn walk_partial(db: &DatabaseConnection) -> Result<()> {
    let mut dirs_stream = entity::directorie::Entity::find().stream(db).await?;
    while let Some(item) = dirs_stream.next().await {
        let item: entity::directorie::Model = item?;
        let meta = fs::metadata(&item.path).await;
        let is_empty = PathBuf::from(&item.path)
            .read_dir()
            .map(|mut i| i.next().is_none())
            .unwrap_or(false);
        if meta.is_ok() && !is_empty {
            let _ftime: SystemTime = meta.unwrap().modified().unwrap();
            let ftime: DateTime<Utc> = _ftime.into();

            let dbtime: NaiveDateTime = item.mtime;

            if ftime.naive_utc() > dbtime {
                tracing::info!("Dir changed {}", item.path);
                walk_dir(db, item.path).await?;
            } else {
                tracing::debug!("Dir hasn't {}", item.path);
            }
        } else {
            tracing::info!("Dropping all items for path {}", item.path);
            // Drop all songs for missing path

            entity::song::Entity::delete_many()
                .filter(entity::song::Column::Path.contains(&item.path))
                .exec(db)
                .await?;
            entity::directorie::Entity::delete_many()
                .filter(entity::directorie::Column::Path.contains(&item.path))
                .exec(db)
                .await?;
        }
    }

    Ok(())
}

pub async fn walk_dir(db: &DatabaseConnection, dir: String) -> Result<()> {
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: String = entry.path().to_string_lossy().to_string();

        if entry.file_type().is_dir() {
            let is_empty = PathBuf::from(&path)
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false);
            let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
            let mtime: DateTime<Utc> = fmtime.into();
            if !is_empty {
                insert_directory(&path, &mtime, db).await?;
            }
        }
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".flac") {
            let metadata = skip_fail!(tag_helper::get_metadata(path.to_owned()));
            skip_fail!(services::song::create_or_update(db, metadata).await);
        }
        if f_name.contains("cover.") {
            services::album::update_cover_for_path(
                db,
                path,
                entry.path().parent().unwrap().to_string_lossy().to_string(),
            )
            .await?;
        }
    }
    Ok(())
}
pub async fn walk_full(db: &DatabaseConnection) -> Result<()> {
    tracing::info!("Starting scan");
    //let dirs: Vec<entity::directories::Model> = entity::directories::Entity::find().all(db).await?;
    let current_dir: &str = SETTINGS.media_path.as_str();
    for entry in WalkDir::new(current_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: String = entry.path().to_string_lossy().to_string();
        if entry.file_type().is_dir() {
            if entry.file_type().is_dir() {
                let is_empty = PathBuf::from(&path)
                    .read_dir()
                    .map(|mut i| i.next().is_none())
                    .unwrap_or(false);
                let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
                let mtime: DateTime<Utc> = fmtime.into();
                if !is_empty {
                    let start = Instant::now();

                    insert_directory(&path, &mtime, db).await?;
                    let duration = start.elapsed();
                    println!("Time elapsed in insert_directory() is: {:?}", duration);
                }
            }
        }
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".flac") {
            let metadata = skip_fail!(tag_helper::get_metadata(path.to_owned()));
            skip_fail!(services::song::create_song(db, metadata).await);
        }
        if f_name.contains("cover.") {
            //println!("Found cover for {:?}", path);
            services::album::update_cover_for_path(
                db,
                path,
                entry.path().parent().unwrap().to_string_lossy().to_string(),
            )
            .await?;
        }
    }

    Ok(())
} /*
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
   */
pub async fn insert_directory(
    path: &String,
    mtime: &DateTime<Utc>,
    db: &DatabaseConnection,
) -> Result<()> {
    let init_time: String = Utc::now().naive_local().to_string();
    let dir: entity::directorie::ActiveModel = entity::directorie::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        path: Set(path.to_owned()),
        mtime: Set(mtime.naive_utc()),
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
    };

    entity::directorie::Entity::insert(dir)
        .on_conflict(
            OnConflict::column(entity::directorie::Column::Path)
                .update_column(entity::directorie::Column::UpdatedAt)
                .update_column(entity::directorie::Column::Mtime)
                .to_owned(),
        )
        .exec(db)
        .await
        .expect("Failed to insert dir");
    Ok(())
}
