use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use super::m2023_01_02_001_create_mode_switch_table::StateChange;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::Username)
                            .char()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Email).char().unique_key().not_null())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(StateChange::Table)
                    .add_column(
                        ColumnDef::new(User::Table) // Column named `user`
                            .integer()
                            .not_null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name(&User::StateChangeUserFKey.into_iden().to_string())
                            .from_tbl(StateChange::Table)
                            .from_col(User::Table) // state_change.user -> user.id
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign key from state_change.user to user.id
        manager
            .alter_table(
                Table::alter()
                    .table(StateChange::Table)
                    .drop_foreign_key(User::StateChangeUserFKey)
                    .to_owned(),
            )
            .await?;
        // Drop column state_change.user
        manager
            .alter_table(
                Table::alter()
                    .table(StateChange::Table)
                    .drop_column(
                        User::Table, // Column named `user`
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,

    StateChangeUserFKey,
}
