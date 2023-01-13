use sea_orm::{EntityTrait, QuerySelect, Select};
use serde::Serialize;

pub mod album;
pub mod artist;
pub mod playlist;
pub mod song;

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

#[derive(Serialize)]
pub struct DbArtist {
    id: String,
    name: String,
    image: String,
    bio: String,
    albums: Vec<entity::album::Model>,
}
