use api_types::v1::UnparsedEvent;
use sea_orm::{entity::prelude::*, QueryOrder, QuerySelect, Set};
use serde::{Deserialize, Serialize};

use crate::Snowflake;

use super::event_stream;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Snowflake,
    pub stream: Snowflake,
    pub when: DateTimeUtc,
    pub data: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::event_stream::Entity",
        from = "Column::Stream",
        to = "crate::entity::event_stream::Column::Id"
    )]
    EventStream,
}

impl Related<crate::entity::event_stream::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventStream.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn find_model_by_id(db: &DbConn, id: Snowflake) -> Result<Option<Model>, DbErr> {
    let event_stream = Entity::find_by_id(id).one(db).await?;
    match event_stream {
        Some(event) => Ok(Some(event)),
        None => Ok(None),
    }
}

pub async fn create(db: &DbConn, stream: Snowflake, data: String) -> Result<Snowflake, DbErr> {
    let id = Snowflake::new().await;
    let now = chrono::Utc::now();

    Entity::insert(ActiveModel {
        id: Set(id),
        stream: Set(stream),
        when: Set(now),
        data: Set(data),
    })
    .exec(db)
    .await?;
    Ok(id)
}

pub async fn find_with_stream(
    db: &DbConn,
    stream_id: Snowflake,
    id: Snowflake,
) -> Result<Option<(Model, event_stream::Model)>, DbErr> {
    let filter = Entity::find()
        .filter(Column::Id.eq(id))
        .find_also_related(event_stream::Entity)
        .filter(event_stream::Column::Id.eq(stream_id))
        .one(db)
        .await?;
    match filter {
        Some((event, Some(stream))) => Ok(Some((event, stream))),
        // the second element should always be Some because it comes from an FK relation
        Some((_, None)) => Err(DbErr::Custom(
            "Missing EventStream for valid Event?!".to_string(),
        )),
        None => Ok(None),
    }
}

pub async fn list_by_stream(
    db: &DbConn,
    stream_id: Snowflake,
    limit: u64,
    since: Snowflake,
) -> Result<Vec<Model>, DbErr> {
    let filter = Entity::find()
        .order_by_asc(Column::When)
        .filter(Column::Stream.eq(stream_id))
        .filter(Column::Id.gte(since))
        .limit(limit)
        .all(db)
        .await?;
    Ok(filter)
}

pub async fn count_by_stream(db: &DbConn, stream_id: Snowflake) -> Result<u64, DbErr> {
    let filter = Entity::find()
        .filter(Column::Stream.eq(stream_id))
        .count(db)
        .await?;
    Ok(filter)
}

impl From<Model> for UnparsedEvent {
    fn from(event: Model) -> Self {
        UnparsedEvent {
            id: event.id,
            stream_id: event.stream,
            when: event.when,
            data: event.data,
        }
    }
}
