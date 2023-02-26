use std::net::IpAddr;

use chrono::Duration;
use sea_orm::{entity::prelude::*, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "registration")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Snowflake,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_by_ip: String,
    pub expires: DateTimeUtc,
    pub confirm_token: String,
    pub email_resend_after: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn token_expiration() -> Duration {
    Duration::hours(8)
}

fn email_resend_after() -> Duration {
    Duration::minutes(5)
}

const TOKEN_LENGTH: u16 = 32;

pub async fn find_pending_registration(
    db: &DbConn,
    username: &str,
    email: &lettre::Address,
) -> Result<Option<Model>, DbErr> {
    let now = chrono::Utc::now();
    let filter = Entity::find()
        .filter(
            Column::Username
                .eq(username)
                .or(Column::Email.eq(email.to_string()))
                .and(Column::Expires.gt(now)),
        )
        .one(db)
        .await?;
    Ok(filter)
}

pub async fn make_registration(
    db: DbConn,
    username: &str,
    email: lettre::Address,
    password: &str,
    ip: &IpAddr,
) -> Result<Snowflake, DbErr> {
    let snowflake = Snowflake::new().await;
    let now = snowflake.timestamp();
    let token = crate::security::token::generate_token(TOKEN_LENGTH);
    let expires = now + token_expiration();
    let password_hash = crate::security::password::make_hash(password);
    let registration_new = ActiveModel {
        id: Set(snowflake),
        username: Set(username.to_string()),
        email: Set(email.to_string()),
        password_hash: Set(password_hash),
        created_by_ip: Set(ip.to_string()),
        expires: Set(expires),
        confirm_token: Set(token.clone()),
        email_resend_after: Set(now + email_resend_after()),
    };
    registration_new.insert(&db).await?;

    let message = mail::templates::registration::make_registration_confirm_email(email, &token);
    let status = mail::delivery::send_message(message).await;
    match status {
        Ok(_) => Ok(snowflake),
        Err(error) => {
            tracing::error!("Error while sending message: {:?}", error);
            // TODO: figure out if error is temporary or permanent, and maybe error out if permanent
            Ok(snowflake)
        }
    }
}

pub async fn get_by_id(db: &DbConn, id: Snowflake) -> Result<Option<Model>, DbErr> {
    let now = chrono::Utc::now();
    let filter = Entity::find()
        .filter(Column::Id.eq(id).and(Column::Expires.gt(now)))
        .one(db)
        .await?;
    Ok(filter)
}

pub async fn upgrade_to_user(db: &DbConn, registration: &Model) -> Result<(), DbErr> {
    // Transactionally delete the registration and create the user
    let tx = db.begin().await?;
    let registration_to_delete = registration.clone();
    registration_to_delete.delete(&tx).await?;
    let user_new = crate::entity::user::ActiveModel {
        id: Set(registration.id),
        username: Set(registration.username.clone()),
        email: Set(registration.email.clone()),
        password_hash: Set(registration.password_hash.clone()),
    };
    user_new.insert(&tx).await?;
    tx.commit().await?;
    Ok(())
}
