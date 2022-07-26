use anyhow::Result;
use entity::songs;
use entity::songs::Entity as Song;
use sea_orm::{DatabaseConnection, EntityTrait};
pub async fn get_song(db: &DatabaseConnection, id: String) -> Result<Option<entity::songs::Model>> {
    let song: Option<songs::Model> = Song::find_by_id(id.to_owned())
        .one(db)
        .await
        .expect("Failed to get song from db");
    Ok(song)
}
