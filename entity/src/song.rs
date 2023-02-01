//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, ToSchema)]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub path: String,
    pub title: String,
    pub artist: String,
    pub artist_sort: Option<String>,
    pub artist_credit: Option<String>,
    pub album_name: String,
    pub album_artist: Option<String>,
    pub album_sort: Option<String>,
    pub album_artist_credit: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub discogs_albumid: Option<String>,
    pub discogs_artistid: Option<String>,
    pub discogs_labelid: Option<String>,
    pub lyricist: Option<String>,
    pub composer: Option<String>,
    pub composer_sort: Option<String>,
    pub work: Option<String>,
    pub mb_workid: Option<String>,
    pub work_disambig: Option<String>,
    pub arranger: Option<String>,
    pub grouping: Option<String>,
    pub year: Option<i32>,
    pub lyrics: Option<String>,
    pub comments: Option<String>,
    pub bpm: Option<i32>,
    pub comp: Option<i32>,
    pub mb_track_id: Option<String>,
    pub mb_album_id: Option<String>,
    pub mb_artist_id: Option<String>,
    pub mb_albumartist_id: Option<String>,
    pub mb_releasetrack_id: Option<String>,
    pub mb_releasegroup_id: Option<String>,
    pub track_disambig: Option<String>,
    pub album_type: Option<String>,
    pub album_types: Option<String>,
    pub acoustid_fingerprint: Option<String>,
    pub acoustid_id: Option<String>,
    pub asin: Option<String>,
    pub isrc: Option<String>,
    pub catalog_num: Option<String>,
    pub script: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub album_status: Option<String>,
    pub media: Option<String>,
    pub album_disambig: Option<String>,
    pub release_group_disambig: Option<String>,
    pub disc_title: Option<String>,
    pub encoder: Option<String>,
    pub original_year: Option<i32>,
    pub initial_key: Option<String>,
    pub bitrate: Option<i32>,
    pub bitrate_mode: Option<i32>,
    pub encoder_info: Option<String>,
    pub encoder_settings: Option<String>,
    pub format: Option<String>,
    pub bitdepth: Option<String>,
    pub channels: Option<String>,
    pub track: Option<i32>,
    pub disk: Option<i32>,
    pub codec: Option<String>,
    pub length: u32,
    pub label: Option<String>,
    pub sample_rate: Option<String>,
    pub bits_per_sample: Option<i32>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub album_id: Option<String>,
    pub liked: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::album::Entity",
        from = "Column::AlbumId",
        to = "super::album::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Album,
}

impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
