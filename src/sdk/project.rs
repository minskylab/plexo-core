use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{member::Member, task::Task};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,

    pub owner_id: Uuid,

    pub labels: Vec<String>,
}

#[ComplexObject]
impl Project {
    pub async fn owner(&self, ctx: &Context<'_>) -> Member {
        todo!()
    }

    pub async fn members(&self, ctx: &Context<'_>) -> Vec<Member> {
        todo!()
    }

    pub async fn tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        todo!()
    }
}