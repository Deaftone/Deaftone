use std::{
    fs::{self},
    path::PathBuf,
    pin::Pin,
    str::FromStr,
    time::{Duration, Instant},
};

use crate::SETTINGS;
use anyhow::{Error, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use futures::Stream;
use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteQueryResult,
        SqliteSynchronous,
    },
    ConnectOptions, Execute, Pool, Row, Sqlite, SqlitePool, Transaction,
};
use tokio_stream::StreamExt;

use std::result::Result::Ok;
use std::time::SystemTime;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::SCAN_STATUS;

use self::tag_helper::AudioMetadata;
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

#[derive(Clone)]
pub struct Scanner {}
impl Scanner {
    pub fn new() -> Result<Scanner> {
        let scanner: Scanner = Scanner {};
        Ok(scanner)
    }

    pub fn start_scan(&mut self) {
        let start = Instant::now();

        tokio::spawn(async move {
            SCAN_STATUS
                .lock()
                .unwrap()
                .store(true, std::sync::atomic::Ordering::Relaxed);
            let database_file = "deaftone.sqlite";
            let database_url = format!("sqlite://{}", database_file);
            let pool_timeout = Duration::from_secs(30);
            let connection_options = SqliteConnectOptions::from_str(&database_url)
                .unwrap()
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(pool_timeout)
                .disable_statement_logging()
                .clone();

            let sqlite_pool = SqlitePoolOptions::new()
                .max_connections(10)
                .connect_with(connection_options)
                .await
                .unwrap();
            let before: Instant = Instant::now();

            /*             let has_scanned_full =
                sqlx::query!("SELECT value FROM settings WHERE name = 'scanned'")
                    .fetch_one(&sqlite_pool)
                    .await;
            match has_scanned_full {
                Err(sqlx::Error::RowNotFound) => Self::walk_full(&sqlite_pool).await.unwrap(),
                value => match value.unwrap().value == "1" {
                    true => {
                        tracing::info!("Starting partial scan");
                        Self::walk_partial(&sqlite_pool).await.unwrap();
                    }
                    _ => Self::walk_full(&sqlite_pool).await.unwrap(),
                },
            } */

            sqlx::query("pragma temp_store = memory;")
                .execute(&sqlite_pool)
                .await
                .unwrap();
            sqlx::query("pragma mmap_size = 30000000000;")
                .execute(&sqlite_pool)
                .await
                .unwrap();
            sqlx::query("pragma page_size = 4096;")
                .execute(&sqlite_pool)
                .await
                .unwrap();

            //Self::walk_full(&sqlite_pool).await.unwrap();
            Self::walk_partial(&sqlite_pool).await.unwrap();

            /*             sqlx::query("pragma temp_store = memory;")
                           .execute(&sqlite_pool)
                           .await
                           .unwrap();
                       sqlx::query("pragma mmap_size = 30000000000;")
                           .execute(&sqlite_pool)
                           .await
                           .unwrap();
                       sqlx::query("pragma synchronous = normal;")
                           .execute(&sqlite_pool)
                           .await
                           .unwrap();
                       sqlx::query("pragma page_size = 4096;")
                           .execute(&sqlite_pool)
                           .await
                           .unwrap();
                       sqlx::query("PRAGMA foreign_keys = ON;")
                           .execute(&sqlite_pool)
                           .await
                           .unwrap();
            */

            tracing::info!("Scan completed in: {:.2?}", before.elapsed());

            // Cleanup orphans
            /*             db.execute(Statement::from_string(
                db.get_database_backend(),
                "delete from albums where id not in (
                select albumId from songs
              )"
                .to_owned(),
            ))
            .await
            .unwrap();
            db.execute(Statement::from_string(
                db.get_database_backend(),
                "delete from artists where id not in (
                select artistId from albums
              )"
                .to_owned(),
            ))
            .await
            .unwrap(); */
            let duration = start.elapsed();
            println!("Time elapsed in scan() is: {:?}", duration);
            SCAN_STATUS
                .lock()
                .unwrap()
                .store(false, std::sync::atomic::Ordering::Relaxed);
        });
    }

