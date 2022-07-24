use anyhow::{Context, Result};
use metaflac::{block::VorbisComment, Tag};
#[derive(Debug, PartialEq, Clone)]
pub struct AudioMetadata {
    pub name: String,
    pub number: u32,
    pub album: String,
    pub album_artist: String,
    pub year: i32,
    pub track: u32,
    pub artists: Vec<String>,
    pub path: String,
    pub lossless: bool,
}

pub fn get_metadata(path: String) -> Result<AudioMetadata> {
    let tag = Tag::read_from_path(&path).unwrap_or_default();
    let vorbis = tag
        .vorbis_comments()
        .with_context(|| format!("Failed to read file {}", path))?;

    let metadata = AudioMetadata {
        name: vorbis
            .title()
            .map(|v| v[0].clone())
            .with_context(|| "Failed to read title")?,
        number: vorbis.track().unwrap_or_default(),
        album: vorbis
            .album()
            .map(|v| v[0].clone())
            .with_context(|| "Failed to read album")?,
        album_artist: match vorbis.album_artist().map(|v| v[0].clone()) {
            Some(e) => e,
            None => vorbis
                .artist()
                .map(|v| v[0].clone())
                .with_context(|| "Failed to read album_artist")?,
        },
        year: get_year(vorbis).with_context(|| "Failed to read year")?,
        track: vorbis.track().with_context(|| "Failed to read track")?,
        artists: vorbis
            .artist()
            .with_context(|| "Failed to read artist")?
            .to_owned(),
        path,
        lossless: true,
    };
    Ok(metadata)
}

fn get_year(vorbis: &VorbisComment) -> Result<i32> {
    let year = vorbis
        .comments
        .get("ORIGINALYEAR")
        .and_then(|d| d[0].parse::<i32>().ok())
        .unwrap();
    Ok(year)
}
