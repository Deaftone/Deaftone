use anyhow::{Context, Result};
use metaflac::{block::VorbisComment, Tag};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AudioMetadata {
    pub name: String,
    pub number: u32,
    pub album: String,
    pub album_artist: String,
    pub year: i32,
    pub track: u32,
    pub path: String,
    pub lossless: bool,
    pub duration: u32,
}

pub fn get_metadata(path: String) -> Result<AudioMetadata> {
    let tag = Tag::read_from_path(&path)?;
    let vorbis: &VorbisComment = tag
        .vorbis_comments()
        .with_context(|| format!("Failed to read tags for {}", path))?;

    let mut stream_info = tag.get_blocks(metaflac::BlockType::StreamInfo);
    let duration = match stream_info.next() {
        Some(metaflac::Block::StreamInfo(s)) => {
            Some(s.total_samples as u32 / s.sample_rate)
        }
        _ => None,
    };
    let metadata: AudioMetadata = AudioMetadata {
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
        duration: duration.unwrap_or_default(),
    };
    Ok(metadata)
}
// This is ugly. But why is there 3 different tags for date?
fn get_year(vorbis: &VorbisComment) -> Result<i32> {
    let original_year: String = vorbis
        .comments
        .get("ORIGINALYEAR")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();

    let date: String = vorbis
        .comments
        .get("DATE")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();
    let year: String = vorbis
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
