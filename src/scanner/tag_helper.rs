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
            .unwrap_or("FAILED TO READ TITLE DEAFTONE".to_string()),
        number: vorbis.track().unwrap_or_default(),
        album: vorbis
            .album()
            .map(|v| v[0].clone())
            .unwrap_or("FAILED TO READ ALBUM DEAFTONE".to_string()),
        album_artist: match vorbis.album_artist().map(|v| v[0].clone()) {
            Some(e) => e,
            None => vorbis
                .artist()
                .map(|v| v[0].clone())
                .unwrap_or("FAILED TO READ ARTIST DEAFTONE".to_string()),
        },
        year: get_year(vorbis).with_context(|| "Failed to read year")?,
        track: vorbis.track().unwrap_or(0),
        path,
        lossless: true,
    };
    Ok(metadata)
}
// This is ugly. But why is there 3 different tags for date?
fn get_year(vorbis: &VorbisComment) -> Result<i32> {
    let original_year = vorbis
        .comments
        .get("ORIGINALYEAR")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();

    let date = vorbis
        .comments
        .get("DATE")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();
    let year = vorbis
        .comments
        .get("YEAR")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();

    if year.chars().count() >= 4 {
        Ok(parse_year(year).with_context(|| "Failed to parse YEAR")?)
    } else if date.chars().count() >= 4 {
        Ok(parse_year(date).with_context(|| "Failed to parse DATE")?)
    } else if original_year.chars().count() >= 4 {
        Ok(parse_year(original_year).with_context(|| "Failed to parse ORIGINALDATE")?)
    } else {
        Ok(0)
    }
}

fn parse_year(mut year: String) -> Result<i32> {
    if year.chars().count() == 10 {
        year.truncate(4);
        Ok(year.parse::<i32>().unwrap_or_default())
    } else {
        Ok(year.parse::<i32>().unwrap_or_default())
    }
}