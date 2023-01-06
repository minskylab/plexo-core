use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{project::Project, task::Task};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Member {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub email: String,

    pub auth_id: String,

    pub role: MemberRole,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum MemberRole {
    Admin,
    Member,
}

#[ComplexObject]
impl Member {
    pub async fn tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        todo!()
    }

    pub async fn assigned_tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        todo!()
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        todo!()
    }
}
