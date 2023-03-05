use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Organization {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
}

#[ComplexObject]
impl Organization {}
