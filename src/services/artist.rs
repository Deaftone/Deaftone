use anyhow::anyhow;
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Serialize;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, Transaction};
use super::{DbArtist, DeaftoneSelect};
pub async fn create_artist(
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
#[derive(Serialize)]
pub struct DbArtist {
    id: String,
    name: String,
    image: String,
    bio: String,
    albums: Vec<entity::album::Model>,
}
pub async fn get_artist_by_id(
    db: &DatabaseConnection,
    artist_id: String,
) -> anyhow::Result<DbArtist> {
    let artist_db = entity::artist::Entity::find_by_id(artist_id)
        .order_by_desc(entity::album::Column::Year)
        .find_with_related(entity::album::Entity)
        .all(db)
        .await?;
    let artist = artist_db.first().expect("No artist found");
    let artist_model = artist.0.to_owned();
    let albums = artist.1.to_owned();
    Ok(DbArtist {
        id: artist_model.id,
        name: artist_model.name,
        image: artist_model.image.unwrap_or_default(),
        bio: artist_model.bio.unwrap_or_default(),
        albums,
    })
}

pub async fn get_artists(
    db: &DatabaseConnection,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::artist::Model>> {
    let order = match sort.as_deref() {
        Some("name") => entity::artist::Column::Name,
        Some("latest") => entity::artist::Column::CreatedAt,
        _ => entity::artist::Column::Name,
    };
    let result = match order {
        entity::artist::Column::CreatedAt => {
            entity::artist::Entity::find()
                .order_by_desc(order)
                .limit_option(size)
                .all(db)
                .await
        }
        _ => {
            entity::artist::Entity::find()
                .order_by_asc(order)
                .limit_option(size)
                .all(db)
                .await
        }
    }
    .map_err(|e| anyhow!("Failed to get artists: {}", e))?;

    Ok(result)
}

pub async fn get_artists_paginate(
    db: &DatabaseConnection,
    page: Option<u64>,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::artist::Model>> {
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
