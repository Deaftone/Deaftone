use sea_orm::{EntityTrait, QuerySelect, Select};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct DbArtist {
    pub id: String,
    pub name: String,
    pub image: String,
    pub bio: String,
    pub albums: Vec<entity::album::Model>,
}
