use sea_orm::*;

pub struct Query;

pub enum LoginIdentifier {
    Email(String),
}

impl Query {
    pub async fn get_user_by_login(
        db: &DbConn,
        login: LoginIdentifier,
        password: &str,
    ) -> Option<crate::entity::user::Model> {
        // Option is returned to avoid user enumeration
        // First find the user entity
        let user = match login {
            LoginIdentifier::Email(email) => crate::entity::user::Entity::find()
                .filter(crate::entity::user::Column::Email.eq(email))
                .one(db)
                .await
                .ok()?,
        };
        let user = user?;
        // Then check the password
        if !crate::security::password::check_hash(password, &user.password_hash) {
            return None;
        }
        Some(user)
    }
}
