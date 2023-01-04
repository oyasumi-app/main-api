use crate::entity::state_change::Entity as StateChangeEntity;
use sea_orm::*;

pub struct Query;

pub enum LoginIdentifier<'a> {
    Username(&'a str),
    Email(&'a str),
}

impl<'a> From<&'a str> for LoginIdentifier<'a> {
    fn from(s: &'a str) -> Self {
        if s.contains('@') {
            LoginIdentifier::Email(s)
        } else {
            LoginIdentifier::Username(s)
        }
    }
}
    

impl Query {
    pub async fn get_user_by_login(
        db: &DbConn,
        login: LoginIdentifier<'_>,
        password: &str,
    ) -> Option<crate::entity::user::Model> {  // Option is returned to avoid user enumeration
        // First find the user entity
        let user = match login {
            LoginIdentifier::Username(username) => {
                crate::entity::user::Entity::find()
                    .filter(crate::entity::user::Column::Username.eq(username))
                    .one(db)
                    .await.ok()?
            }
            LoginIdentifier::Email(email) => {
                crate::entity::user::Entity::find()
                    .filter(crate::entity::user::Column::Email.eq(email))
                    .one(db)
                    .await.ok()?
            }
        };
        let user = user?;
        // Then check the password
        if !crate::security::password::check_hash(password, &user.password_hash) {
            return None;
        }
        Some(user)
    }
}