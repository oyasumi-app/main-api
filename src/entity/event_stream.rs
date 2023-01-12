use api_types::v1::{EventStreamCreateRequest, EventStreamPatchRequest};

use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event_stream")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Snowflake,
    pub owner: Snowflake,
    pub name: String,
    pub stream_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::user::Entity",
        from = "Column::Owner",
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

impl From<Model> for api_types::v1::EventStream {
    fn from(event_stream: Model) -> Self {
        Self {
            id: event_stream.id,
            owner: event_stream.owner,
            name: event_stream.name,
            stream_type: event_stream
                .stream_type
                .parse()
                .expect("Found unknown value in database stream_type"),
        }
    }
}
pub async fn find_model_by_id(
    db: &DbConn,
    id: Snowflake,
) -> Result<Option<Model>, DbErr> {
    let event_stream = Entity::find_by_id(id).one(db).await?;
    match event_stream {
        Some(event_stream) => Ok(Some(event_stream)),
        None => Ok(None),
    }
}

pub async fn find_by_id(
    db: &DbConn,
    id: Snowflake,
) -> Result<Option<api_types::v1::EventStream>, DbErr> {
    let event_stream = find_model_by_id(db, id).await?;
    Ok(event_stream.map(Into::into))
}

pub async fn find_by_owner(
    db: &DbConn,
    owner: Snowflake,
) -> Result<Vec<api_types::v1::EventStream>, DbErr> {
    let event_streams = Entity::find()
        .filter(Column::Owner.eq(owner))
        .all(db)
        .await?;
    Ok(event_streams
        .into_iter()
        .map(|event_stream| event_stream.into())
        .collect())
}

pub async fn patch_by_id(db: &DbConn, id: Snowflake, patch: EventStreamPatchRequest) -> Result<(), DbErr> {
    let event_stream = find_model_by_id(db, id).await?;
    if let Some(event_stream) = event_stream {
        let mut active_model: ActiveModel = event_stream.into();

        if let Some(name) = patch.name {
            active_model.name = Set(name);
        }
        if let Some(stream_type) = patch.stream_type {
            active_model.stream_type = Set(stream_type.to_string());
        }
        {
            active_model.update(db).await?;
        }

        Ok(())
    } else {
        Err(DbErr::Custom("Event stream not found".to_string()))
    }
}

//use ;

pub async fn create(
    db: &DbConn,
    owner: Snowflake,
    data: EventStreamCreateRequest,
) -> Result<Snowflake, DbErr> {
    let id = Snowflake::new().await;
    let event_stream_insert = Entity::insert(ActiveModel {
        id: Set(id),
        owner: Set(owner),
        name: Set(data.name),
        stream_type: Set(data.stream_type.to_string()),
    });
    event_stream_insert.exec(db).await?;
    Ok(id)
}
