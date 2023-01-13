use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use super::m2023_01_02_001_create_mode_switch_table::StateChange;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StateChange::Table).cascade().to_owned())
            .await
    }
}
