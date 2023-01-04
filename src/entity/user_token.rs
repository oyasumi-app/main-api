use std::net::IpAddr;

use chrono::Duration;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Snowflake,
    pub user: Snowflake,
    pub token: String,
    pub created_by_ip: String,
    pub expires: DateTimeUtc,
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

const TOKEN_LENGTH: u16 = 32;

fn token_expiration() -> Duration {
    Duration::days(30)
}

pub async fn make_token(
    db: &DbConn,
    user: &crate::entity::user::Model,
    ip: &IpAddr,
) -> Result<String, DbErr> {
    let token = crate::security::token::generate_token(TOKEN_LENGTH);
    let now = chrono::Utc::now();
    let expires = now + token_expiration();
    let user_token_new = ActiveModel {
        user: Set(user.id),
        token: Set(token.clone()),
        created_by_ip: Set(ip.to_string()),
        expires: Set(expires),
        ..Default::default()
    };
    user_token_new.insert(db).await?;
    Ok(token)
}
