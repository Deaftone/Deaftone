use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sqlx::{sqlite::SqliteQueryResult, Sqlite, Transaction};

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
