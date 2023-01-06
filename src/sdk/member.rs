use sqlx::types::{time::PrimitiveDateTime, Uuid};

use super::task::Task;

pub struct Member {
    id: Uuid,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,

    name: String,
    email: String,

    auth_id: String,

    role: MemberRole,
}

pub enum MemberRole {
    Admin,
    Member,
}

impl Member {
    pub fn tasks(&self) -> Vec<Task> {
        todo!()
    }
}
