use crate::{services, SCAN_STATUS, SETTINGS};
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Pool, Row};
use std::result::Result::Ok;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::{
    fs::{self},
    path::PathBuf,
    time::Instant,
};
use tokio_stream::StreamExt;
use uuid::Uuid;
use walkdir::WalkDir;

use super::album::AlbumService;

pub mod tag_helper;

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
#[derive(Debug)]
pub enum ScanType {
    FullScan,
    PartialScan,
}

#[derive(Clone)]
pub struct ScanService {
    db: Pool<sqlx::Sqlite>,
    album_service: AlbumService,
}
impl ScanService {
    pub fn new(db: Pool<sqlx::Sqlite>, album_service: AlbumService) -> ScanService {
        ScanService { db, album_service }
    }

    pub async fn start_scan(&self, scan_type: ScanType) {
        // Set global SCAN_STATUS to true
        SCAN_STATUS.store(true, Ordering::Release);
        sqlx::query("pragma temp_store = memory;")
            .execute(&self.db)
            .await
            .unwrap();
        sqlx::query("pragma mmap_size = 30000000000;")
            .execute(&self.db)
            .await
            .unwrap();
        sqlx::query("pragma page_size = 4096;")
            .execute(&self.db)
            .await
            .unwrap();
        let before: Instant = Instant::now();
        let current_dir = SETTINGS.media_path.clone();

        match scan_type {
            ScanType::FullScan => {
                tracing::info!("Starting full scan");
                self.walk_full_initial(current_dir).await.unwrap();
            }
            ScanType::PartialScan => {
                tracing::info!("Starting partial scan");
                self.walk_partial().await.unwrap();
            }
        }
        tracing::info!("Scan completed in: {:.2?}", before.elapsed());

        // Set global SCAN_STATUS to false
        SCAN_STATUS.store(false, Ordering::Release);
    }

