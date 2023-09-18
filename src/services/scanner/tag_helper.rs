use std::path::PathBuf;

use anyhow::{Context, Result};
use metaflac::{block::VorbisComment, Tag};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AudioMetadata {
    pub name: String,
    pub artist: String,
    pub artist_sort: Option<String>,
    pub album_name: String,
    pub album_artist: String,
    pub album_sort: Option<String>,
    pub genre: Option<Vec<String>>,
    pub style: Option<Vec<String>>,
    pub discogs_albumid: Option<String>,
    pub discogs_artistid: Option<String>,
    pub discogs_labelid: Option<String>,
    pub lyricist: Option<String>,
    pub composer: Option<String>,
    pub composer_sort: Option<String>,
    pub work: Option<String>,
    pub mb_workid: Option<String>,
    pub arranger: Option<String>,
    pub grouping: Option<String>,
    pub year: i32,
    pub lyrics: Option<String>,
    pub comments: Option<String>,
    pub bpm: Option<i32>,
    pub compilation: Option<String>,
    pub mb_track_id: Option<String>,
    pub mb_album_id: Option<String>,
    pub mb_artist_id: Option<String>,
    pub mb_albumartist_id: Option<String>,
    pub mb_releasetrack_id: Option<String>,
    pub mb_releasegroup_id: Option<String>,
    pub trackdisambig: Option<String>,
    pub album_type: Option<String>,
    pub acoustid_fingerprint: Option<String>,
    pub acoustid_id: Option<String>,
    pub asin: Option<String>,
    pub isrc: Option<String>,
    pub catalog_num: Option<String>,
    pub script: Option<String>,
    pub country: Option<String>,
    pub albumstatus: Option<String>,
    pub media: Option<String>,
    pub album_disambig: Option<String>,
    pub release_group_disambig: Option<String>,
    pub encodedby: Option<String>,
    pub original_year: Option<String>,
    pub initial_key: Option<String>,
    pub bit_rate: Option<i64>,
    pub encoder_settings: Option<String>,
    pub channels: Option<u8>,
    pub bit_depth: Option<u8>,
    pub sample_rate: Option<u32>,
    pub track: u32,
    pub disc: u32,
    pub length: u32,
    pub label: Option<String>,
    pub path: String,
    pub parent_path: String,
}

