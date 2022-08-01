use entity;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

async fn create_table<E>(db: &DbConn, schema: &Schema, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let stmt = builder.build(&schema.create_table_from_entity(entity));

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn create_tables(db: &DbConn) {
    let schema = Schema::new(db.get_database_backend());
    create_table(db, &schema, entity::songs::Entity).await;
    create_table(db, &schema, entity::albums::Entity).await;
    create_table(db, &schema, entity::artists::Entity).await;
    create_table(db, &schema, entity::directories::Entity).await;
}
#[async_trait::async_trait]
#[allow(unused_must_use)]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_tables(manager.get_connection()).await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::albums::Entity.to_string(),
                        entity::albums::Column::Id.to_string()
                    ))
                    .table(entity::albums::Entity)
                    .col(entity::albums::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::songs::Entity.to_string(),
                        entity::songs::Column::Id.to_string()
                    ))
                    .table(entity::songs::Entity)
                    .col(entity::songs::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::artists::Entity.to_string(),
                        entity::artists::Column::Id.to_string()
                    ))
                    .table(entity::artists::Entity)
                    .col(entity::artists::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::directories::Entity.to_string(),
                        entity::directories::Column::Id.to_string()
                    ))
                    .table(entity::directories::Entity)
                    .col(entity::directories::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::artists::Entity.to_string(),
                        entity::artists::Column::Name.to_string()
                    ))
                    .table(entity::artists::Entity)
                    .col(entity::artists::Column::Name)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::albums::Entity.to_string(),
                        entity::albums::Column::Name.to_string()
                    ))
                    .table(entity::albums::Entity)
                    .col(entity::albums::Column::Name)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::songs::Entity.to_string(),
                        entity::songs::Column::Path.to_string()
                    ))
                    .table(entity::songs::Entity)
                    .col(entity::songs::Column::Path)
                    .to_owned(),
            )
            .await;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