    pub async fn walk_partial(&self) -> Result<()> {
        let mut rows = sqlx::query("SELECT * FROM directories").fetch(&self.db);

        while let Some(row) = rows.try_next().await? {
            let path: String = row.get("path");
            let ddirectory_mtime: NaiveDateTime = row.get("mtime");
            let meta = fs::metadata(&path);
            let is_empty = PathBuf::from(&path)
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false);

            if meta.is_ok() && !is_empty {
                if let Ok(meta) = meta {
                    let _ftime: SystemTime = meta.modified().unwrap();
                    let ftime: DateTime<Utc> = _ftime.into();

                    if ftime.naive_utc() > ddirectory_mtime {
                        tracing::debug!("Dir changed {:}", &path);
                        //Self::walk_dir(db, item.path).await?;
                    } else {
                        tracing::debug!("Dir hasn't {}", &path);
                    }
                } else {
                    tracing::debug!("Failed to read metadata");
                }
            } else {
                tracing::info!("Dropping all items for path {}", &path);
                sqlx::query("DELETE FROM directories WHERE path LIKE ?")
                    .bind(&path)
                    .persistent(true)
                    .execute(&self.db)
                    .await?;
                sqlx::query("DELETE FROM songs WHERE path LIKE ?")
                    .bind(&path)
                    .persistent(true)
                    .execute(&self.db)
                    .await?;
            }
        }
        /*  let mut dirs_stream = entity::directorie::Entity::find().stream(db).await?;
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
                    Self::walk_dir(db, item.path).await?;
                } else {
                    tracing::debug!("Dir hasn't {}", item.path);
                }
            } else {
                tracing::info!("Dropping all items for path {}", item.path);
                // Drop all songs for missing path
                /*             entity::song::Entity::delete_many()
                    .filter(entity::song::Column::Path.contains(&item.path))
                    .exec(db)
                    .await?;
                entity::directorie::Entity::delete_many()
                    .filter(entity::directorie::Column::Path.contains(&item.path))
                    .exec(db)
                    .await?; */
            }
        } */
        Ok(())
    }
    // This is only run on the first initital scan of Deaftone. Since we dont need to checking if the directory exists or has been modified in the database
    pub async fn walk_full_initial(&self, current_dir: String) -> Result<()> {
        for entry in WalkDir::new(current_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|f| {
                !f.path()
                    .iter()
                    .any(|s| s.to_str().map(|x| x.starts_with('.')).unwrap_or(false))
            })
        {
            if entry.file_type().is_dir() {
                let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
                let mtime: DateTime<Utc> = fmtime.into();
                let path: String = entry.path().to_string_lossy().to_string();
                Self::insert_directory(&path, &mtime, &self.db).await?;
                tracing::debug!("Created directory {:}", &path);
                skip_fail!(self.scan_dir(&path).await);
            }
        }
        self.update_scan_state(true).await?;
        Ok(())
    }

    async fn _scan_dir_partial(
        _entry: &walkdir::DirEntry,
        _path: &str,
        _sqlite_pool: &Pool<sqlx::Sqlite>,
    ) -> Result<()> {
        Ok(())
    }

    // Scan dir function for a full directory scan missing check for seen songs
    async fn scan_dir(&self, path: &str) -> Result<()> {
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| anyhow!("Error beginning transaction: {}", e))?;
        tracing::debug!("Scanning dir {:}", path);
        let mut create_album = true;
        let mut create_artist = true;
        let mut album_id = String::new();
        let mut artist_id = String::new();

        for entry in fs::read_dir(path)? {
            // Is assigning here bad? Since in a large collection it could be alot of allocations
            let path = entry?.path();
            let path_parent = path.parent().unwrap().to_string_lossy().to_string();

            if path.extension() == Some(std::ffi::OsStr::new("flac")) {
                let metadata = skip_fail!(tag_helper::get_metadata_flac(path));
                // Check if album has been created. This is a nice speedup since we can assume that when we are in a folder of tracks the they are all from the same album
                if create_artist {
                    let artists_exists = sqlx::query("SELECT * FROM artists WHERE name = ?")
                        .bind(&metadata.album_artist)
                        .persistent(true)
                        .fetch_one(&self.db)
                        .await;
                    // Check if artist exists on this loop
                    match artists_exists {
                        Err(sqlx::Error::RowNotFound) => {
                            artist_id = skip_fail!(
                                services::artist::create_artist(
                                    &mut tx,
                                    &metadata.album_artist,
                                    &metadata.mb_artist_id
                                )
                                .await
                            );
                            // Set create artist to false since we know its created now. This can later be used to to skip a db query
                            create_artist = false;
                            // Set artist_id here since on the first run of a scan it wont be found since we have the create_album inside the transaction
                            tracing::info!("Creating artists \"{:}\"", metadata.album_artist)
                        }
                        value => {
                            artist_id = value.unwrap().get("id");
                        }
                    }
                }
                // Check if album has been created before inside this folder
                if create_album {
                    let album_exists = sqlx::query("SELECT * FROM albums WHERE name = ?")
                        .bind(&metadata.album_name)
                        .persistent(true)
                        .fetch_one(&self.db)
                        .await;
                    match album_exists {
                        Err(sqlx::Error::RowNotFound) => {
                            // Searching for cover here allows us to not have to check every iteration of the album to find the cover. Rather we search the dir once. Which should already be cached by the system
                            let mut cover: Option<String> = None;
                            for entry in fs::read_dir(&path_parent)? {
                                let f_name = entry?.path().to_string_lossy().to_string().clone();
                                if f_name.contains("cover.") {
                                    cover = Some(f_name);
                                }
                            }
                            let id = skip_fail!(
                                self.album_service
                                    .create_album(&mut tx, cover, &artist_id, &metadata)
                                    .await
                            );

                            // Set create album to false since we know its created now
                            create_album = false;
                            // Set album_id here since on the first run of a scan it wont be found since we have the create_album inside the transaction
                            album_id = id;
                            tracing::info!("Creating album \"{:}\"", metadata.album_name)
                        }
                        value => {
                            album_id = value.unwrap().get("id");
                        }
                    }
                }
                tracing::info!("Creating song \"{:}\"", metadata.name);
                // Create song. Skip loop iteration of failed
                skip_fail!(services::song::create_song(&mut tx, &album_id, &metadata).await);
            }
        }
        tx.commit().await.unwrap();
        Ok(())
    }

    // Inserts a directory into the database with provided path and mtime
    async fn insert_directory(
        path: &str,
        mtime: &DateTime<Utc>,
        tx: &Pool<sqlx::Sqlite>,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        let init_time: String = Utc::now().naive_local().to_string();
        Ok(sqlx::query(
            "INSERT OR REPLACE INTO directories (
                        id,
                        path,
                        mtime,
                        created_at,
                        updated_at
                    )
                    VALUES (?,?,?,?,?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(path)
        .bind(mtime.naive_utc())
        .bind(&init_time)
        .bind(&init_time)
        .execute(tx)
        .await?)
    }

    async fn update_scan_state(&self, value: bool) -> Result<SqliteQueryResult, anyhow::Error> {
        Ok(sqlx::query(
            "INSERT OR REPLACE INTO settings (
                        name,
                        value
                    )
                    VALUES (?,?)",
        )
        .bind("scanned".to_string())
        .bind(value)
        .execute(&self.db)
        .await?)
    }
}
