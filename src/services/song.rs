use anyhow::Ok;
use chrono::Utc;
use migration::OnConflict;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use std::path::Path;
use uuid::Uuid;

use crate::scanner::tag_helper::AudioMetadata;
pub async fn get_song(
    db: &DatabaseConnection,
    id: String,
) -> anyhow::Result<Option<entity::songs::Model>> {
    let song: Option<entity::songs::Model> = entity::songs::Entity::find_by_id(id.to_owned())
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(song)
}
pub async fn get_song_by_path(
    db: &DatabaseConnection,
    path: String,
) -> anyhow::Result<Option<entity::songs::Model>> {
    let song: Option<entity::songs::Model> = entity::songs::Entity::find()
        .filter(entity::songs::Column::Path.eq(path))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(song)
}
pub async fn create_or_update(
    db: &DatabaseConnection,
    metadata: AudioMetadata,
) -> anyhow::Result<()> {
    let db_song = entity::songs::Entity::find()
        .filter(entity::songs::Column::Path.eq(metadata.path.to_owned()))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    if db_song.is_some() {
        let mut song: entity::songs::ActiveModel = db_song.unwrap().into();
        let update_time: String = Utc::now().naive_local().to_string();
        song.title = Set(metadata.name);
        song.album_name = Set(metadata.album);
        song.artist = Set(metadata.album_artist);
        song.disk = Set(Some(metadata.number as i32));
        song.track = Set(Some(metadata.track as i32));
        song.year = Set(Some(metadata.year));
        song.updated_at = Set(update_time);
        song.update(db).await.map_err(|e| anyhow::anyhow!(e))?;
    } else {
        create_song(db, metadata).await?;
    }
    Ok(())
}
pub async fn create_song(db: &DatabaseConnection, metadata: AudioMetadata) -> anyhow::Result<()> {
    let id: Uuid = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();

    let album = super::album::find_by_name(db, metadata.album.to_owned())
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let mut song: entity::songs::ActiveModel = entity::songs::ActiveModel {
        id: Set(id.to_string()),
        path: Set(metadata.path.to_owned()),
        title: Set(metadata.name),
        disk: Set(Some(metadata.number as i32)),
        artist: Set(metadata.album_artist.to_owned()),
        album_name: Set(metadata.album.to_owned()),
        codec: NotSet,
        sample_rate: NotSet,
        bits_per_sample: NotSet,
        track: Set(Some(metadata.track as i32)),
        year: Set(Some(metadata.year)),
        label: NotSet,
        music_brainz_recording_id: NotSet,
        music_brainz_artist_id: NotSet,
        music_brainz_track_id: NotSet,
        created_at: Set(init_time.to_owned()),
        updated_at: Set(init_time),
        album_id: NotSet,
        duration: Set(metadata.duration),
    };

    if album.is_some() {
        song.set(
            entity::songs::Column::AlbumId,
            Set(album.unwrap().id).into_value().unwrap(),
        )
    } else {
        let album_id = super::album::create_album(
            db,
            metadata.album.to_owned(),
            metadata.album_artist.to_owned(),
            Path::new(&metadata.path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            Some(metadata.year),
        )
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
        song.set(
            entity::songs::Column::AlbumId,
            Set(album_id.to_string()).into_value().unwrap(),
        )
    }

    entity::songs::Entity::insert(song)
        .on_conflict(
            OnConflict::column(entity::songs::Column::Path)
                .update_column(entity::songs::Column::UpdatedAt)
                .update_column(entity::songs::Column::AlbumName)
                .update_column(entity::songs::Column::Artist)
                .update_column(entity::songs::Column::Disk)
                .update_column(entity::songs::Column::Label)
                .update_column(entity::songs::Column::MusicBrainzArtistId)
                .update_column(entity::songs::Column::MusicBrainzRecordingId)
                .update_column(entity::songs::Column::MusicBrainzTrackId)
                .update_column(entity::songs::Column::SampleRate)
                .update_column(entity::songs::Column::Title)
                .update_column(entity::songs::Column::Track)
                .update_column(entity::songs::Column::Year)
                .update_column(entity::songs::Column::Duration)
                .to_owned(),
        )
        .exec(db)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    //song.insert(db).await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}
