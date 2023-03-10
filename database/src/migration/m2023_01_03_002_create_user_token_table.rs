use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use crate::migration::m2023_01_03_001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserToken::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserToken::User).integer().not_null())
                    .col(ColumnDef::new(UserToken::Token).text().not_null())
                    .col(ColumnDef::new(UserToken::CreatedByIp).text().not_null())
                    .col(ColumnDef::new(UserToken::Expires).date_time().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name(&UserToken::TokenUserFK.into_iden().to_string())
                            .from_tbl(UserToken::Table)
                            .from_col(UserToken::User) // user_token.user -> user.id
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    // .index(
                    //     &mut Index::create()
                    //         .name(&UserToken::User.to_string())
                    //         .table(UserToken::Table)
                    //         .col(UserToken::User)
                    //         .to_owned(),
                    // )
                    // .index(
                    //     &mut Index::create()
                    //         .name(&UserToken::Token.to_string())
                    //         .table(UserToken::Table)
                    //         .col(UserToken::Token)
                    //         .to_owned(),
                    // )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("user_token_user_idx")
                    .table(UserToken::Table)
                    .col(UserToken::User)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("user_token_token_idx")
                    .table(UserToken::Table)
                    .col(UserToken::Token)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserToken::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserToken {
    Table,
    Id,
    User,
    Token,
    CreatedByIp,
    Expires,

    TokenUserFK,
}
