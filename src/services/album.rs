use crate::{handlers::ApiError, services::scanner::tag_helper::AudioMetadata};

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use sea_orm::{PaginatorTrait, QuerySelect};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

// Creates a album entry with belonging to provided artist_id
pub async fn create_album(
    tx: &mut Transaction<'_, Sqlite>,
    cover: Option<String>,
    artist_id: &String,
    metadata: &AudioMetadata,
) -> Result<String, anyhow::Error> {
    let id: String = Uuid::new_v4().to_string();
    let init_time: String = Utc::now().naive_local().to_string();
    sqlx::query(
        "INSERT OR REPLACE INTO albums (
            id,
            path,
            name,
            album_artist,
            album_artist_sort,
            album_artist_credit,
            discogs_albumid,
            discogs_artistid,
            discogs_labelid,
            year,
            composer,
            mb_album_id,
            mb_artist_id,
            album_type,
            label,
            mb_releasegroup_id,
            asin,
            catalog_num,
            script,
            country,
            album_disambig,
            release_group_disambig,
            artist_name,
            cover,
            created_at,
            updated_at,
            artist_id
         )
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(&id)
    .bind(&metadata.path)
    .bind(&metadata.album_name)
    .bind(&metadata.album_artist)
    .bind(&metadata.album_sort) // ALBUM_ARTIST_SORT
    .bind(&metadata.artist) // ARTIST CREDIT
    /*     .bind(&metadata.genre) // ARTIST CREDIT
    .bind(&metadata.style) // ARTIST CREDIT */
    .bind(&metadata.discogs_albumid)
    .bind(&metadata.discogs_artistid)
    .bind(&metadata.discogs_labelid)
    .bind(metadata.year)
    .bind(&metadata.composer)
    .bind(&metadata.mb_album_id)
    .bind(&metadata.mb_artist_id)
    .bind(&metadata.album_type)
    .bind(&metadata.label)
    .bind(&metadata.mb_releasegroup_id)
    .bind(&metadata.asin)
    .bind(&metadata.catalog_num)
    .bind(&metadata.script)
    /*     .bind(&metadata.language)
     */
    .bind(&metadata.country)
    /*     .bind(&metadata.album_status)
     */
    .bind(&metadata.album_disambig)
    .bind(&metadata.release_group_disambig)
    .bind(&metadata.artist)
    .bind(cover)
    .bind(&init_time)
    .bind(&init_time)
    .bind(artist_id)
    .execute(&mut *tx)
    .await?;
    Ok(id)
}

// Returns a album by the album_id with songs
pub async fn get_album_by_id(
    db: &DatabaseConnection,
    album_id: String,
) -> Result<(entity::album::Model, Vec<entity::song::Model>), ApiError> {
    match entity::album::Entity::find_by_id(album_id.clone())
        .find_with_related(entity::song::Entity)
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?
        .first()
    {
        Some(album) => Ok(album.to_owned()),
        None => Err(ApiError::RecordNotFound(format!(
            "Album \"{album_id}\" not found"
        ))),
    }
}

// Returns a album by the album_id
pub async fn get_album_by_id_single(
    db: &DatabaseConnection,
    album_id: String,
) -> anyhow::Result<entity::album::Model> {
    Ok(entity::album::Entity::find_by_id(album_id)
        .one(db)
        .await?
        .expect("Failed to get album"))
}
pub async fn _find_by_name(
    db: &DatabaseConnection,
    album_name: String,
) -> anyhow::Result<Option<entity::album::Model>> {
    entity::album::Entity::find()
        .filter(entity::album::Column::Name.eq(album_name))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn _update_cover_for_path(
    db: &DatabaseConnection,
    cover_path: String,
    album_path: String,
) -> anyhow::Result<()> {
    let db_album = entity::album::Entity::find()
        .filter(entity::album::Column::Path.eq(album_path))
        .one(db)
        .await?;
    if db_album.is_some() {
        let mut album: entity::album::ActiveModel = db_album.unwrap().into();
        album.cover = Set(Some(cover_path));
        album.update(db).await?;
    }
    Ok(())
}

// Returns vec of albums taking into account sorting options and size
// Sort options: name artist_name year latest
pub async fn get_albums(
    db: &DatabaseConnection,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::album::Model>, ApiError> {
    let order = match sort.as_deref() {
        Some("name") => entity::album::Column::Name,
        Some("artist_name") => entity::album::Column::ArtistName,
        Some("year") => entity::album::Column::Year,
        Some("latest") => entity::album::Column::CreatedAt,
        _ => entity::album::Column::Name,
    };

    let limit = size.unwrap_or(500);
    Ok(match order {
        entity::album::Column::CreatedAt => {
            entity::album::Entity::find()
                .order_by_desc(order)
                .limit(limit)
                .all(db)
                .await?
        }
        _ => {
            entity::album::Entity::find()
                .order_by_asc(order)
                .limit(limit)
                .all(db)
                .await?
        }
    })
}

// Returns a vec of albums but paginated according to page and size params default page size 100 will also taking into account sorting options
pub async fn get_albums_paginate(
    db: &DatabaseConnection,
    page: Option<u64>,
    size: Option<u64>,
    sort: Option<String>,
) -> anyhow::Result<Vec<entity::album::Model>, ApiError> {
    let order = match sort.unwrap_or_default().as_str() {
        "name" => entity::album::Column::Name,
        "artist_name" => entity::album::Column::ArtistName,
        "year" => entity::album::Column::Year,
        "latest" => entity::album::Column::CreatedAt,
        _ => entity::album::Column::Name,
    };

    let db_albums = match order {
        entity::album::Column::CreatedAt => entity::album::Entity::find()
            .order_by_desc(order)
            .paginate(db, size.unwrap_or(100)),
        _ => entity::album::Entity::find()
            .order_by_asc(order)
            .paginate(db, size.unwrap_or(100)),
    };
    Ok(db_albums.fetch_page(page.unwrap_or(0)).await?)
}
