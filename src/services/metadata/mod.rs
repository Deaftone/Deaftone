use anyhow::{Context, Result};
use chrono::Utc;
use futures::TryStreamExt;
use musicbrainz_rs::{
    entity::{artist::Artist, relations::RelationContent},
    Fetch,
};

use sqlx::{Pool, Row, Sqlite};
use uuid::Uuid;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArtistMetadata {
    bio: Option<String>,
    image: Option<String>,
    twitter: Option<String>,
    instagram: Option<String>,
    all_music: Option<String>,
    amazon_music: Option<String>,
    apple_music: Option<String>,
    beatport: Option<String>,
    facebook: Option<String>,
    discogs: Option<String>,
    imdb: Option<String>,
    deezer: Option<String>,
    spotify: Option<String>,
    tidal: Option<String>,
    tiktok: Option<String>,
    wiki: Option<String>,
    youtube: Option<String>,
}

impl ArtistMetadata {
    pub fn new() -> Self {
        Self {
            bio: None,
            image: None,
            twitter: None,
            instagram: None,
            all_music: None,
            amazon_music: None,
            apple_music: None,
            beatport: None,
            facebook: None,
            discogs: None,
            imdb: None,
            deezer: None,
            spotify: None,
            tidal: None,
            tiktok: None,
            wiki: None,
            youtube: None,
        }
    }

    pub fn get_links(
        &mut self,
        relations: &std::option::Option<Vec<musicbrainz_rs::entity::relations::Relation>>,
    ) -> &mut Self {
        for val in relations.as_ref().unwrap().iter() {
            match val.relation_type.as_str() {
                "youtube" => {
                    if let RelationContent::Url(a) = &val.content {
                        self.youtube = Some(a.resource.clone())
                    }
                }
                "allmusic" => {
                    if let RelationContent::Url(a) = &val.content {
                        if self.all_music.is_none() {
                            self.all_music = Some(a.resource.clone())
                        }
                    }
                }
                "wikidata" => {
                    if let RelationContent::Url(a) = &val.content {
                        self.wiki = Some(a.resource.clone())
                    }
                }
                "discogs" => {
                    if let RelationContent::Url(a) = &val.content {
                        self.discogs = Some(a.resource.clone())
                    }
                }
                _ => (),
            }
        }
        self
    }
    fn trim_whitespace(s: &str) -> String {
        let mut new_str = s.trim().to_owned();
        let mut prev = ' '; // The initial value doesn't really matter
        new_str.retain(|ch| {
            let result = ch != ' ' || prev != ' ';
            prev = ch;
            result
        });
        new_str
    }
    // FIXME whitespace
    pub async fn get_allmusic_bio(&mut self) -> Result<&mut Self> {
        tracing::debug!(
            "Requesting allmusic page {:}",
            &self.all_music.as_ref().unwrap()
        );
        let response = reqwest::get(format!(
            "{}{}",
            self.all_music.as_ref().unwrap(),
            String::from("/biography")
        ))
        .await
        .unwrap()
        .text()
        .await
        .with_context(|| "Failed to request bio page")?;

        let document = scraper::Html::parse_document(&response);
        let bio_select = scraper::Selector::parse("div.text").unwrap();
        let bio = document
            .select(&bio_select)
            .next()
            .with_context(|| "Failed to select bio")?;
        let formated_bio = ArtistMetadata::trim_whitespace(
            bio.text()
                .collect::<String>()
                .trim()
                .replace('\n', "")
                .as_str(),
        );
        self.bio = Some(formated_bio);
        Ok(self)
    }
}
impl Default for ArtistMetadata {
    fn default() -> Self {
        ArtistMetadata::new()
    }
}
pub async fn scrap_metadata(sqlite_pool: &Pool<Sqlite>) {
    // Get all artists that have a mb_artist_id and DONT have a entry in the artist_metadata table
    let mut rows =
        sqlx::query("SELECT * FROM \"artists\" WHERE \"artists\".\"mb_artist_id\" NOT NULL GROUP BY \"artists\".\"mb_artist_id\"")
            .fetch(sqlite_pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        // map the row into a user-defined domain type
        let mb_artist_id: &str = row.try_get("mb_artist_id").unwrap();
        let mb_artist_relations = Artist::fetch()
            .id(mb_artist_id)
            .with_url_relations()
            .execute()
            .await
            .unwrap()
            .relations;
        let artist_id: &str = row.try_get("id").unwrap();
        let artist_metadata = ArtistMetadata::default()
            .get_links(&mb_artist_relations)
            .get_allmusic_bio()
            .await
            .cloned();

        match artist_metadata {
            Ok(metadata) => {
                let id: String = Uuid::new_v4().to_string();
                let init_time: String = Utc::now().naive_local().to_string();
                let metadata_exists =
                    sqlx::query("SELECT * FROM artists_metadata WHERE artist_id = ?")
                        .bind(artist_id)
                        .persistent(true)
                        .fetch_one(sqlite_pool)
                        .await;

                match metadata_exists {
                    Err(sqlx::Error::RowNotFound) => {
                        tracing::debug!(
                            "Creating artists_metadata entry {:} for artist {:}",
                            &id,
                            artist_id
                        );
                        sqlx::query(
                            "INSERT OR REPLACE INTO artists_metadata (
                                        id, 
                                        artist_id,
                                        mb_artist_id,
                                        bio,
                                        created_at,
                                        updated_at
                                     )
                                VALUES (?,?,?,?,?,?)",
                        )
                        .bind(&id)
                        .bind(artist_id)
                        .bind(mb_artist_id)
                        .bind(&metadata.bio)
                        .bind(&init_time)
                        .bind(&init_time)
                        .execute(sqlite_pool)
                        .await
                        .unwrap();
                    }
                    Ok(value) => {
                        let am_id: &str = value.get("id");
                        tracing::debug!(
                            "Updating artists_metadata entry {:} for artist {:}",
                            &am_id,
                            artist_id
                        );
                        sqlx::query(
                            "INSERT OR REPLACE INTO artists_metadata (
                                        id, 
                                        artist_id,
                                        mb_artist_id,
                                        bio,
                                        created_at,
                                        updated_at
                                     )
                                VALUES (?,?,?,?,?,?)",
                        )
                        .bind(am_id)
                        .bind(artist_id)
                        .bind(mb_artist_id)
                        .bind(&metadata.bio)
                        .bind(&init_time)
                        .bind(&init_time)
                        .execute(sqlite_pool)
                        .await
                        .unwrap();
                    }
                    Err(err) => {
                        tracing::error!("Error in query {:}", err);
                    }
                }
            }
            Err(err) => {
                tracing::error!(
                    "Failed to scrap metadata for {:} and mb_artist {:}. Error: {:}",
                    artist_id,
                    mb_artist_id,
                    err
                )
            }
        }
    }
}
