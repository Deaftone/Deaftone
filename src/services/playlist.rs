use chrono::{NaiveDateTime, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::ApiError;

pub async fn _create_playlist(db: &DatabaseConnection) -> anyhow::Result<()> {
    let id: Uuid = Uuid::new_v4();
    let init_time: NaiveDateTime = Utc::now().naive_local();

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
pub async fn get_playlist_by_id_single(
    db: &DatabaseConnection,
    playlist_id: String,
) -> Result<entity::playlist::Model, ApiError> {
    match entity::playlist::Entity::find_by_id(&playlist_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })? {
        Some(playlist) => Ok(playlist),
        None => Err(ApiError::RecordNotFound),
    }
}
