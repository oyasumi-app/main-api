use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use super::m2023_01_12_001_create_event_stream_table::EventStream;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Event::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Event::Stream).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name(&Event::EventEventStreamFK.into_iden().to_string())
                            .from_tbl(Event::Table)
                            .from_col(Event::Stream) // event.stream -> event_stream.id
                            .to_tbl(EventStream::Table)
                            .to_col(EventStream::Id),
                    )
                    .col(ColumnDef::new(Event::When).date_time().not_null())
                    .col(ColumnDef::new(Event::Data).text().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Event {
    Table,
    Id,
    Stream,
    When,
    Data,

    EventEventStreamFK,
}
