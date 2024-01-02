use anyhow::anyhow;
use chrono::{NaiveDateTime, Utc};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use super::http::error::ApiError;

pub async fn create_playlist(
    db: &DatabaseConnection,
    playlist_name: &str,
) -> Result<String, ApiError> {
    let id: String = Uuid::new_v4().to_string();
    let init_time: NaiveDateTime = Utc::now().naive_local();
    let playlist = entity::playlist::ActiveModel {
        id: Set(id.clone()),
        name: Set(playlist_name.to_string()),
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
    };
    entity::playlist::Entity::insert(playlist).exec(db).await?;
    Ok(id)
}
pub async fn get_playlist_by_id_slim(
    db: &DatabaseConnection,
    playlist_id: &str,
) -> Result<entity::playlist::Model, ApiError> {
    match entity::playlist::Entity::find_by_id(playlist_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })? {
        Some(playlist) => Ok(playlist),
        None => Err(ApiError(
            StatusCode::NOT_FOUND,
            anyhow!("Unable to find Playlist with id: {}", playlist_id),
        )),
    }
}
pub async fn get_playlist_by_id(
    db: &DatabaseConnection,
    playlist_id: &str,
) -> Result<(entity::playlist::Model, Vec<entity::song::Model>), ApiError> {
    match entity::playlist::Entity::find_by_id(playlist_id)
        .find_with_related(entity::song::Entity)
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?
        .first()
    {
        Some(playlist) => Ok(playlist.clone()),
        None => Err(ApiError(
            StatusCode::NOT_FOUND,
            anyhow!("Unable to find Playlist with id: {}", playlist_id),
        )),
    }
    /*     match entity::playlist::Entity::find_by_id(playlist_id)
        .find_also_related(entity::song::Entity)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })? {
        Some(playlist) => Ok(playlist),
        None => Err(ApiError::RecordNotFound),
    } */
}

pub async fn add_song_to_playlist(
    db: &DatabaseConnection,
    playlist_id: &str,
    song_id: &str,
) -> Result<String, ApiError> {
    match self::get_playlist_by_id(db, playlist_id).await {
        Ok(_) => match crate::services::song::get_song_by_id(db, song_id).await {
            Ok(_) => {
                let id: String = Uuid::new_v4().to_string();
                let playlist_song = entity::playlist_song::ActiveModel {
                    id: Set(id),
                    playlist_id: Set(Some(playlist_id.to_string())),
                    song_id: Set(Some(song_id.to_string())),
                };
                let insert = entity::playlist_song::Entity::insert(playlist_song)
                    .exec(db)
                    .await?
                    .last_insert_id;
                Ok(insert)
            }

            Err(_) => Err(ApiError(
                StatusCode::NOT_FOUND,
                anyhow!("Unable to find Playlist with id: {}", playlist_id),
            )),
        },
        Err(_) => Err(ApiError(
            StatusCode::NOT_FOUND,
            anyhow!("Unable to find Playlist with id: {}", playlist_id),
        )),
    }
}
pub async fn get_playlists(
    db: &DatabaseConnection,
    playlist_id: &str,
) -> Result<Vec<entity::playlist::Model>, ApiError> {
    let playlists = entity::playlist::Entity::find_by_id(playlist_id)
        .all(db)
        .await?;
    Ok(playlists)
}
pub async fn delete_playlist(
    db: &DatabaseConnection,
    playlist_id: &str,
) -> anyhow::Result<(), ApiError> {
    entity::playlist::Entity::delete_by_id(playlist_id)
        .exec(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
