use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "state_change")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub when: DateTimeUtc,
    pub new_state: bool,
    pub user: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::user::Entity",
        from = "Column::User",
        to = "crate::entity::user::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
