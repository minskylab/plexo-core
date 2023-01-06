use async_graphql::{ComplexObject, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::task::Task;

#[derive(SimpleObject)]
// #[graphql(complex)]
pub struct Member {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    name: String,
    email: String,

    auth_id: String,

    role: MemberRole,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum MemberRole {
    Admin,
    Member,
}

impl Member {
    pub fn tasks(&self) -> Vec<Task> {
        todo!()
    }
}
