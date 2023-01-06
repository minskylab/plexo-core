use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{member::Member, task::Task};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Project {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    title: String,
    description: Option<String>,

    owner_id: Uuid,

    labels: Vec<String>,
}

#[ComplexObject]
impl Project {
    pub async fn owner(&self) -> Member {
        todo!()
    }

    pub async fn members(&self) -> Vec<Member> {
        todo!()
    }

    pub async fn tasks(&self) -> Vec<Task> {
        todo!()
    }
}
