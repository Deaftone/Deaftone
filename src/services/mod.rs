use sea_orm::{EntityTrait, QuerySelect, Select};

use crate::handlers::ArtistResponse;

pub mod album;
pub mod artist;
pub mod playlist;
pub mod song;
pub mod task;

trait DeaftoneSelect {
    fn limit_option(self, limit: Option<u64>) -> Self;
}

impl<E> DeaftoneSelect for Select<E>
where
    E: EntityTrait,
{
    fn limit_option(self, limit: Option<u64>) -> Self {
        match limit {
            Some(v) => self.limit(v),
            None => self,
        }
    }
}

pub type DbArtist = ArtistResponse;
