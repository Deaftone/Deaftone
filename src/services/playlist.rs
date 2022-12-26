use anyhow::Ok;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub async fn _create_playlist(db: &DatabaseConnection) -> anyhow::Result<()> {
    let id: Uuid = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();

    let playlist = entity::playlist::ActiveModel {
        id: Set(id.to_string()),
        name: Set("New music".to_string()),
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
    };

    let p_id: Uuid = Uuid::new_v4();

    let playlist_song = entity::playlist_song::ActiveModel {
        id: Set(p_id.to_string()),
        playlist_id: Set(Some(id.to_string())),
        song_id: Set(Some("b69f58cb-3dac-41ee-b29b-910b12c45b11".to_string())),
    };
    entity::playlist::Entity::insert(playlist).exec(db).await?;
    entity::playlist_song::Entity::insert(playlist_song)
        .exec(db)
        .await?;
    Ok(())
}
