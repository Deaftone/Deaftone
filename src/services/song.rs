use anyhow::Ok;
use chrono::Utc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sqlx::{sqlite::SqliteQueryResult, Sqlite, Transaction};

use uuid::Uuid;

use crate::scanner::tag_helper::AudioMetadata;
pub async fn get_song(
    db: &DatabaseConnection,
    id: String,
) -> anyhow::Result<Option<entity::song::Model>> {
    let song: Option<entity::song::Model> = entity::song::Entity::find_by_id(id.to_owned())
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(song)
}

pub async fn like_song(db: &DatabaseConnection, id: String) -> Result<bool, anyhow::Error> {
    /*     Ok(sqlx::query(
        "UPDATE songs
    SET liked = ?
    WHERE id = ?",
    )
    .bind(id)
    .bind(like)
    .await?) */
    let song: Option<entity::song::Model> = entity::song::Entity::find_by_id(id).one(db).await?;
    let mut song: entity::song::ActiveModel = song.unwrap().into();
    match song.liked.unwrap() {
        true => {
            song.liked = Set(false);
        }
        _ => {
            song.liked = Set(true);
        }
    }
    Ok(song.update(db).await?.liked)
}
pub async fn get_song_by_path(
    db: &DatabaseConnection,
    path: String,
) -> anyhow::Result<Option<entity::song::Model>> {
    let song: Option<entity::song::Model> = entity::song::Entity::find()
        .filter(entity::song::Column::Path.eq(path))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(song)
}
pub async fn create_song(
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
            albumId,
            liked
         )
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
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
    .bind(false)
    .execute(&mut *tx)
    .await?)
}
