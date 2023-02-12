use chrono::Utc;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

use crate::ApiError;

// Creates a artist entry with artist name passed and MusicBrainzArtistId
pub async fn create_artist(
    tx: &mut Transaction<'_, Sqlite>,
    artist_name: &str,
    mb_artist_id: &Option<String>,
) -> Result<String, anyhow::Error> {
    let id: String = Uuid::new_v4().to_string();
    let init_time: String = Utc::now().naive_local().to_string();
    sqlx::query(
        "INSERT OR REPLACE INTO artists (
            id, 
            name,
            mb_artist_id,
            created_at,
            updated_at
         )
    VALUES (?,?,?,?,?)",
    )
    .bind(&id)
    .bind(artist_name)
    .bind(mb_artist_id)
    .bind(&init_time)
    .bind(&init_time)
    .execute(&mut *tx)
    .await?;
    Ok(id)
}

// Return a artist by there artist_id. Also merges there connected MusicBrainArtistId albums
pub async fn get_artist_by_id(
    db: &DatabaseConnection,
    artist_id: &String,
) -> anyhow::Result<(entity::artist::Model, Vec<entity::album::Model>), ApiError> {
    match entity::artist::Entity::find_by_id(artist_id)
        .order_by_desc(entity::album::Column::Year)
        .find_with_related(entity::album::Entity)
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?
        .first()
    {
        Some(artist) => Ok(artist.to_owned()),
        None => Err(ApiError::RecordNotFound),
    }
}

// Returns a vec of artist with size and sort options
pub async fn get_artists(
    db: &DatabaseConnection,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::artist::Model>, ApiError> {
    let order = match sort.as_deref() {
        Some("name") => entity::artist::Column::Name,
        Some("latest") => entity::artist::Column::CreatedAt,
        _ => entity::artist::Column::Name,
    };
    let result = match order {
        entity::artist::Column::CreatedAt => {
            entity::artist::Entity::find()
                .order_by_desc(order)
                .limit(size)
                .all(db)
                .await?
        }
        _ => {
            entity::artist::Entity::find()
                .order_by_asc(order)
                .limit(size)
                .all(db)
                .await?
        }
    };
    Ok(result)
}

// Returns a vec of artists but paginated according to page and size params default page size 100 will also taking into account sorting options
pub async fn get_artists_paginate(
    db: &DatabaseConnection,
    page: Option<u64>,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::artist::Model>, ApiError> {
    let order = match sort.unwrap_or_default().as_str() {
        "name" => entity::artist::Column::Name,
        "latest" => entity::artist::Column::CreatedAt,
        _ => entity::artist::Column::Name,
    };

    let db_artist = match order {
        entity::artist::Column::CreatedAt => entity::artist::Entity::find()
            .order_by_desc(order)
            .paginate(db, size.unwrap_or(u64::MAX)),
        _ => entity::artist::Entity::find()
            .order_by_asc(order)
            .paginate(db, size.unwrap_or(u64::MAX)),
    };
    Ok(db_artist.fetch_page(page.unwrap_or(0)).await?)
}

// Returns a vec of the latest artists added to the database
pub async fn get_latest_artist(
    db: &DatabaseConnection,
    size: Option<u64>,
) -> anyhow::Result<Vec<entity::artist::Model>> {
    Ok(entity::artist::Entity::find()
        .order_by_desc(entity::artist::Column::CreatedAt)
        .limit(size.unwrap_or(50))
        .all(db)
        .await?)
}

pub async fn _find_by_name(
    db: &DatabaseConnection,
    artist_name: String,
) -> anyhow::Result<Option<entity::artist::Model>> {
    entity::artist::Entity::find()
        .filter(entity::artist::Column::Name.eq(artist_name))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
