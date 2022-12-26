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
    create_table(db, &schema, entity::setting::Entity).await;
    create_table(db, &schema, entity::song::Entity).await;
    create_table(db, &schema, entity::album::Entity).await;
    create_table(db, &schema, entity::artist::Entity).await;
    create_table(db, &schema, entity::directory::Entity).await;
    create_table(db, &schema, entity::playlist::Entity).await;
    create_table(db, &schema, entity::playlist_song::Entity).await;
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
                        entity::album::Entity.to_string(),
                        entity::album::Column::Id.to_string()
                    ))
                    .table(entity::album::Entity)
                    .col(entity::album::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::song::Entity.to_string(),
                        entity::song::Column::Id.to_string()
                    ))
                    .table(entity::song::Entity)
                    .col(entity::song::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::artist::Entity.to_string(),
                        entity::artist::Column::Id.to_string()
                    ))
                    .table(entity::artist::Entity)
                    .col(entity::artist::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::directory::Entity.to_string(),
                        entity::directory::Column::Id.to_string()
                    ))
                    .table(entity::directory::Entity)
                    .col(entity::directory::Column::Id)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::artist::Entity.to_string(),
                        entity::artist::Column::Name.to_string()
                    ))
                    .table(entity::artist::Entity)
                    .col(entity::artist::Column::Name)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::album::Entity.to_string(),
                        entity::album::Column::Name.to_string()
                    ))
                    .table(entity::album::Entity)
                    .col(entity::album::Column::Name)
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                Index::create()
                    .name(&format!(
                        "idx-{}-{}",
                        entity::song::Entity.to_string(),
                        entity::song::Column::Path.to_string()
                    ))
                    .table(entity::song::Entity)
                    .col(entity::song::Column::Path)
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
