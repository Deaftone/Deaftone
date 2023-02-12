use sqlx::{Pool, Sqlite};

pub async fn scrap_metadata(_sqlite_pool: &Pool<Sqlite>) {}