pub struct StreamInfo {
    length: Option<u32>,
    _total_samples: Option<u64>,
    sample_rate: Option<u32>,
    bits_per_sample: Option<u8>,
    /*     bit_rate: Option<i64>,
     */ num_channels: Option<u8>,
}
// Retreives the metadata from a flac file. Returning generic AudioMetadata struct
pub fn get_metadata_flac(path: PathBuf) -> Result<AudioMetadata> {
    let tag = Tag::read_from_path(&path)?;
    let vorbis: &VorbisComment = tag
        .vorbis_comments()
        .with_context(|| format!("Failed to read tags for {}", path.to_str().unwrap()))?;
    //let meta = fs::metadata(&path)?;

    let mut file_stream_info = tag.get_blocks(metaflac::BlockType::StreamInfo);
    let stream_info = match file_stream_info.next() {
        Some(metaflac::Block::StreamInfo(s)) => StreamInfo {
            length: Some(s.total_samples as u32 / s.sample_rate),
            _total_samples: Some(s.total_samples),
            sample_rate: Some(s.sample_rate),
            bits_per_sample: Some(s.bits_per_sample),
            /*             bit_rate: Some((meta.len() * (s.total_samples as u64 / s.sample_rate as u64)) as i64),
             */            /*             $kbps = floor((filesize($file) * 8) / (1024 * $seconds));
             */            /* bit_rate: Some(s.bits_per_sample * s.num_channels), */
            num_channels: Some(s.num_channels),
        },
        _ => anyhow::bail!("Failed to read stream info"),
    };

    let metadata: AudioMetadata = AudioMetadata {
        name: vorbis
            .title()
            .map(|v| v[0].clone())
            .unwrap_or_else(|| "FAILED TO READ TITLE DEAFTONE".to_string()),
        artist: vorbis
            .artist()
            .map(|v| v[0].clone())
            .unwrap_or_else(|| "FAILED TO READ ARTIST DEAFTONE".to_string()),
        artist_sort: vorbis
            .get("ARTISTSORT")
            .and_then(|d| d[0].parse::<String>().ok()),
        album_name: vorbis
            .album()
            .map(|v| v[0].clone())
            .unwrap_or_else(|| "FAILED TO READ ALBUM DEAFTONE".to_string()),
        album_artist: vorbis
            .album_artist()
            .map(|v| v[0].clone())
            .unwrap_or_else(|| "FAILED TO READ ALBUM DEAFTONE".to_string()),
        album_sort: vorbis
            .get("ALBUMSORT")
            .and_then(|d| d[0].parse::<String>().ok()),
        genre: vorbis.genre().cloned(),
        style: vorbis.get("STYLE").cloned(),
        discogs_albumid: vorbis
            .get("DISCOGS_ALBUMID")
            .and_then(|d| d[0].parse::<String>().ok()),
        discogs_artistid: vorbis
            .get("DISCOGS_ARTISTID")
            .and_then(|d| d[0].parse::<String>().ok()),
        discogs_labelid: vorbis
            .get("DISCOGS_LABELID")
            .and_then(|d| d[0].parse::<String>().ok()),
        lyricist: vorbis
            .get("LYRICIST")
            .and_then(|d| d[0].parse::<String>().ok()),
        composer: vorbis
            .get("COMPOSER")
            .and_then(|d| d[0].parse::<String>().ok()),
        composer_sort: vorbis
            .get("COMPOSERSORT")
            .and_then(|d| d[0].parse::<String>().ok()),
        work: vorbis.get("WORK").and_then(|d| d[0].parse::<String>().ok()),
        mb_workid: vorbis
            .get("MUSICBRAINZ_WORKID")
            .and_then(|d| d[0].parse::<String>().ok()),
        // TODO work_disambig
        arranger: vorbis
            .get("ARRANGER")
            .and_then(|d| d[0].parse::<String>().ok()),
        grouping: vorbis
            .get("GROUPING")
            .and_then(|d| d[0].parse::<String>().ok()),
        year: get_year(vorbis).with_context(|| "Failed to read year")?,
        lyrics: vorbis
            .get("LYRICS")
            .and_then(|d| d[0].parse::<String>().ok()),
        comments: vorbis
            .get("COMMENTS")
            .and_then(|d| d[0].parse::<String>().ok()),
        bpm: vorbis.get("BPM").and_then(|d| d[0].parse::<i32>().ok()),
        compilation: vorbis
            .get("COMPILATION")
            .and_then(|d| d[0].parse::<String>().ok()),
        mb_track_id: vorbis
            .get("MUSICBRAINZ_RELEASETRACKID")
            .and_then(|d| d[0].parse::<String>().ok()),
        mb_album_id: vorbis
            .get("MUSICBRAINZ_ALBUMID")
            .and_then(|d| d[0].parse::<String>().ok()),
        mb_artist_id: vorbis
            .get("MUSICBRAINZ_ARTISTID")
            .and_then(|d| d[0].parse::<String>().ok()),
        mb_albumartist_id: vorbis
            .get("MUSICBRAINZ_ALBUMARTISTID")
            .and_then(|d| d[0].parse::<String>().ok()),
        mb_releasetrack_id: vorbis
            .get("MUSICBRAINZ_RELEASETRACKID")
            .and_then(|d| d[0].parse::<String>().ok()),
        mb_releasegroup_id: vorbis
            .get("MUSICBRAINZ_RELEASEGROUPID")
            .and_then(|d| d[0].parse::<String>().ok()),
        trackdisambig: vorbis
            .get("TRACKDISAMBIG")
            .and_then(|d| d[0].parse::<String>().ok()),
        album_type: vorbis
            .get("RELEASETYPE")
            .and_then(|d| d[0].parse::<String>().ok()), // TODO return array
        acoustid_fingerprint: vorbis
            .get("ACOUSTID_FINGERPRINT")
            .and_then(|d| d[0].parse::<String>().ok()),
        acoustid_id: vorbis
            .get("ACOUSTID_ID")
            .and_then(|d| d[0].parse::<String>().ok()),
        asin: vorbis.get("ASIN").and_then(|d| d[0].parse::<String>().ok()),
        isrc: vorbis.get("ISRC").and_then(|d| d[0].parse::<String>().ok()),
        catalog_num: vorbis
            .get("CATALOGNUMBER")
            .and_then(|d| d[0].parse::<String>().ok()),
        script: vorbis
            .get("SCRIPT")
            .and_then(|d| d[0].parse::<String>().ok()),
        country: vorbis
            .get("RELEASECOUNTRY")
            .and_then(|d| d[0].parse::<String>().ok()),
        albumstatus: vorbis
            .get("RELEASESTATUS")
            .and_then(|d| d[0].parse::<String>().ok()),
        media: vorbis
            .get("MEDIA")
            .and_then(|d| d[0].parse::<String>().ok()),
        album_disambig: vorbis
            .get("album_disambig")
            .and_then(|d| d[0].parse::<String>().ok()),
        release_group_disambig: vorbis
            .get("release_group_disambig")
            .and_then(|d| d[0].parse::<String>().ok()),
        // TODO disctitle
        encodedby: vorbis
            .get("ENCODEDBY")
            .and_then(|d| d[0].parse::<String>().ok()),
        original_year: vorbis
            .get("ORIGINALYEAR")
            .and_then(|d| d[0].parse::<String>().ok()),
        initial_key: vorbis.get("KEY").and_then(|d| d[0].parse::<String>().ok()),
        bit_rate: Some(0),
        sample_rate: stream_info.sample_rate,
        /*             bitrate_mode: vorbis
                    .get("CATALOGNUMBER")
                    .and_then(|d| d[0].parse::<String>().ok()), */
        /*             encoder_info: vorbis
                    .get("CATALOGNUMBER")
                    .and_then(|d| d[0].parse::<String>().ok()), */
        encoder_settings: vorbis
            .get("ENCODERSETTINGS")
            .and_then(|d| d[0].parse::<String>().ok()),
        // TODO format
        // TODO bitdepth
        // TODO channels
        channels: stream_info.num_channels,
        bit_depth: stream_info.bits_per_sample,
        track: vorbis.track().unwrap_or(0),
        disc: vorbis
            .get("DISCNUMBER")
            .and_then(|d| d[0].parse::<u32>().ok())
            .unwrap_or_default(),
        // TODO codec
        length: stream_info.length.unwrap_or_default(),
        label: vorbis
            .get("LABEL")
            .and_then(|d| d[0].parse::<String>().ok()),
        // TODO sample_rate
        // TODO bits_per_sample
        // TODO albumtypes
        path: path.to_string_lossy().to_string(),
        parent_path: path.parent().unwrap().to_string_lossy().to_string(),
    };
    Ok(metadata)
}
// This is ugly. But why is there 3 different tags for date?
// Returns year tag from VorbisComment block
// YEAR -> DATE -> ORIGINALYEAR
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

// Parses string year into i32 year
fn parse_year(mut year: String) -> Result<i32> {
    if year.chars().count() == 10 {
        year.truncate(4);
        Ok(year.parse::<i32>().unwrap_or_default())
    } else {
        Ok(year.parse::<i32>().unwrap_or_default())
    }
}
