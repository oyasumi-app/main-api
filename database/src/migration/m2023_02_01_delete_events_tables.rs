use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use super::m2023_01_12_001_create_event_stream_table::EventStream;
use super::m2023_01_13_001_create_event_table::Event;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Event::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager.drop_table(
            Table::drop()
                .table(EventStream::Table)
                .if_exists()
                .to_owned(),
        ).await?;
        Ok(())
    }

}

