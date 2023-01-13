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

impl Related<crate::entity::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
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
        id: Set(Snowflake::new().await),
        user: Set(user.id),
        token: Set(token.clone()),
        created_by_ip: Set(ip.to_string()),
        expires: Set(expires),
    };
    user_token_new.insert(db).await?;
    Ok(token)
}

pub async fn find_token(
    db: &DbConn,
    token: &str,
) -> Result<Option<(crate::entity::user_token::Model, crate::entity::user::Model)>, DbErr> {
    let now = chrono::Utc::now();
    let filter = Entity::find()
        .filter(Column::Token.eq(token).and(Column::Expires.gt(now)))
        .find_also_related(crate::entity::user::Entity)
        .one(db)
        .await?;
    match filter {
        None => Ok(None),
        Some((user_token, maybe_user)) => {
            let user = maybe_user
                .unwrap_or_else(|| panic!("UserToken ID={} exists without User", user_token.id));
            Ok(Some((user_token, user)))
        }
    }
}

pub async fn find_token_by_id(
    db: &DbConn,
    id: Snowflake,
) -> Result<Option<(crate::entity::user_token::Model, crate::entity::user::Model)>, DbErr> {
    let now = chrono::Utc::now();
    let filter = Entity::find()
        .filter(Column::Id.eq(id).and(Column::Expires.gt(now)))
        .find_also_related(crate::entity::user::Entity)
        .one(db)
        .await?;
    match filter {
        None => Ok(None),
        Some((user_token, maybe_user)) => {
            let user = maybe_user
                .unwrap_or_else(|| panic!("UserToken ID={} exists without User", user_token.id));
            Ok(Some((user_token, user)))
        }
    }
}

pub async fn delete_token(db: &DbConn, id: Snowflake) -> Result<(), DbErr> {
    Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn find_tokens_by_user(
    db: &DbConn,
    user_id: Snowflake,
) -> Result<Vec<crate::entity::user_token::Model>, DbErr> {
    let now = chrono::Utc::now();
    let filter = Entity::find()
        .filter(Column::User.eq(user_id).and(Column::Expires.gt(now)))
        .all(db)
        .await?;
    Ok(filter)
}

/// Delete all tokens associated with the given user, except the one with the given ID.
pub async fn delete_tokens_by_user_except(
    db: &DbConn,
    user_id: Snowflake,
    except_id: Snowflake,
) -> Result<(), DbErr> {
    let now = chrono::Utc::now();
    Entity::delete_many()
        .filter(
            Column::User
                .eq(user_id)
                .and(Column::Expires.gt(now))
                .and(Column::Id.ne(except_id)),
        )
        .exec(db)
        .await?;
    Ok(())
}
