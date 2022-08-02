use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;
pub async fn create_artist(db: &DatabaseConnection, artist_name: String) -> anyhow::Result<Uuid> {
    let id = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();

    let artist = entity::artists::ActiveModel {
        id: Set(id.to_string()),
        name: Set(artist_name.to_owned()),
        image: NotSet,
        bio: NotSet,
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
    };

    artist.insert(db).await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(id)
}

pub async fn find_by_name(
    db: &DatabaseConnection,
    artist_name: String,
) -> anyhow::Result<Option<entity::artists::Model>> {
    entity::artists::Entity::find()
        .filter(entity::artists::Column::Name.eq(artist_name))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