    pub async fn walk_partial(pool: &Pool<sqlx::Sqlite>) -> Result<()> {
        let mut connection = pool.acquire().await?;

        let mut rows = sqlx::query("SELECT * FROM directories").fetch(&mut connection);

        while let Some(row) = rows.try_next().await? {
            let path: String = row.get("path");
            let ddirectory_mtime: NaiveDateTime = row.get("mtime");
            let meta = fs::metadata(&path);
            let is_empty = PathBuf::from(&path)
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false);

            if meta.is_ok() && !is_empty {
                let _ftime: SystemTime = meta.unwrap().modified().unwrap();
                let ftime: DateTime<Utc> = _ftime.into();

                if ftime.naive_utc() > ddirectory_mtime {
                    tracing::info!("Dir changed {:}", &path);
                    //Self::walk_dir(db, item.path).await?;
                } else {
                    tracing::info!("Dir hasn't {}", &path);
                }
            } else {
                tracing::info!("Dropping all items for path {}", &path);
                let query = sqlx::query("DELETE FROM directories WHERE path LIKE ?")
                    .bind(&path)
                    .persistent(true)
                    .execute(pool)
                    .await;

                println!("{:?}", query.unwrap());
                let query = sqlx::query("DELETE FROM songs WHERE path LIKE ?")
                    .bind(&path)
                    .persistent(true)
                    .execute(pool)
                    .await;

                println!("{:?}", query.unwrap());
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

    pub async fn walk_full(db: &Pool<sqlx::Sqlite>) -> Result<()> {
        tracing::info!("Starting scan");
        let current_dir: &str = SETTINGS.media_path.as_str();
        for entry in WalkDir::new(current_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
                let mtime: DateTime<Utc> = fmtime.into();
                let path: String = entry.path().to_string_lossy().to_string();

                let directory_exists = sqlx::query("SELECT * FROM directories WHERE path = ?")
                    .bind(&path)
                    .persistent(true)
                    .fetch_one(db)
                    .await;
                match directory_exists {
                    Err(sqlx::Error::RowNotFound) => {
                        tracing::info!("Creating directory");
                        Self::insert_directory(&path, &mtime, &db).await?;
                        tracing::info!("Creating directory {:}", &path);
                        skip_fail!(Self::scan_dir(&entry, &path, db).await);
                    }
                    value => {
                        let directory_mtime: DateTime<Utc> = value.unwrap().get("mtime");
                        if directory_mtime < mtime {
                            tracing::info!(
                                "Found modified directory {:} dtime: {:} ftime: {:}",
                                &path,
                                directory_mtime,
                                mtime
                            );
                            skip_fail!(Self::scan_dir(&entry, &path, db).await);
                        } else {
                            tracing::info!(
                                "Skipping directory {:} dtime: {:} ftime: {:}",
                                &path,
                                directory_mtime,
                                mtime
                            );
                        }
                    }
                }
            }
        }

        sqlx::query(
            "INSERT OR REPLACE INTO settings (
                    name,
                    value
                )
                VALUES (?,?)",
        )
        .bind("scanned".to_string())
        .bind(true)
        .execute(db)
        .await?;
        Ok(())
    }

