pub use sea_orm_migration::prelude::*;

mod m2023_01_02_001_create_mode_switch_table;
mod m2023_01_03_001_create_user_table;
mod m2023_01_03_002_create_user_token_table;
mod m2023_01_04_001_create_registration_table;
mod m2023_01_10_001_delete_mode_switch_table;
mod m2023_01_12_001_create_event_stream_table;
mod m2023_01_13_001_create_event_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m2023_01_02_001_create_mode_switch_table::Migration),
            Box::new(m2023_01_03_001_create_user_table::Migration),
            Box::new(m2023_01_03_002_create_user_token_table::Migration),
            Box::new(m2023_01_04_001_create_registration_table::Migration),
            Box::new(m2023_01_10_001_delete_mode_switch_table::Migration),
            Box::new(m2023_01_12_001_create_event_stream_table::Migration),
            Box::new(m2023_01_13_001_create_event_table::Migration),
        ]
    }
}
