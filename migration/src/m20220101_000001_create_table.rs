use sea_orm_migration::prelude::*;

use crate::create_tables;

#[derive(DeriveMigrationName)]
pub struct Migration;

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
