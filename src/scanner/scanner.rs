use chrono::Utc;
use entity;
use metaflac::Tag;
use migration::OnConflict;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait, Set,
};
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Clone)]
pub struct AudioMetadata {
    pub name: String,
    pub number: u32,
    pub album: String,
    pub album_artist: String,
    pub year: i32,
    pub track: u32,
    pub artists: Vec<String>,
    pub path: std::path::PathBuf,
    pub lossless: bool,
}

pub async fn walk(db: &DatabaseConnection) -> anyhow::Result<()> {
    let current_dir = "G:\\aa";
    for entry in WalkDir::new(current_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".flac") {
            println!("{}", f_name);
            let tag = Tag::read_from_path(entry.path()).unwrap();
            let vorbis = tag.vorbis_comments().ok_or(0).unwrap();
            let year = vorbis
                .comments
                .get("YEAR")
                .and_then(|d| d[0].parse::<i32>().ok());

            let metadata = AudioMetadata {
                name: vorbis.title().map(|v| v[0].clone()).unwrap(),
                number: vorbis.track().unwrap(),
                album: vorbis.album().map(|v| v[0].clone()).unwrap(),
                album_artist: match vorbis.album_artist().map(|v| v[0].clone()) {
                    Some(e) => e,
                    None => vorbis.artist().map(|v| v[0].clone()).unwrap(),
                },
                year: year.unwrap_or(0),
                track: vorbis.track().unwrap(),
                artists: vorbis.artist().unwrap().to_owned(),
                path: entry.path().to_owned(),
                lossless: true,
            };
            let path = entry.path().to_string_lossy().to_string();
            let id = Uuid::new_v4();
            let song = entity::songs::ActiveModel {
                id: Set(id.to_string()),
                path: Set(path),
                title: Set(metadata.name),
                disk: NotSet,
                artist: Set(metadata.album_artist),
                album_name: Set(metadata.album),
                codec: NotSet,
                sample_rate: NotSet,
                bits_per_sample: NotSet,
                track: NotSet,
                year: Set(year),
                label: NotSet,
                music_brainz_recording_id: NotSet,
                music_brainz_artist_id: NotSet,
                music_brainz_track_id: NotSet,
                created_at: Set(Utc::now().naive_local().to_string()),
                updated_at: Set(Utc::now().naive_local().to_string()),
                album_id: NotSet,
            };
            /*      .insert(db)
            .await
            .expect("Failed to insert"); */

            entity::songs::Entity::insert(song)
                .on_conflict(
                    // on conflict do nothing
                    OnConflict::column(entity::songs::Column::Path)
                        .do_nothing()
                        .to_owned(),
                )
                .exec(db)
                .await
                .expect("Failed to insert song");
        }
    }

    Ok(())
}