    async fn scan_dir(
        entry: &walkdir::DirEntry,
        path: &String,
        sqlite_pool: &Pool<sqlx::Sqlite>,
    ) -> Result<()> {
        let mut tx = sqlite_pool.begin().await.unwrap();
        tracing::info!("Scanning dir {:}", &path);

        let mut create_album = true;
        let mut create_artist = true;
        let mut album_id: String = "".to_string();
        let mut artist_id: String = "".to_string();

        for entry in fs::read_dir(path)? {
            // Is assigning here bad? Since in a large collection it could be alot of allocations
            let path = entry?.path();
            let path_string = path.as_path().to_string_lossy().to_string();
            let path_parent = path.parent().unwrap().to_string_lossy().to_string();

            if path.extension() == Some(std::ffi::OsStr::new("flac")) {
                let metadata = skip_fail!(tag_helper::get_metadata(path_string.clone()));
                // Check if album has been created. This is a nice speedup since we can assume that when we are in a folder of tracks the they are all from the same album
                if create_artist {
                    let artists_exists = sqlx::query("SELECT * FROM artists WHERE name = ?")
                        .bind(&metadata.album_artist)
                        .persistent(true)
                        .fetch_one(sqlite_pool)
                        .await;
                    match artists_exists {
                        Err(sqlx::Error::RowNotFound) => {
                            let id: String = Uuid::new_v4().to_string();
                            skip_fail!(
                                Self::create_artist(&mut tx, &id, &metadata.album_artist,).await
                            );
                            // Set create artist to false since we know its created now
                            create_artist = false;
                            // Set artist_id here since on the first run of a scan it wont be found since we have the create_album inside the transaction
                            artist_id = id;
                            tracing::info!("Creating artists \"{:}\"", metadata.album_artist)
                        }
                        value => {
                            artist_id = value.unwrap().get("id");
                        }
                    }
                }
                if create_album {
                    let album_exists =
                        sqlx::query("SELECT * FROM albums WHERE name = ? AND path = ?")
                            .bind(&metadata.album)
                            .bind(&path_parent)
                            .persistent(true)
                            .fetch_one(sqlite_pool)
                            .await;
                    match album_exists {
                        Err(sqlx::Error::RowNotFound) => {
                            let id: String = Uuid::new_v4().to_string();
                            // Searching for cover here allows us to not have to check every iteration of the album to find the cover. Rather we search the dir once. Which should already be cached by the system
                            let mut cover: Option<String> = None;
                            for entry in fs::read_dir(&path_parent)? {
                                let f_name = entry?.path().to_string_lossy().to_string().clone();
                                if f_name.contains("cover.") {
                                    cover = Some(f_name);
                                }
                            }
                            skip_fail!(
                                Self::create_album(
                                    &mut tx,
                                    &id,
                                    cover,
                                    &artist_id,
                                    &metadata.album,
                                    &metadata.album_artist,
                                    &path_parent,
                                    &metadata.year
                                )
                                .await
                            );

                            // Set create album to false since we know its created now
                            create_album = false;
                            // Set album_id here since on the first run of a scan it wont be found since we have the create_album inside the transaction
                            album_id = id;
                            tracing::info!("Creating album \"{:}\"", metadata.album)
                        }
                        value => {
                            album_id = value.unwrap().get("id");
                        }
                    }
                }
                skip_fail!(Self::create_song(&mut tx, &album_id, &metadata).await);
            }
        }
        tx.commit().await.unwrap();
        Ok(())
    }
    async fn insert_directory(
        path: &String,
        mtime: &DateTime<Utc>,
        tx: &Pool<sqlx::Sqlite>,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        let init_time: String = Utc::now().naive_local().to_string();
        Ok(sqlx::query(
            "INSERT OR REPLACE INTO directories (
                    id,
                    path,
                    mtime,
                    createdAt,
                    updatedAt
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

    async fn create_song(
        tx: &mut Transaction<'_, Sqlite>,
        album_id: &String,
        metadata: &AudioMetadata,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        let id: Uuid = Uuid::new_v4();
        let init_time: String = Utc::now().naive_local().to_string();
        Ok(sqlx::query(
            "INSERT OR REPLACE INTO songs (
                id, 
                path,
                title,
                disk,
                artist,
                albumName,
                track,
                year,
                createdAt,
                updatedAt,
                duration,
                albumId
             )
        VALUES (?, ? ,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(id.to_string())
        .bind(&metadata.path)
        .bind(&metadata.name)
        .bind(metadata.number)
        .bind(&metadata.album_artist)
        .bind(&metadata.album)
        .bind(metadata.track)
        .bind(metadata.year)
        .bind(&init_time)
        .bind(&init_time)
        .bind(metadata.duration)
        .bind(album_id)
        .execute(&mut *tx)
        .await?)
    }
    async fn create_album(
        tx: &mut Transaction<'_, Sqlite>,
        id: &String,
        cover: Option<String>,
        artist_id: &String,
        album_name: &String,
        artist_name: &String,
        path: &String,
        year: &i32,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        let init_time: String = Utc::now().naive_local().to_string();
        Ok(sqlx::query(
            "INSERT OR REPLACE INTO albums (
                id, 
                name,
                artistName,
                cover,
                path,
                year,
                createdAt,
                updatedAt,
                artistId
             )
        VALUES (?,?,?,?,?,?,?,?,?)",
        )
        .bind(id)
        .bind(album_name)
        .bind(artist_name)
        .bind(cover.unwrap_or_default())
        .bind(path)
        .bind(year)
        .bind(&init_time)
        .bind(&init_time)
        .bind(artist_id)
        .execute(&mut *tx)
        .await?)
    }
    async fn create_artist(
        tx: &mut Transaction<'_, Sqlite>,
        id: &String,
        artist_name: &String,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        let init_time: String = Utc::now().naive_local().to_string();
        Ok(sqlx::query(
            "INSERT OR REPLACE INTO artists (
                id, 
                name,
                createdAt,
                updatedAt
             )
        VALUES (?,?,?,?)",
        )
        .bind(id)
        .bind(artist_name)
        .bind(&init_time)
        .bind(&init_time)
        .execute(&mut *tx)
        .await?)
    }
}
