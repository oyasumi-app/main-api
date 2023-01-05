use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Snowflake,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::entity::user_token::Entity")]
    UserToken,
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn find_by_username(db: &DbConn, username: &str) -> Result<Option<Model>, DbErr> {
    let filter = Entity::find()
        .filter(Column::Username.eq(username))
        .one(db)
        .await?;
    Ok(filter)
}

pub async fn find_by_email(db: &DbConn, email: &str) -> Result<Option<Model>, DbErr> {
    let filter = Entity::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(filter)
}
