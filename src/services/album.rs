use anyhow::Ok;
use chrono::Utc;
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

pub async fn get_album_by_id(
    db: &DatabaseConnection,
    album_id: String,
) -> anyhow::Result<Vec<(entity::album::Model, Vec<entity::songs::Model>)>> {
    Ok(entity::album::Entity::find_by_id(album_id)
        .find_with_related(entity::songs::Entity)
        .all(db)
        .await?)
}
pub async fn find_by_name(
    db: &DatabaseConnection,
    album_name: String,
) -> anyhow::Result<Option<entity::album::Model>> {
    entity::album::Entity::find()
        .filter(entity::album::Column::Name.eq(album_name))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn update_cover_for_path(
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

pub async fn get_all_albums(db: &DatabaseConnection) -> anyhow::Result<Vec<entity::album::Model>> {
    let albums: Vec<entity::album::Model> = entity::album::Entity::find()
        .all(db)
        .await
        .expect("Failed to get albums");
    Ok(albums)
}
pub async fn get_albums_paginate(
    db: &DatabaseConnection,
    page: usize,
    size: usize,
) -> anyhow::Result<Vec<entity::album::Model>> {
    let db_albums = entity::album::Entity::find().paginate(db, size);
    let albums = db_albums.fetch_page(page).await?;
    //albums.Ok(albums)
    Ok(albums)
}
pub async fn create_album(
    db: &DatabaseConnection,
    album_name: String,
    artist_name: String,
    path: String,
    year: Option<i32>,
) -> anyhow::Result<Uuid> {
    let db_album = entity::artists::Entity::find()
        .filter(entity::artists::Column::Name.eq(artist_name.to_owned()))
        .one(db)
        .await?;

    let id: Uuid = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();

    let mut album = entity::album::ActiveModel {
        id: Set(id.to_string()),
        name: Set(album_name.to_owned()),
        artist_name: Set(artist_name.to_owned()),
        year: Set(year.unwrap_or_default()),
        path: Set(path),
        cover: NotSet,
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
        artist_id: NotSet,
    };

    if db_album.is_some() {
        album.set(
            entity::album::Column::ArtistId,
            Set(db_album.unwrap().id).into_value().unwrap(),
        )
    } else {
        let artist_id: Uuid = super::artist::create_artist(db, artist_name)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        album.set(
            entity::album::Column::ArtistId,
            Set(artist_id.to_string()).into_value().unwrap(),
        )
    }
    album.insert(db).await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(id)
}
