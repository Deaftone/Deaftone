pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ConnectionTrait, EntityTrait, Schema};

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}

async fn create_table<E>(db: &SchemaManagerConnection<'_>, schema: &Schema, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let stmt = builder.build(&schema.create_table_from_entity(entity));

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {e}"),
    }
}

pub async fn create_tables(db: &SchemaManagerConnection<'_>) {
    let schema = Schema::new(db.get_database_backend());
    create_table(db, &schema, entity::setting::Entity).await;
    create_table(db, &schema, entity::song::Entity).await;
    create_table(db, &schema, entity::album::Entity).await;
    create_table(db, &schema, entity::artist::Entity).await;
    create_table(db, &schema, entity::artist_metadata::Entity).await;
    create_table(db, &schema, entity::directory::Entity).await;
    create_table(db, &schema, entity::playlist::Entity).await;
    create_table(db, &schema, entity::playlist_song::Entity).await;
}
