use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Registration::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Registration::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Registration::Username).text().not_null())
                    .col(ColumnDef::new(Registration::Email).text().not_null())
                    .col(ColumnDef::new(Registration::PasswordHash).text().not_null())
                    .col(ColumnDef::new(Registration::CreatedByIp).text().not_null())
                    .col(ColumnDef::new(Registration::Expires).date_time().not_null())
                    .col(
                        ColumnDef::new(Registration::EmailResendAfter)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Registration::ConfirmToken).text().not_null())
                    .index(
                        &mut Index::create()
                            .name("username")
                            .table(Registration::Table)
                            .col(Registration::Username)
                            .unique()
                            .to_owned(),
                    )
                    .index(
                        &mut Index::create()
                            .name("email")
                            .table(Registration::Table)
                            .col(Registration::Email)
                            .unique()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Registration::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Registration {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    CreatedByIp,
    Expires,
    EmailResendAfter,
    ConfirmToken,
}
