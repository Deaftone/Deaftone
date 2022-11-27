use std::{
    fs::{self, DirEntry},
    str::FromStr,
    time::{Duration, Instant},
};

use crate::SETTINGS;
use anyhow::Result;
use chrono::{DateTime, Utc};

use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteQueryResult,
        SqliteSynchronous,
    },
    ConnectOptions, Pool,
};

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
                .max_connections(5)
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
            Self::walk_full(&sqlite_pool).await.unwrap();
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

    pub async fn walk_partial(_db: &Pool<sqlx::Sqlite>) -> Result<()> {
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

    /*   pub async fn walk_dir(db: &Pool<sqlx::Sqlite>, dir: String) -> Result<()> {
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
                    Self::insert_directory(&path, &mtime, db).await?;
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
    } */
    pub async fn walk_full(db: &Pool<sqlx::Sqlite>) -> Result<()> {
        tracing::info!("Starting scan");
        let current_dir: &str = SETTINGS.media_path.as_str();
        for entry in WalkDir::new(current_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let start = Instant::now();
            if entry.file_type().is_dir() {
                skip_fail!(Self::scan_dir(&entry, db).await);
            }
            /*             if f_name.ends_with(".flac") {
                let metadata = skip_fail!(tag_helper::get_metadata(path.to_owned()));
                skip_fail!(Self::create_song(db, metadata).await);
            } */
            /*             if f_name.contains("cover.") {
                //println!("Found cover for {:?}", path);
                services::album::update_cover_for_path(
                    db,
                    path,
                    entry.path().parent().unwrap().to_string_lossy().to_string(),
                )
                .await?;
            } */

            let duration = start.elapsed();
            println!("Time elapsed in walk_interation is: {:?}", duration);
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

    async fn scan_dir(entry: &walkdir::DirEntry, sqlite_pool: &Pool<sqlx::Sqlite>) -> Result<()> {
        let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
        let mtime: DateTime<Utc> = fmtime.into();
        let path: String = entry.path().to_string_lossy().to_string();
        Self::insert_directory(&path, &mtime, sqlite_pool).await?;
        /*         if f_name.ends_with(".flac") {
            let metadata = tag_helper::get_metadata(path.to_owned())?;
            Self::create_song(sqlite_pool, metadata).await?;
        } */

        let mut tracks: Vec<AudioMetadata> = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if entry.file_name().to_string_lossy().ends_with(".flac") {
                let metadata = skip_fail!(tag_helper::get_metadata(
                    path.as_path().to_string_lossy().to_string()
                ));
                tracks.push(metadata);
                //skip_fail!(Self::create_song(db, metadata).await);
            }
        }
        for ele in tracks {
            println!("{:}", ele.album)
        }
        Ok(())
    }
    pub async fn insert_directory(
        path: &String,
        mtime: &DateTime<Utc>,
        sqlite_pool: &Pool<sqlx::Sqlite>,
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
        .bind(path.to_owned())
        .bind(mtime.naive_utc())
        .bind(init_time.to_owned())
        .bind(init_time)
        .execute(sqlite_pool)
        .await?)
    }

    pub async fn create_song(
        sqlite_pool: &Pool<sqlx::Sqlite>,
        metadata: AudioMetadata,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        tracing::info!("Inserting {}", metadata.path);
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
                duration
             )
        VALUES (?, ? ,?,?,?,?,?,?,?,?,?)",
        )
        .bind(id.to_string())
        .bind(metadata.path)
        .bind(metadata.name)
        .bind(metadata.number)
        .bind(metadata.album_artist)
        .bind(metadata.album)
        .bind(metadata.track)
        .bind(metadata.year)
        .bind(init_time.to_owned())
        .bind(init_time)
        .bind(metadata.duration)
        .execute(sqlite_pool)
        .await?)
    }
}
