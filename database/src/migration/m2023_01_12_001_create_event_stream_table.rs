use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use super::m2023_01_03_001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventStream::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EventStream::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EventStream::Owner).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name(&EventStream::EventStreamOwnerFK.into_iden().to_string())
                            .from_tbl(EventStream::Table)
                            .from_col(EventStream::Owner) // event_category.owner_id -> user.id
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .col(ColumnDef::new(EventStream::Name).text().not_null())
                    .col(ColumnDef::new(EventStream::StreamType).char().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(EventStream::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum EventStream {
    Table,
    Id,
    Owner,
    EventStreamOwnerFK,

    Name,
    StreamType,
}
