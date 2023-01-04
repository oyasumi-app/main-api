use crate::entity::state_change;
use chrono::Utc;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn insert_state_change(
        db: &DbConn,
        new_state: bool,
    ) -> Result<state_change::ActiveModel, DbErr> {
        let state_change_new = state_change::ActiveModel {
            when: Set(Utc::now()),
            new_state: Set(new_state),
            ..Default::default()
        };
        state_change_new.save(db).await
    }
}
