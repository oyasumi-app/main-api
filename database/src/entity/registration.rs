use std::{net::IpAddr, time::SystemTime};

use chrono::Duration;
use sea_orm::{entity::prelude::*, IntoActiveModel, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::Snowflake;

use super::{user, user_token};

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

    // Check if there is a user already registered for this username, and for this address.
    // If there is a user for this email address, send a message to that email address.
    // If there is a user for this username,

    let email_str = email.to_string();
    let user = user::find_by_email(&db, &email_str).await?;
    if user.is_none() {
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
    } else {
        // This is where we need to send the email that does not contain the token.
        let message = mail::templates::registration::make_duplicate_registration_email(email);
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
}

pub async fn resend_confirmation_email_by_model(
    db: &DbConn,
    registration: &Model,
) -> Result<Result<(), lettre::transport::smtp::Error>, DbErr> {
    let email: lettre::Address = registration
        .email
        .parse()
        .expect("Failed to parse email from database?!");
    let message = mail::templates::registration::make_registration_confirm_email(
        email.clone(),
        &registration.confirm_token,
    );
    let status = mail::delivery::send_message(message).await;

    let now: DateTimeUtc = SystemTime::now().into();

    match status {
        Ok(_) => {
            // Update the repeat timer
            let mut active_model = registration.clone().into_active_model();
            active_model.email_resend_after = Set(now + email_resend_after());
            active_model.save(db).await?;
            Ok(Ok(()))
        }
        Err(error) => {
            tracing::error!(
                "Failed to send repeat confirmation for email {:?} for registration {}: {:?}",
                email,
                registration.id,
                error
            );

            // Update the repeat timer anyway (TODO: figure out whether the error is on our side, and do not resend if so)
            let mut active_model = registration.clone().into_active_model();
            active_model.email_resend_after = Set(now + email_resend_after());
            active_model.save(db).await?;

            Ok(Err(error))
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

pub async fn upgrade_to_user(db: &DbConn, registration: &Model) -> Result<String, DbErr> {
    // Transactionally delete the registration and create the user. Return the instance of the new token created for the user.
    let tx = db.begin().await?;
    // Delete registration
    let registration_to_delete = registration.clone();
    registration_to_delete.delete(&tx).await?;
    // Create user
    let user_new = crate::entity::user::ActiveModel {
        id: Set(registration.id),
        username: Set(registration.username.clone()),
        email: Set(registration.email.clone()),
        password_hash: Set(registration.password_hash.clone()),
    };
    let created_user = user_new.insert(&tx).await?;

    // Make token
    let new_token = user_token::make_token(
        &tx,
        &created_user,
        &registration
            .created_by_ip
            .parse()
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))), // This is not expected to fail because it comes from the database
    )
    .await?;

    tx.commit().await?;
    Ok(new_token)
}
