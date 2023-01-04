use sea_orm_migration::prelude::*;
use sea_orm::DbErr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StateChange::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StateChange::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StateChange::When).date_time().not_null())
                    .col(ColumnDef::new(StateChange::NewState).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StateChange::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum StateChange {
    Table,
    Id,
    When,
    NewState,
}